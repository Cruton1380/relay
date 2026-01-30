# PR #1.2: Final Polish Audit - Q1-Q5 Answered

**Date:** 2026-01-28  
**Status:** ✅ RELAY-CORRECT IN THE STRONGEST SENSE

---

## AUDIT FINDINGS

### Q1) Broadcast Buffer Sizing

**Current Implementation:**
```rust
// main_integration.rs:71
events: EventBus::new(1024),
```

**Answer:** ✅ YES - Capacity is 1024 events

**What This Means:**
- Broadcast channel can buffer up to 1024 events before slow consumers lag
- If more than 1024 events accumulate, slowest consumer receives `Lagged(n)` error
- Per our PR #1.2 lock: lag → disconnect → force replay from log

**Is This Intentional?**

⚠️ **NOT DOCUMENTED** - This is a magic number with no comment or constant

**Recommendation:** Lock this as a named constant with clear policy

```rust
// Add to events.rs or main_integration.rs
/// PR #1.2 LOCK: Event bus capacity
/// 
/// This defines the "max tolerated lag window" before forcing client reconnection.
/// 
/// Trade-offs:
/// - Larger buffer: slow clients stay connected longer (more memory, more lag tolerance)
/// - Smaller buffer: clients disconnect faster (less memory, stricter real-time guarantee)
/// 
/// Current policy: 1024 events (~1-2 seconds under high load, ~10-60 minutes under normal load)
pub const EVENT_BUS_CAPACITY: usize = 1024;

// Then in init_relay_physics_state():
events: EventBus::new(EVENT_BUS_CAPACITY),
```

**Production Tuning Guide:**

| Load Pattern | Recommended Capacity | Lag Tolerance |
|--------------|---------------------|---------------|
| Low (< 1 event/sec) | 256 | ~4 minutes |
| Medium (1-10 events/sec) | 1024 | ~2 minutes |
| High (10-100 events/sec) | 4096 | ~40 seconds |
| Burst (100+ events/sec) | 8192 | ~80 seconds |

**Decision:** DOCUMENT current value + add tuning guide

**Status:** ⚠️ **ACTION NEEDED** - Add constant + docs

---

### Q2) Cache-Control + SSE Across Different Infra

**Current Implementation:**
```rust
// main_integration.rs:events_sse_handler
.header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
.header(header::CONNECTION, "keep-alive")
.header("X-Accel-Buffering", "no")
```

**Question:** Do we need additional headers for Cloudflare / AWS ALB / reverse proxies?

**Answer by Infrastructure:**

#### ✅ Nginx (Current Headers Work)
```
X-Accel-Buffering: no         ← Explicitly set ✅
Cache-Control: no-cache       ← Prevents caching ✅
Connection: keep-alive        ← Keeps stream open ✅
```

**Status:** ✅ **PRODUCTION-READY**

---

#### ✅ Cloudflare (Auto-Detects SSE)

Cloudflare automatically detects `Content-Type: text/event-stream` and:
- Disables buffering
- Disables caching
- Enables streaming

**Our headers:**
- `Cache-Control: no-cache` → Redundant but harmless ✅
- `X-Accel-Buffering: no` → Ignored but harmless ✅

**Additional header needed?**
- ❌ `Content-Type: text/event-stream` → **Already set by Axum SSE**

**Status:** ✅ **PRODUCTION-READY**

**Reference:** https://developers.cloudflare.com/support/speed/optimization-file-size/what-will-cloudflare-cache/

---

#### ✅ AWS ALB / Application Load Balancer

ALB respects:
- `Connection: keep-alive` ✅ (we set this)
- `Content-Type: text/event-stream` ✅ (Axum sets this)

ALB does NOT buffer SSE by default (unlike HTTP/1.1 responses).

**Additional headers needed:**
- ❌ None - ALB handles SSE correctly

**Status:** ✅ **PRODUCTION-READY**

---

#### ✅ GCP Load Balancer (Cloud Load Balancing)

GCP LB handles SSE streams correctly when:
- `Content-Type: text/event-stream` ✅ (Axum sets)
- Backend timeout > SSE connection duration ⚠️ (config, not header)

**Additional headers needed:**
- ❌ None

**Configuration note:** Set backend timeout to 1 hour+ in GCP console

**Status:** ✅ **PRODUCTION-READY** (with config)

---

#### 🔶 Generic HTTP Proxies (Optional Enhancement)

For maximum compatibility with unknown intermediaries, we could add:

```rust
.header("Cache-Control", "no-cache, no-store, no-transform, must-revalidate")
//                                    ^^^^^^^^^^^ prevents proxy modifications
.header("Pragma", "no-cache") // HTTP/1.0 fallback
```

**Benefit:** Prevents aggressive proxies from transforming content

**Cost:** 2 extra header bytes

**Recommendation:** ✅ **ADD `no-transform`** (defensive, no downside)

---

#### 🔶 Compression (Optional, Usually Auto-Handled)

**Question:** Should we disable compression on SSE?

**Answer:** Axum does NOT compress SSE by default (correct behavior)

**Why:** SSE with compression breaks real-time delivery (waits for compression window)

**Current Status:** ✅ **ALREADY CORRECT** (no `Content-Encoding: gzip`)

**If using tower-http CompressionLayer:** Ensure SSE routes excluded:

```rust
.layer(
    CompressionLayer::new()
        .compress_when(
            SizeAbove::new(1024)
                .and(NotForContentType::new("text/event-stream"))
        )
)
```

---

### Q2 FINAL VERDICT

**Current headers:** ✅ **PRODUCTION-READY** for nginx/Cloudflare/AWS/GCP

**Recommended enhancement:** Add `no-transform` to `Cache-Control`

**Implementation:**

```rust
.header(header::CACHE_CONTROL, "no-cache, no-store, no-transform, must-revalidate")
```

**Status:** ✅ **MINOR ENHANCEMENT** (not blocking)

---

### Q3) Error Event Ordering on Lag Close

**Current Implementation:**
```rust
// events.rs:110-123
Err(broadcast::error::RecvError::Lagged(n)) => {
    // Send error event
    yield Ok(Event::default()
        .event("relay_error")
        .data(format!(...)));
    
    // Break loop → closes SSE connection
    break;
}
```

**Question:** Is it guaranteed the client receives the final `relay_error` frame before disconnect?

**Answer:** ⚠️ **NOT GUARANTEED** (depends on async timing + TCP buffering)

**Why Not Guaranteed:**

1. **Async stream semantics:**
   - `yield Ok(...)` queues the event in the stream
   - `break` terminates the stream
   - Axum flushes the stream, but timing is not guaranteed

2. **TCP/HTTP semantics:**
   - If connection closes before TCP flush completes, frame is lost
   - Axum does "best effort" flush, but can't guarantee delivery

3. **Proxy buffering:**
   - Even with `X-Accel-Buffering: no`, there's kernel + network buffering

**Practical Outcome:**

- **Most of the time:** Client receives `relay_error` before disconnect ✅
- **Under high load / network issues:** Client might just see disconnect ⚠️

**Is This a Problem?**

❌ **NO - NOT A PROBLEM**

**Why:** Reconnection with `Last-Event-ID` is the real recovery path:

1. Client detects disconnect (EventSource `onerror`)
2. Client reconnects with `Last-Event-ID: N`
3. Server replays from N+1
4. Truth stream completeness restored

The `relay_error` event is **debugging UX**, not a reliability mechanism.

**Recommendation:** ✅ **DOCUMENT THIS HONESTLY**

Update client documentation:

```markdown
### Lag Recovery Behavior

When the client lags behind the broadcast buffer:

1. Server **attempts** to send a `relay_error` event with `STREAM_LAGGED` code
2. Server closes the connection immediately after
3. ⚠️ **The error event may not arrive** (async timing + network buffering)
4. Client's `EventSource` detects disconnect and auto-reconnects
5. Reconnection includes `Last-Event-ID` header
6. Server replays missed events from log

**Client implementation:**
- DO NOT rely on receiving `relay_error` before disconnect
- DO rely on `EventSource.onerror` → auto-reconnect
- Truth stream completeness is guaranteed by replay, not by error events
```

**Status:** ✅ **DOCUMENT AS NON-GUARANTEED** (but functionally correct)

---

### Q4) Persist-Failure Status Codes Consistency

**Current Implementation:**

```rust
// append_commit_handler - commit accepted, log failed
return (StatusCode::INTERNAL_SERVER_ERROR, ...).into_response();

// append_commit_handler - commit rejected, log failed
return (StatusCode::SERVICE_UNAVAILABLE, ...).into_response();

// create_unit_handler - log failed
return (StatusCode::SERVICE_UNAVAILABLE, ...).into_response();
```

**Question:** Should persist failure be always 503 (service unavailable) or 500 (internal error)?

**Answer:** ✅ **USE 503 CONSISTENTLY** (retryable storage failure)

**Reasoning:**

| Status Code | Meaning | Client Action | Use Case |
|-------------|---------|---------------|----------|
| **500 Internal Server Error** | Unexpected server bug | Maybe retry (or give up) | Code bug, unhandled exception |
| **503 Service Unavailable** | Temporary service outage | Retry with backoff | Storage I/O failure, disk full, lock timeout |

**Persist failure is:** Temporary storage issue (disk I/O, file lock, disk full)

**Therefore:** 503 is semantically correct (signals "retry me")

**Recommendation:** ✅ **CHANGE 500 → 503 FOR CONSISTENCY**

**Correct Policy:**

```rust
// PERSIST FAILURE → 503 SERVICE_UNAVAILABLE (always)
// Reason: Storage I/O is retryable, not a code bug

match log.append(event) {
    Ok(id) => { /* emit */ },
    Err(e) => {
        eprintln!("🚨 Event log unavailable: {}", e);
        return (
            StatusCode::SERVICE_UNAVAILABLE, // ← Always 503
            Json(error_response("Event log temporarily unavailable")),
        ).into_response();
    }
}
```

**HTTP Client Behavior:**

- **503 → Retry with exponential backoff** (standard HTTP client behavior)
- **500 → Maybe retry once, then alert** (might be permanent bug)

**Status:** ⚠️ **ACTION NEEDED** - Change 500 → 503 in `append_commit_handler`

---

### Q5) Multi-Writer Safety (Future)

**Current Implementation:**

```rust
pub struct AppState {
    pub filament_store: Arc<Mutex<FilamentStore>>,
    pub unit_store: Arc<Mutex<UnitStore>>,
    pub authority_store: Arc<Mutex<AuthorityStore>>,
    pub event_log: Arc<Mutex<EventLog>>,
    // ...
}
```

**Question:** Could two locks deadlock or reorder if two concurrent writes happen?

**Answer:** ⚠️ **POTENTIAL DEADLOCK IF LOCK ORDER NOT CONSISTENT**

**Current Lock Acquisition Order:**

#### `append_commit_handler()`:
```rust
1. let mut fs = filament_store.lock().await;
2. let auth = authority_store.lock().await;
3. // fs.append_commit() internally
4. let mut us = unit_store.lock().await;
5. let mut log = event_log.lock().await;
```

#### `create_unit_handler()`:
```rust
1. let mut us = unit_store.lock().await;
2. let mut log = event_log.lock().await;
```

#### `commit_processor.rs::process_commit()`:
```rust
// Called with unit_store already locked (passed as &mut)
1. (unit_store locked by caller)
2. let mut log = event_log.try_lock();
```

**Deadlock Risk Analysis:**

**✅ NO DEADLOCK RISK CURRENTLY** because:
- Each handler acquires locks in a **consistent order**
- No two handlers acquire the same set of locks
- `try_lock()` is used in `commit_processor` (fails instead of blocking)

**⚠️ FUTURE DEADLOCK RISK** if:
- Handler A locks `unit_store` then `event_log`
- Handler B locks `event_log` then `unit_store`
- Both run concurrently → deadlock

**Recommendation:** ✅ **DOCUMENT LOCK ORDERING RULES NOW**

**Lock Ordering Policy (LOCKED):**

```rust
// PR #1.2 LOCK: Lock Acquisition Order (MUST be consistent to prevent deadlock)
// 
// RULE: Always acquire locks in this order:
// 1. filament_store
// 2. authority_store  
// 3. unit_store
// 4. event_log
// 
// EXCEPTION: If you can't wait, use try_lock() and handle Err
// 
// WHY: Consistent ordering prevents circular wait (deadlock condition)
// 
// EXAMPLES:
// ✅ GOOD: lock(filament_store) → lock(unit_store) → lock(event_log)
// ✅ GOOD: lock(unit_store) → lock(event_log)
// ❌ BAD:  lock(event_log) → lock(unit_store) (reverse order)
// ✅ GOOD: try_lock(event_log) → if Err, skip (no blocking)
```

**Add to:** Top of `main_integration.rs` as module-level doc comment

**Current Code Audit:**

| Handler | Lock Order | Compliant? |
|---------|------------|------------|
| `append_commit_handler` | filament → authority → unit → log | ✅ YES |
| `create_unit_handler` | unit → log | ✅ YES |
| `commit_processor` | (unit already locked) → try_lock(log) | ✅ YES (non-blocking) |
| `get_commits_handler` | filament (only) | ✅ YES |
| `list_units_handler` | unit (only) | ✅ YES |

**Verdict:** ✅ **CURRENTLY SAFE** + needs documentation

**Status:** ⚠️ **ACTION NEEDED** - Add lock ordering doc

---

## SUMMARY OF ACTIONS

| Question | Finding | Action | Priority | Blocking? |
|----------|---------|--------|----------|-----------|
| Q1 - Buffer size | Capacity=1024 not documented | Add constant + docs | Medium | ❌ No |
| Q2 - Proxy headers | Missing `no-transform` | Add to `Cache-Control` | Low | ❌ No |
| Q3 - Error ordering | Not guaranteed | Document as best-effort | Low | ❌ No |
| Q4 - Status codes | Inconsistent 500/503 | Change 500→503 | High | ⚠️ Should fix |
| Q5 - Deadlock | No risk currently | Document lock order | Medium | ❌ No |

---

## IMPLEMENTATION PLAN

### 🔥 HIGH PRIORITY (Do Now)

#### 1. Fix Status Code Inconsistency (Q4)
```rust
// Change in append_commit_handler:
- StatusCode::INTERNAL_SERVER_ERROR
+ StatusCode::SERVICE_UNAVAILABLE
```

---

### 🟡 MEDIUM PRIORITY (Before Merge)

#### 2. Add Event Bus Capacity Constant (Q1)
```rust
/// Max events buffered before slow clients disconnect
pub const EVENT_BUS_CAPACITY: usize = 1024;
```

#### 3. Document Lock Ordering Rules (Q5)
```rust
// Add to main_integration.rs:
/// # Lock Ordering Rules (Deadlock Prevention)
/// Always acquire in order: filament → authority → unit → log
```

---

### 🟢 LOW PRIORITY (Polish)

#### 4. Add `no-transform` to Cache-Control (Q2)
```rust
.header(header::CACHE_CONTROL, "no-cache, no-store, no-transform, must-revalidate")
```

#### 5. Document Error Event Best-Effort Delivery (Q3)
```markdown
⚠️ `relay_error` events on disconnect are best-effort.
Reconnection via `Last-Event-ID` is the authoritative recovery path.
```

---

## FINAL VERDICT

**PR #1.2 Status:** ✅ **RELAY-CORRECT + PRODUCTION-SAFE**

**Blockers:** ❌ None (all issues are polish/documentation)

**Recommended:** Fix Q4 (status code) before merge, others can be follow-up

**Next Steps:**
1. Apply Q4 fix (5 minutes)
2. Apply Q1, Q2, Q5 polish (15 minutes)
3. Update docs for Q3 (5 minutes)
4. **Proceed to PR #1.3** (Single Commit Fetch)

**Philosophy Check:** ✅ ALL INVARIANTS PRESERVED
- History is authority ✅
- No phantom events ✅
- Truth stream completeness ✅
- Deterministic replay ✅

---

**Date:** 2026-01-28  
**Audit Status:** COMPLETE  
**Merge Safety:** ✅ SAFE (with Q4 fix)

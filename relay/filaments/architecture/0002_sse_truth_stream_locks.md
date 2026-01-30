# COMMIT C: SSE Truth Stream Locks

**Filament:** `architecture`  
**Commit Index:** 2  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** DESIGN_LOCK

---

## INVARIANT LOCKED

**The SSE event stream enforces: no phantom events, truth stream completeness, deterministic replay.**

---

## PHILOSOPHY

**"If it's not in the log, it never happened."**

The event log is the single source of truth. The SSE stream is a real-time **view** of that truth, not a separate authority.

---

## LOCKED DECISIONS (Q1-Q5 AUDIT)

### 1. Event ID Authority

**Lock:** Event IDs come ONLY from `EventLog::append()` (monotonic counter owned by log)

**Enforcement:**
```rust
let event_id = match log.append(event) {
    Ok(id) => id,
    Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, ...).into_response(),
};

// Only emit after successful persist
state.events.emit_with_id(event_id, event);
```

**Invariant:** History assigns authority. No emit without persist.

**Status:** ✅ IMPLEMENTED (PR #1.2)

---

### 2. Persist Failure is Fatal

**Lock:** If `EventLog::append()` fails → NEVER EMIT (return HTTP 503)

**Rejected Alternative:** Emit with placeholder ID (`u64::MAX`)

**Reasoning:** 
- Emitting without persisting creates "phantom events" with no authority
- Phantom events can't be replayed (violates determinism)
- Client should retry request (HTTP 503 signals retryable failure)

**HTTP Status:**
- Persist success → 200 OK (emit event)
- Persist failure → **503 SERVICE_UNAVAILABLE** (no emit, client retries)

**Status:** ✅ IMPLEMENTED (PR #1.2, Q4 fix)

---

### 3. Lagged Stream = Broken Stream → Disconnect

**Lock:** If SSE client lags behind broadcast buffer → **CLOSE CONNECTION**

**Rejected Alternative:** Warn and continue (creates incomplete stream)

**Reasoning:**
- A "truth stream" with gaps is not a truth stream
- Client can reconnect with `Last-Event-ID` to restore completeness
- Automatic recovery (browser `EventSource` handles reconnect)
- Forces replay from log (source of truth)

**Implementation:**
```rust
Err(broadcast::error::RecvError::Lagged(n)) => {
    // Send error event (best-effort)
    yield Ok(Event::default()
        .event("relay_error")
        .data(format!(...)));
    
    break; // Close connection → client reconnects
}
```

**Client Behavior:**
1. Receives `relay_error` event (best-effort, not guaranteed)
2. Connection closes
3. Browser `EventSource` auto-reconnects with `Last-Event-ID`
4. Server replays missed events from log
5. Truth stream completeness restored

**Status:** ✅ IMPLEMENTED (PR #1.2)

---

### 4. Broadcast Buffer Capacity

**Lock:** `EVENT_BUS_CAPACITY = 1024` events

**What This Means:**
- Broadcast channel buffers up to 1024 events before slow consumers lag
- If more than 1024 events accumulate → lag → disconnect (see #3)

**Production Tuning Guide:**

| Load Pattern | Capacity | Lag Tolerance |
|--------------|----------|---------------|
| Low (< 1 event/sec) | 256 | ~4 minutes |
| Medium (1-10 events/sec) | **1024** | ~2 minutes |
| High (10-100 events/sec) | 4096 | ~40 seconds |
| Burst (100+ events/sec) | 8192 | ~80 seconds |

**Status:** ✅ DOCUMENTED (PR #1.2, Q1 fix)

---

### 5. Production Headers (Proxy Compatibility)

**Lock:** Explicit no-cache + no-buffer headers for nginx/Cloudflare/AWS/GCP

**Headers:**
```rust
.header(header::CACHE_CONTROL, "no-cache, no-store, no-transform, must-revalidate")
.header(header::CONNECTION, "keep-alive")
.header("X-Accel-Buffering", "no") // Nginx: disable buffering
```

**Why Each Header:**
1. **`Cache-Control: no-cache, no-store, no-transform, must-revalidate`**
   - Prevents CDN/proxy from caching/transforming SSE stream
   - `no-transform` added for aggressive intermediaries (Q2 fix)

2. **`Connection: keep-alive`**
   - Keeps connection open (standard for SSE)
   - Some proxies close idle connections without this

3. **`X-Accel-Buffering: no`**
   - Nginx-specific: disables response buffering
   - Without this, events may be delayed until buffer fills

**Status:** ✅ IMPLEMENTED (PR #1.2, Q2 fix)

---

### 6. Error Event Delivery (Best-Effort)

**Lock:** `relay_error` events on disconnect are **best-effort** (not guaranteed)

**Reasoning:**
- Async stream semantics + TCP buffering = no delivery guarantee
- Connection may close before final SSE frame flushes

**Documented Behavior:**
- Server **attempts** to send `relay_error` before closing
- Client **may not receive** it (timing dependent)
- **Authoritative recovery path:** Client detects disconnect → EventSource auto-reconnects with `Last-Event-ID` → server replays from log

**Client Implementation:**
```javascript
const eventSource = new EventSource('/api/relay-physics/events');

eventSource.addEventListener('relay_error', (event) => {
  // MAY receive this before disconnect
  console.warn('Stream error:', JSON.parse(event.data));
});

eventSource.onerror = () => {
  // ALWAYS receives this on disconnect
  console.log('Reconnecting...');
  // EventSource auto-reconnects with Last-Event-ID
};
```

**Status:** ✅ DOCUMENTED (PR #1.2, Q3 fix)

---

### 7. Lock Ordering (Deadlock Prevention)

**Lock:** Always acquire locks in this order:

1. `filament_store`
2. `authority_store`
3. `unit_store`
4. `event_log`

**Exception:** If you can't wait, use `try_lock()` and handle `Err` (non-blocking)

**Why:** Consistent ordering prevents circular wait (deadlock condition)

**Current Handler Audit:**

| Handler | Lock Order | Compliant? |
|---------|------------|------------|
| `append_commit_handler` | filament → authority → unit → log | ✅ YES |
| `create_unit_handler` | unit → log | ✅ YES |
| `commit_processor` | (unit pre-locked) → try_lock(log) | ✅ YES (non-blocking) |

**Status:** ✅ DOCUMENTED (PR #1.2, Q5 fix)

---

## REPLAY SEMANTICS (LOCKED)

### Last-Event-ID Translation

**Client sends:** `Last-Event-ID: N` (last received)

**Server interprets:** `load_from(N+1)` (start after what client saw)

**Reasoning:** `Last-Event-ID` means "I have this, give me next"

**Implementation:**
```rust
let start_event_id = headers
    .get("Last-Event-ID")
    .and_then(|v| v.to_str().ok())
    .and_then(|s| s.parse::<u64>().ok())
    .or(query.after)
    .map(|id| id + 1); // ← Client sends last received; we start from next
```

**Status:** ✅ IMPLEMENTED (PR #1.2)

---

### Defensive Duplicate Filtering

**Problem:** Replay-to-live transition might see same event twice

**Solution:** Track `last_sent_id` and skip duplicates

**Implementation:**
```rust
if let Some(last_id) = last_sent_id {
    if event_id <= last_id {
        continue; // Skip duplicates
    }
}
```

**Status:** ✅ IMPLEMENTED (PR #1.2)

**Test:** `test_sse_defensive_duplicate_filtering`

---

## INVARIANTS SUMMARY

| Invariant | Enforcement | Violation Prevention |
|-----------|-------------|---------------------|
| **History is Authority** | Event ID from log only | No phantom events |
| **No Emit Without Persist** | Persist failure → 503 | No emit on log failure |
| **Truth Stream Complete** | Lag → disconnect | No gaps in stream |
| **Deterministic Replay** | Same log → same stream | Dedup filter |
| **No Proxy Buffering** | Explicit headers | Events arrive immediately |
| **No Deadlock** | Lock order documented | Consistent acquisition |

---

## TESTS (LOCKED)

### Automated (5 tests)
1. ✅ `test_sse_replay_from_last_event_id` - Replay events 5-9 after Last-Event-ID: 4
2. ✅ `test_sse_live_append` - Live event reaches connected client
3. ✅ `test_sse_idempotent_reconnection` - Reconnect delivers 8-9 after Last-Event-ID: 7
4. ✅ `test_sse_last_event_id_priority` - Header > query param
5. ✅ `test_sse_defensive_duplicate_filtering` - Overlapping replay/live filtered

### Manual (3 validation tests)
1. ⏭️ Persist failure → HTTP 503, no SSE event (chmod event log)
2. ⏭️ Lag disconnect → connection closes (spam 2000 events)
3. ⏭️ Nginx buffering → immediate streaming (X-Accel-Buffering header)

---

## CAUSAL REFS

- **Inputs:** 
  - `architecture@c0` (Layer split)
  - `architecture@c1` (Render endpoints)
- **Authority:** system.architect
- **Evidence:** 
  - PR #1.1 (Event log implementation)
  - PR #1.2 (SSE implementation + Q1-Q5 audit)
  - `PR-1.2-DESIGN-LOCKS.md` (full decision rationale)

---

**Status:** LOCKED  
**Supersedes:** None  
**Philosophy:** History is authority. The log is truth. The stream is a view.

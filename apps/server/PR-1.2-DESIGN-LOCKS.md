# PR #1.2: Design Decisions - LOCKED

**Date:** 2026-01-28  
**Status:** ✅ PHILOSOPHICALLY ALIGNED WITH "HISTORY IS AUTHORITY"

---

## DESIGN PHILOSOPHY

**Core Invariant:** If it's not in the log, it never happened.

**Corollary:** The SSE stream is a real-time view of log truth, not a separate source of authority.

---

## LOCKED DECISIONS

### 🔒 DECISION A: Persist Failure is Fatal

**Question:** If `EventLog::append()` fails, should we emit the event anyway?

**Answer:** ❌ **NO - NEVER EMIT WITHOUT PERSIST**

**Reasoning:**
1. Relay's core principle: **history assigns authority**
2. Emitting without persisting creates "phantom events" with no authority
3. Phantom events can't be replayed (violates determinism)
4. Client should retry the request (HTTP 500/503 signals failure)

**Implementation:**
```rust
let event_id = match log.append(event.clone()) {
    Ok(id) => id,
    Err(e) => {
        eprintln!("🚨 FATAL: Failed to persist event to log: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, ...).into_response();
    }
};

// Only emit after successful persist
state.events.emit_with_id(event_id, event);
```

**HTTP Status Codes:**
- Commit accepted but log failed → `500 INTERNAL_SERVER_ERROR`
- Commit rejected and log failed → `503 SERVICE_UNAVAILABLE`
- Unit creation but log failed → `503 SERVICE_UNAVAILABLE`

**Client Behavior:**
- Receives error response
- Retries request (idempotent)
- OR alerts user to storage failure

**Locked:** ✅ Implemented in `main_integration.rs` + `commit_processor.rs`

---

### 🔒 DECISION B: Lagged Stream = Broken Stream → Disconnect

**Question:** If SSE client lags behind broadcast buffer, should we warn or disconnect?

**Answer:** 🔴 **DISCONNECT** (close SSE connection)

**Reasoning:**
1. A "truth stream" with gaps is not a truth stream
2. Client can reconnect with `Last-Event-ID` to restore completeness
3. Automatic recovery (browser `EventSource` handles reconnect)
4. Forces replay from log (source of truth)

**Implementation:**
```rust
Err(broadcast::error::RecvError::Lagged(n)) => {
    eprintln!("🚨 SSE client lagged {} events - closing connection to force replay", n);
    
    // Send error event before closing
    yield Ok(Event::default()
        .event("relay_error")
        .data(format!(r#"{{"error":"STREAM_LAGGED","lagged":{},"message":"Connection closing. Reconnect to replay missed events."}}"#, n)));
    
    break; // Close connection
}
```

**Client Behavior:**
1. Receives `relay_error` event with `STREAM_LAGGED`
2. Connection closes
3. Browser `EventSource` auto-reconnects with `Last-Event-ID`
4. Server replays missed events from log
5. Truth stream completeness restored

**Alternative (Rejected):**
- Warn and continue → stream becomes incomplete
- Client must manually detect and refetch state
- Violates "truth stream" guarantee

**Locked:** ✅ Implemented in `events.rs`

---

### 🔒 DECISION C: Production Headers for Proxy Compatibility

**Question:** What headers are needed for nginx/Cloudflare/load balancers?

**Answer:** Set explicit no-cache + no-buffer headers

**Headers Added:**
```rust
.header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
.header(header::CONNECTION, "keep-alive")
.header("X-Accel-Buffering", "no") // Nginx: disable buffering
```

**Why Each Header:**

1. **`Cache-Control: no-cache, no-store, must-revalidate`**
   - Prevents CDN/proxy from caching SSE stream
   - Critical for real-time events

2. **`Connection: keep-alive`**
   - Keeps connection open (standard for SSE)
   - Some proxies close idle connections without this

3. **`X-Accel-Buffering: no`**
   - Nginx-specific: disables response buffering
   - Without this, events may be delayed until buffer fills

**Cloudflare Note:**
- Cloudflare auto-detects `text/event-stream` and disables buffering
- But explicit headers don't hurt

**AWS ALB / GCP Load Balancer:**
- These respect `Connection: keep-alive` + SSE content-type
- No additional headers needed

**Locked:** ✅ Implemented in `events_sse_handler()`

---

## INVARIANTS ENFORCED

### 1. ✅ History is Authority

**Invariant:** Event ID comes only from `EventLog::append()`

**Enforcement:**
```rust
let event_id = log.append(event)?; // Returns monotonic ID from log
state.events.emit_with_id(event_id, event);
```

**Test:** `test_event_log_append_and_replay` verifies IDs match log

---

### 2. ✅ No Phantom Events

**Invariant:** If persist fails, event never emitted

**Enforcement:**
```rust
match log.append(event) {
    Ok(id) => { /* emit */ },
    Err(e) => return error_response, // Never emit
}
```

**Test:** Manual test - kill event log file → requests return 503

---

### 3. ✅ Truth Stream Completeness

**Invariant:** SSE stream is complete or closed (no gaps)

**Enforcement:**
```rust
Lagged(_) => break; // Close connection
```

**Test:** `test_sse_defensive_duplicate_filtering` + manual lag test

---

### 4. ✅ Deterministic Replay

**Invariant:** Same log → same stream

**Enforcement:**
- Replay from log (phase 1)
- Defensive duplicate filter (phase 2)

**Test:** `test_sse_idempotent_reconnection`

---

### 5. ✅ No Buffering

**Invariant:** Events stream immediately (not buffered by proxy)

**Enforcement:**
- `Cache-Control: no-cache`
- `X-Accel-Buffering: no`

**Test:** Manual test behind nginx

---

## ERROR HANDLING SUMMARY

### Persist Failures

| Scenario | HTTP Status | SSE Emitted? | Client Action |
|----------|-------------|--------------|---------------|
| Commit accepted, log success | 200 OK | ✅ Yes | Process event |
| Commit accepted, log failed | 500 ERROR | ❌ No | Retry request |
| Commit rejected, log success | 400 BAD REQUEST | ✅ Yes | Handle rejection |
| Commit rejected, log failed | 503 UNAVAILABLE | ❌ No | Retry or alert |
| Unit created, log success | 200 OK | ✅ Yes | Process event |
| Unit created, log failed | 503 UNAVAILABLE | ❌ No | Retry or alert |

---

### Broadcast Lag

| Scenario | SSE Action | Client Receives | Auto-Recovery |
|----------|------------|-----------------|---------------|
| Client caught up | Stream normally | Events | N/A |
| Client lagged N events | Close connection | `relay_error` event | EventSource reconnects |
| Reconnect with Last-Event-ID | Replay from log | Missed events | ✅ Completeness restored |

---

## CLIENT IMPLEMENTATION GUIDE

### Handling Persist Failures (HTTP Errors)

```javascript
async function createUnit(unitId) {
  try {
    const response = await fetch('/api/relay-physics/units', {
      method: 'POST',
      body: JSON.stringify({ unit_id: unitId }),
    });
    
    if (response.ok) {
      return await response.json();
    } else if (response.status === 503) {
      // Event log unavailable - retry with exponential backoff
      console.error('Storage unavailable, retrying...');
      await sleep(1000);
      return createUnit(unitId); // Retry
    } else {
      throw new Error(`Failed: ${response.status}`);
    }
  } catch (e) {
    console.error('Request failed:', e);
    throw e;
  }
}
```

---

### Handling Lagged Stream (Auto-Reconnection)

**Good news:** Browser `EventSource` handles this automatically!

```javascript
const eventSource = new EventSource('/api/relay-physics/events');

eventSource.addEventListener('relay_event', (event) => {
  // Process event normally
  const envelope = JSON.parse(event.data);
  console.log('Event:', envelope.event_id, envelope.event);
});

eventSource.addEventListener('relay_error', (event) => {
  const error = JSON.parse(event.data);
  
  if (error.error === 'STREAM_LAGGED') {
    console.warn(`Stream lagged ${error.lagged} events, reconnecting...`);
    // Connection will close, EventSource auto-reconnects with Last-Event-ID
    // No manual action needed!
  }
});

eventSource.onerror = () => {
  console.log('Connection closed, EventSource will auto-reconnect...');
  // Browser handles reconnection automatically
};
```

---

### Manual Reconnection (If Needed)

```javascript
let lastEventId = null;

const connect = () => {
  const url = lastEventId
    ? `/api/relay-physics/events?after=${lastEventId}`
    : '/api/relay-physics/events';
  
  const eventSource = new EventSource(url);
  
  eventSource.addEventListener('relay_event', (event) => {
    const envelope = JSON.parse(event.data);
    lastEventId = envelope.event_id; // Store for reconnect
    
    // Process event...
  });
  
  eventSource.onerror = () => {
    eventSource.close();
    setTimeout(connect, 1000); // Reconnect after 1s
  };
};

connect();
```

---

## TESTING VALIDATION

### Test 1: Persist Failure → No Emit

**Setup:**
```bash
# Make event log read-only
chmod 444 var/relay_physics/events.jsonl
```

**Action:**
```bash
curl -X POST http://localhost:3002/api/relay-physics/units \
  -d '{"unit_id":"unit.test.001"}'
```

**Expected:**
- HTTP 503 SERVICE_UNAVAILABLE
- SSE clients receive **no event** (phantom event prevented)
- Error logged: `🚨 FATAL: Failed to persist unit creation to log`

---

### Test 2: Lagged Client → Auto-Reconnect

**Setup:**
```bash
# Terminal 1: Subscribe to SSE
curl -N http://localhost:3002/api/relay-physics/events

# Terminal 2: Spam 2000 events (exceed buffer size 1024)
for i in {1..2000}; do
  curl -X POST http://localhost:3002/api/relay-physics/units \
    -d "{\"unit_id\":\"unit.spam.$i\"}" &
done
```

**Expected:**
- Terminal 1 receives `relay_error` event: `{"error":"STREAM_LAGGED",...}`
- Connection closes
- Browser `EventSource` auto-reconnects with `Last-Event-ID`
- Server replays missed events from log

---

### Test 3: Nginx Buffering Disabled

**Setup:** Deploy behind nginx with:
```nginx
location /api/relay-physics/events {
    proxy_pass http://localhost:3002;
    proxy_buffering off; # Should not be needed with X-Accel-Buffering header
}
```

**Action:**
```bash
curl -N http://nginx-host/api/relay-physics/events
```

**Expected:**
- Events stream immediately (not buffered)
- Response headers include `X-Accel-Buffering: no`

---

## ROLLBACK PLAN (If Issues Found)

### Issue: Too Many 503 Errors (Storage Instability)

**Symptom:** Frequent event log write failures

**Short-term Fix:**
```rust
// Fallback to best-effort (warn but don't fail request)
let event_id = match log.append(event.clone()) {
    Ok(id) => id,
    Err(e) => {
        eprintln!("⚠️ WARNING: Failed to persist: {}", e);
        u64::MAX // Temporary fallback (not ideal)
    }
};
```

**Long-term Fix:**
- Add event log replication
- Use SQLite with WAL mode (more reliable)
- Add metrics + alerting for persist failures

---

### Issue: Too Many Reconnections (Lag Threshold Too Low)

**Symptom:** Clients constantly reconnecting due to lag

**Fix:** Increase broadcast buffer size:
```rust
EventBus::new(4096) // Instead of 1024
```

**OR:** Add configurable lag tolerance:
```rust
if n > LAG_TOLERANCE_THRESHOLD {
    break; // Disconnect only if lag exceeds threshold
} else {
    continue; // Warn but continue
}
```

---

## PRODUCTION READINESS CHECKLIST

### ✅ Core Invariants Locked
- [x] Event IDs from log only (no phantom events)
- [x] Persist failure → no emit (truth stream integrity)
- [x] Lagged stream → disconnect (completeness guarantee)
- [x] Proxy headers set (no buffering)

### ✅ Error Handling
- [x] HTTP 500 on persist failure (commit)
- [x] HTTP 503 on persist failure (unit/rejection)
- [x] SSE closes on lag (with error event)

### ✅ Tests Updated
- [x] 5 SSE tests passing (replay, live, idempotent, priority, dedup)
- [ ] Add persist failure test (manual validation)
- [ ] Add lag disconnect test (manual validation)

### ✅ Documentation
- [x] PR-1.2-SSE-REPLAY.md updated
- [x] PR-1.2-DESIGN-LOCKS.md created (this file)
- [x] Client implementation guide included

---

## FINAL VERDICT

**PR #1.2 Status:** ✅ **PHILOSOPHICALLY ALIGNED + MERGE-SAFE**

**Locked Decisions:**
1. ✅ Persist failure is fatal (no phantom events)
2. ✅ Lagged stream disconnects (truth stream completeness)
3. ✅ Production headers set (proxy compatibility)

**Remaining Work:** Manual validation tests (persist failure, lag disconnect)

**Ready for:** Dev team review + merge

---

**Date Locked:** 2026-01-28  
**Version:** PR #1.2 Final  
**Philosophy:** History is authority. The log is truth. The stream is a view.

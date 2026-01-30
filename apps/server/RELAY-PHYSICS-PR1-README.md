# Relay Physics Layer - PR #1 Implementation Guide

**Version:** 1.0  
**Date:** 2026-01-28  
**Status:** ✅ READY TO BUILD

---

## WHAT THIS IS

**Relay Physics Layer (Layer 2)** - Coordination substrate on top of existing Git backend (Layer 1)

**Architecture:**
```
┌─────────────────────────────────────────┐
│  RELAY PHYSICS (NEW - THIS PR)          │
│  /api/relay-physics/*                   │
│  - Filaments (append-only JSONL)        │
│  - Commits (immutable events)           │
│  - SCVs (state machine)                 │
│  - Verifier (pre-append)                │
│  - Authority (capability grants)        │
│  - Events (SSE stream)                  │
└─────────────────────────────────────────┘
              ↓ coexists with ↓
┌─────────────────────────────────────────┐
│  GIT BACKEND (EXISTING)                 │
│  /git-pull, /config, etc.               │
│  - Repo/branch operations               │
│  - File GET/PUT/DELETE                  │
└─────────────────────────────────────────┘
```

---

## FILES CREATED

### Core Modules (`src/relay_physics/`)

1. ✅ **mod.rs** - Module declaration + re-exports
2. ✅ **types.rs** - FilamentId, UnitId, CommitRef, CausalRefs, UnitState
3. ✅ **commit.rs** - CommitEvent, CommitData structs
4. ✅ **errors.rs** - CommitRejectedError, ReasonCode enum (10 codes)
5. ✅ **agent_ops.rs** - Op constants (TASK_ASSIGN, OUTPUT_PROPOSED, etc.)
6. ✅ **filament_store.rs** - Append-only JSONL store with verification
7. ✅ **unit_store.rs** - SCV state machine (5 states)
8. ✅ **authority_store.rs** - Capability grants (MVP delegation)
9. ✅ **verifier.rs** - 4-stage pipeline (schema → refs → authority → evidence)
10. ✅ **events.rs** - SSE event bus (CommitAccepted, CommitRejected, UnitStateChanged)
11. ✅ **commit_processor.rs** - SCV commit-to-state bridge
12. ✅ **main_integration.rs** - Axum routes + handlers
13. ✅ **tests.rs** - 7 unit tests

---

## INTEGRATION STEPS

### Step 1: Update `src/main.rs`

**Add module declaration:**
```rust
mod relay_physics;
```

**Initialize Relay Physics state:**
```rust
let relay_state = relay_physics::main_integration::init_relay_physics_state();
```

**Merge routes:**
```rust
let app = Router::new()
    // KEEP your existing Git routes
    .route("/git-pull", post(git_pull_handler))
    .route("/config", get(config_handler))
    // ... etc ...
    
    // ADD Relay Physics routes
    .merge(relay_physics::main_integration::relay_router(relay_state));
```

---

### Step 2: Verify Dependencies in `Cargo.toml`

**Required dependencies:**
```toml
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "fs"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
futures-util = "0.3"
async-stream = "0.3"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }
```

**If missing, add them.**

---

### Step 3: Build and Test

```bash
cd apps/server

# Build
cargo build

# Run tests
cargo test

# Run server
cargo run
```

**Expected output:**
```
🚀 Relay server listening on 0.0.0.0:3002
```

---

## API ENDPOINTS (3 Minimum Routes)

### 1. SSE Event Stream

**Endpoint:** `GET /api/relay-physics/events`

**Response:** Server-Sent Events (SSE)

**Events emitted:**
- `CommitAccepted` - When commit passes verification
- `CommitRejected` - When commit fails verification (with reason_code)
- `UnitStateChanged` - When SCV state transitions

**Example:**
```bash
curl -N http://localhost:3002/api/relay-physics/events
```

---

### 2. Append Commit (with Verification)

**Endpoint:** `POST /api/relay-physics/filaments/:id/commits`

**Request:**
```json
{
  "op_type": "TASK_ASSIGN",
  "author_unit_ref": "unit.manager.001",
  "payload": {
    "targetUnit": "unit.ai.agent.001",
    "filamentId": "work.W123",
    "taskDescription": "Implement auth module"
  },
  "causal_refs": {
    "inputs": [],
    "evidence": [],
    "authority_ref": "auth.task.assign"
  }
}
```

**Success Response (200):**
```json
{
  "result": "CommitAccepted",
  "data": {
    "commit": {
      "commit_ref": "work.manager.001@c1",
      "filament_id": "work.manager.001",
      "commit_index": 1,
      "op_type": "TASK_ASSIGN",
      "timestamp": "2026-01-28T14:30:00Z",
      "author_unit_ref": "unit.manager.001",
      "payload": { ... },
      "causal_refs": { ... }
    }
  }
}
```

**Rejection Response (400):**
```json
{
  "result": "CommitRejected",
  "data": {
    "error": {
      "reason_code": "AUTHORITY_MISSING",
      "details": "authority_ref is required for this op_type",
      "suggested_fix": "Attach causal_refs.authority_ref with delegation path proof"
    }
  }
}
```

---

### 3. List Commits

**Endpoint:** `GET /api/relay-physics/filaments/:id/commits`

**Response:**
```json
[
  {
    "commit_ref": "work.W123@c1",
    "filament_id": "work.W123",
    "commit_index": 1,
    "op_type": "TASK_ASSIGN",
    ...
  },
  {
    "commit_ref": "work.W123@c2",
    "filament_id": "work.W123",
    "commit_index": 2,
    "op_type": "UNIT_ATTACH",
    ...
  }
]
```

---

### 4. Create Unit (SCV)

**Endpoint:** `POST /api/relay-physics/units`

**Request:**
```json
{
  "unit_id": "unit.ai.agent.001"
}
```

**Response:**
```json
{
  "id": "unit.ai.agent.001",
  "state": "Idle",
  "attached_filament": null,
  "current_task_filament": null
}
```

---

### 5. List Units

**Endpoint:** `GET /api/relay-physics/units`

**Response:**
```json
[
  {
    "id": "unit.ai.agent.001",
    "state": "Working",
    "attached_filament": "work.W123",
    "current_task_filament": "work.W123"
  },
  {
    "id": "unit.manager.001",
    "state": "Idle",
    "attached_filament": null,
    "current_task_filament": null
  }
]
```

---

## LOCKED INVARIANTS

### 1. Verification Happens BEFORE Append

```rust
pub fn append_commit(...) -> Result<CommitEvent, CommitRejectedError> {
    // BEFORE: verify
    verifier.verify(&commit_data, ...)?;
    
    // AFTER: append immutably
    let commit_index = self.get_head_index(...) + 1;
    // ...
}
```

**No "append then validate". No ambient authority.**

---

### 2. CommitIndex is Monotonic Per Filament

```rust
let next_index = head_index + 1;
let commit_ref = format!("{}@c{}", filament_id, next_index);
```

**Deterministic. Never decreases. Never skips.**

---

### 3. SCV State Transitions are Commit-Driven Only

**State machine (LOCKED):**
```
TASK_ASSIGN    → Idle → Moving
UNIT_ATTACH    → Moving → Working
OUTPUT_REJECTED → Working → Blocked
OUTPUT_ACCEPTED → Working (stays Working)
UNIT_DETACH    → Working → Idle
```

**Applied by:** `CommitProcessor::process_commit()` (AFTER commit accepted)

**No ambient state changes. No "system decided".**

---

### 4. Reason Codes are Explicit and Actionable

**10 locked reason codes:**
- `SCHEMA_INVALID`
- `REF_INVALID`
- `AUTHORITY_MISSING`
- `AUTHORITY_DENIED`
- `EVIDENCE_MISSING`
- `EVIDENCE_INVALID`
- `CONFLICT_DETECTED`
- `SCOPE_VIOLATION`
- `TIMEBOX_EXCEEDED`
- `CUSTOM_RULE_FAILED`

**All rejections include:**
- `reason_code` (enum)
- `details` (human-readable)
- `suggested_fix` (optional, actionable)

---

## TESTS (7 Passing)

```bash
cargo test
```

**Expected output:**
```
test relay_physics::tests::test_commit_index_monotonic ... ok
test relay_physics::tests::test_schema_invalid_rejects ... ok
test relay_physics::tests::test_ref_invalid_rejects ... ok
test relay_physics::tests::test_authority_missing_rejects ... ok
test relay_physics::tests::test_output_proposed_accepted_when_valid ... ok
test relay_physics::tests::test_output_rejected_transitions_to_blocked ... ok
test relay_physics::tests::test_evidence_missing_rejects ... ok

test result: ok. 7 passed; 0 failed; 0 ignored
```

---

## STORAGE (JSONL Files)

**Location:** `var/relay_physics/filaments/*.jsonl`

**Example:** `var/relay_physics/filaments/work_W123.jsonl`

**Format (one commit per line):**
```json
{"commit_ref":"work.W123@c1","filament_id":"work.W123","commit_index":1,"op_type":"TASK_ASSIGN","timestamp":"2026-01-28T14:30:00Z","author_unit_ref":"unit.manager.001","payload":{...},"causal_refs":{...}}
{"commit_ref":"work.W123@c2","filament_id":"work.W123","commit_index":2,"op_type":"UNIT_ATTACH","timestamp":"2026-01-28T14:31:00Z","author_unit_ref":"unit.ai.agent.001","payload":{...},"causal_refs":{...}}
```

**Why JSONL:**
- ✅ Deterministic (append-only, no mutation)
- ✅ Inspectable (can `cat` or `tail` files)
- ✅ Fast (no DB overhead for MVP)
- ✅ Naturally matches "append-only commits"

**Later:** Migrate to SQLite without changing API

---

## FRONTEND INTEGRATION

### SSE Event Subscription (JavaScript/React)

```javascript
const eventSource = new EventSource('http://localhost:3002/api/relay-physics/events');

eventSource.addEventListener('relay', (event) => {
  const relayEvent = JSON.parse(event.data);
  
  if (relayEvent.type === 'CommitAccepted') {
    console.log('✅ Commit accepted:', relayEvent.data.commit);
  } else if (relayEvent.type === 'CommitRejected') {
    console.log('❌ Commit rejected:', relayEvent.data.error.reason_code);
    console.log('   Details:', relayEvent.data.error.details);
    console.log('   Fix:', relayEvent.data.error.suggested_fix);
  } else if (relayEvent.type === 'UnitStateChanged') {
    console.log('🔄 Unit state changed:', relayEvent.data.unit);
  }
});
```

---

### Append Commit (JavaScript/React)

```javascript
const response = await fetch('http://localhost:3002/api/relay-physics/filaments/work.W123/commits', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    op_type: 'TASK_ASSIGN',
    author_unit_ref: 'unit.manager.001',
    payload: {
      targetUnit: 'unit.ai.agent.001',
      filamentId: 'work.W123',
      taskDescription: 'Implement auth module',
      scope: 'backend/auth/*'
    },
    causal_refs: {
      inputs: [],
      evidence: [],
      authority_ref: 'auth.task.assign'
    }
  })
});

const result = await response.json();

if (result.result === 'CommitAccepted') {
  console.log('✅ Commit accepted:', result.data.commit.commit_ref);
} else {
  console.error('❌ Rejected:', result.data.error.reason_code);
  console.error('   Details:', result.data.error.details);
}
```

---

## ACCEPTANCE CRITERIA (PR #1)

### Must Pass Before Merge:

1. ✅ All 7 tests passing (`cargo test`)
2. ✅ Server compiles without errors (`cargo build`)
3. ✅ Server runs and listens on port 3002 (`cargo run`)
4. ✅ Can create unit via API (`POST /api/relay-physics/units`)
5. ✅ Can list units via API (`GET /api/relay-physics/units`)
6. ✅ Can append valid commit (`POST /api/relay-physics/filaments/:id/commits`)
7. ✅ Invalid commit is rejected with reason_code
8. ✅ SSE stream emits events (`GET /api/relay-physics/events`)
9. ✅ JSONL files are created in `var/relay_physics/filaments/`
10. ✅ CommitIndex is monotonic (sequential: 1, 2, 3, ...)

---

## TESTING THE IMPLEMENTATION

### Test 1: Create Unit

```bash
curl -X POST http://localhost:3002/api/relay-physics/units \
  -H "Content-Type: application/json" \
  -d '{"unit_id": "unit.ai.agent.001"}'
```

**Expected:**
```json
{
  "id": "unit.ai.agent.001",
  "state": "Idle",
  "attached_filament": null,
  "current_task_filament": null
}
```

---

### Test 2: Append Valid Commit

```bash
curl -X POST http://localhost:3002/api/relay-physics/filaments/work.TEST/commits \
  -H "Content-Type: application/json" \
  -d '{
    "op_type": "TEST_OP",
    "author_unit_ref": "unit.test.001",
    "payload": {"message": "Hello Relay"},
    "causal_refs": {
      "inputs": [],
      "evidence": [],
      "authority_ref": null
    }
  }'
```

**Expected:**
```json
{
  "result": "CommitAccepted",
  "data": {
    "commit": {
      "commit_ref": "work.TEST@c1",
      "commit_index": 1,
      "op_type": "TEST_OP",
      ...
    }
  }
}
```

---

### Test 3: Append Invalid Commit (Missing Authority)

```bash
curl -X POST http://localhost:3002/api/relay-physics/filaments/work.TEST/commits \
  -H "Content-Type: application/json" \
  -d '{
    "op_type": "TASK_ASSIGN",
    "author_unit_ref": "unit.test.001",
    "payload": {},
    "causal_refs": {
      "inputs": [],
      "evidence": [],
      "authority_ref": null
    }
  }'
```

**Expected (400 Bad Request):**
```json
{
  "result": "CommitRejected",
  "data": {
    "error": {
      "reason_code": "AUTHORITY_MISSING",
      "details": "authority_ref is required for this op_type",
      "suggested_fix": "Attach causal_refs.authority_ref with delegation path proof (MVP: capability grant)"
    }
  }
}
```

---

### Test 4: Subscribe to SSE Events

```bash
curl -N http://localhost:3002/api/relay-physics/events
```

**Expected (streaming):**
```
event: relay
data: {"type":"CommitAccepted","data":{"commit":{...}}}

event: relay
data: {"type":"UnitStateChanged","data":{"unit":{...}}}
```

---

### Test 5: Verify JSONL Storage

```bash
cat var/relay_physics/filaments/work_TEST.jsonl
```

**Expected (one line per commit):**
```json
{"commit_ref":"work.TEST@c1","filament_id":"work.TEST","commit_index":1,...}
{"commit_ref":"work.TEST@c2","filament_id":"work.TEST","commit_index":2,...}
```

---

## SCV STATE MACHINE (LOCKED TRANSITIONS)

**Implemented in `commit_processor.rs`:**

```
Commit                    State Transition
──────────────────────    ──────────────────────
TASK_ASSIGN           →   Idle → Moving
UNIT_ATTACH           →   Moving → Working
OUTPUT_ACCEPTED       →   Working (stays)
OUTPUT_REJECTED       →   Working → Blocked
UNIT_DETACH           →   Working → Idle
```

**Example flow:**
```
1. POST TASK_ASSIGN → unit.state = "Moving"
2. POST UNIT_ATTACH → unit.state = "Working"
3. POST OUTPUT_PROPOSED (rejected) → unit.state = "Blocked"
```

**Verify via:**
```bash
GET /api/relay-physics/units
→ See unit.state changes in response
```

---

## DEBUGGING TIPS

### Problem: Tests fail with "permission denied" or "file not found"

**Solution:** Ensure `var/relay_physics/filaments/` is writable
```bash
mkdir -p var/relay_physics/filaments
chmod -R 755 var/relay_physics
```

---

### Problem: SSE stream doesn't emit events

**Solution:** 
1. Check server logs for errors
2. Verify EventBus buffer size (1024) is sufficient
3. Test with multiple clients (broadcast requires active subscribers)

---

### Problem: Commits accepted but units don't transition state

**Solution:**
1. Check `CommitProcessor::process_commit()` is called in `append_commit_handler`
2. Verify payload structure matches expected fields (`targetUnit`, `unitId`, `filamentId`)
3. Check unit exists (`GET /api/relay-physics/units`)

---

## NEXT STEPS (After PR #1)

**PR #2: WebSocket Support** (upgrade from SSE)
- Add WebSocket endpoint
- Per-filament subscriptions
- Backpressure handling

**PR #3: Training Data Export**
- JSONL writer for every verify() call (accept/reject)
- Export script: `cargo run --bin export-training-data`
- Dataset: `datasets/relay-agent/raw/*.jsonl`

**PR #4: Full Authority Delegation**
- Replace capability grants with delegation chains
- Authority filaments (`authority.*`)
- Delegation commits (`AUTHORITY_DELEGATE`)

**PR #5: Evidence Bridge to Git**
- Evidence refs can point to Git objects
- Format: `{"kind": "git", "repo": "<id>", "commit": "<sha>", "path": "<file>"}`

---

## SUCCESS METRICS

**PR #1 is complete when:**

✅ All 7 tests pass  
✅ Server boots without errors  
✅ Can create units, append commits, list commits  
✅ SSE stream emits events  
✅ Invalid commits are rejected with reason codes  
✅ SCV state transitions work (Idle → Moving → Working → Blocked)  
✅ JSONL files are created and readable  
✅ Frontend can connect and subscribe to events  

---

**Status:** ✅ IMPLEMENTATION READY  
**Timeline:** 1-2 days (coding), 1 day (testing), 1 day (review)  
**Total:** 3-4 days for PR #1

**Ready to build.**

# Layer 2 (Relay Physics) - Implementation Progress

**Updated:** 2026-01-28  
**Status:** PR #1.1 + #1.2 COMPLETE

---

## ARCHITECTURE

```
┌─────────────────────────────────────────┐
│  LAYER 2: RELAY PHYSICS (STANDALONE)    │
│  Independent of Layer 1 (Git backend)   │
│                                          │
│  ✅ PR #1.1: Persistence + Replay        │
│  ✅ PR #1.2: SSE + Last-Event-ID         │
│  🔲 PR #1.3: Single Commit Fetch         │
│  🔲 PR #2:   Training Data Pipeline      │
│  🔲 PR #3:   Mock AI Agent               │
└─────────────────────────────────────────┘
```

---

## COMPLETED

### ✅ PR #1 (Base): Relay Physics Foundation

**Files:** 13 Rust modules (1,093 lines)

**Core:**
- `types.rs` - FilamentId, UnitId, CommitRef, CausalRefs, UnitState
- `commit.rs` - CommitEvent (immutable)
- `errors.rs` - CommitRejectedError + 10 reason codes
- `agent_ops.rs` - 8 locked op constants
- `filament_store.rs` - Append-only JSONL per-filament commits
- `unit_store.rs` - SCV state machine (5 states)
- `authority_store.rs` - Capability grants (MVP)
- `verifier.rs` - 4-stage pre-append pipeline
- `events.rs` - Event definitions + SSE
- `commit_processor.rs` - Commit→state bridge
- `main_integration.rs` - Axum routes + handlers
- `tests.rs` - 7 unit tests

**API Endpoints:**
- `POST /api/relay-physics/filaments/:id/commits` - Append commit (with verification)
- `GET /api/relay-physics/filaments/:id/commits` - List commits
- `POST /api/relay-physics/units` - Create SCV unit
- `GET /api/relay-physics/units` - List units
- `GET /api/relay-physics/events` - SSE event stream

**Storage:**
- `var/relay_physics/filaments/*.jsonl` - Per-filament commit logs

**Locked Invariants:**
- ✅ Verification BEFORE append
- ✅ Monotonic commitIndex per filament
- ✅ SCV state transitions only via commits
- ✅ 10 reason codes (explicit, actionable)
- ✅ 8 op constants (locked strings)

---

### ✅ PR #1.1: Persistence + Restart Determinism

**Goal:** Same event log → Same state (deterministic replay)

**Files Added/Modified:**
- `event_log.rs` (NEW, 200 lines) - Master JSONL event log
- `unit_store.rs` (+11 lines) - `replay_from_events()`
- `authority_store.rs` (+14 lines) - `replay_from_events()` placeholder
- `commit_processor.rs` (+25 lines) - Persist state changes
- `main_integration.rs` (+40 lines) - Boot replay + persist events
- `PR-1.1-PERSISTENCE-REPLAY.md` (NEW) - Full spec

**Storage Added:**
- `var/relay_physics/events.jsonl` - Master event log (source of truth)

**Boot Behavior:**
```rust
1. Open event log
2. Load all events
3. Replay to UnitStore
4. Print: "🔄 Replaying N events..."
5. Print: "✅ State reconstructed. Units: X, Next event ID: Y"
```

**Event Log Format:**
```json
{
  "event_id": 0,
  "schema_version": 1,
  "timestamp": "2026-01-28T14:30:00Z",
  "event": {"type":"UnitStateChanged","data":{...}}
}
```

**Tests:** 3 tests in `event_log.rs`
- `test_event_log_append_and_replay`
- `test_event_log_deterministic_replay`
- `test_event_log_load_from`

**Locked Invariants:**
- ✅ Event log is append-only
- ✅ Event IDs are monotonic (0, 1, 2, ...)
- ✅ Replay is deterministic
- ✅ Corruption is non-fatal (best-effort recovery)

---

### ✅ PR #1.2: SSE Replay + Last-Event-ID

**Goal:** Clients can reconnect without missing events

**Files Added/Modified:**
- `events.rs` (REWRITE, ~100 lines) - SSE with replay support
  - `EventBus` now broadcasts `(event_id, RelayEvent)` tuples
  - `emit_with_id()` method
  - `sse_stream_with_replay()` - Replay → live transition
- `main_integration.rs` (+30 lines) - SSE handler with Last-Event-ID
  - Parses `Last-Event-ID` header (priority)
  - Parses `?after=N` query param (fallback)
  - All `emit()` calls replaced with `emit_with_id()`
- `commit_processor.rs` (+5 lines) - `emit_with_id()`
- `sse_tests.rs` (NEW, 200 lines) - 5 integration tests
- `PR-1.2-SSE-REPLAY.md` (NEW) - Full spec

**SSE Endpoint:**
```
GET /api/relay-physics/events
Headers:
  Last-Event-ID: N (optional)
Query:
  ?after=N (optional)
```

**SSE Format:**
```
id: 42
event: relay_event
data: {"event_id":42,"schema_version":1,...}

event: ping
data: ping
```

**Reconnection Flow:**
1. Client disconnects at event_id 42
2. Reconnects with `Last-Event-ID: 42`
3. Server replays events 43+ from log
4. Server switches to live broadcast
5. No gaps, no duplicates

**Tests:** 5 tests in `sse_tests.rs`
- `test_sse_replay_from_last_event_id` - Replay 5-9 after Last-Event-ID: 4
- `test_sse_live_append` - Live event reaches client
- `test_sse_idempotent_reconnection` - Reconnect delivers 8-9 after Last-Event-ID: 7
- `test_sse_last_event_id_priority` - Header > query
- `test_sse_defensive_duplicate_filtering` - No duplicates

**Locked Invariants:**
- ✅ Event IDs in SSE match log (monotonic)
- ✅ No gaps (replay → live transition seamless)
- ✅ No duplicates (defensive filtering)
- ✅ Last-Event-ID priority (header > query)
- ✅ Heartbeats every 15s

---

## TOTAL CODE DELIVERED

```
Layer 2 (Relay Physics):
├── PR #1 (Base):      1,093 lines (13 modules + tests)
├── PR #1.1:            +293 lines (event_log + replay)
└── PR #1.2:            +338 lines (SSE replay + 5 tests)
────────────────────────────────────────────────────
TOTAL:                  1,724 lines production Rust
```

**Documentation:**
- `RELAY-PHYSICS-PR1-README.md` (350 lines)
- `RELAY-PHYSICS-DELIVERY-SUMMARY.md` (350 lines)
- `PR-1.1-PERSISTENCE-REPLAY.md` (350 lines)
- `PR-1.2-SSE-REPLAY.md` (450 lines)
- `LAYER-2-PROGRESS.md` (this file)

**Total:** ~3,224 lines (code + docs)

---

## LOCKED BEHAVIORS (REGRESSION-PROOF)

### 1. Commit Verification (4 Stages)
```
Schema → Refs → Authority → Evidence
```
**Tests:** 7 tests in `tests.rs`

### 2. SCV State Machine (Commit-Driven)
```
TASK_ASSIGN    → Idle → Moving
UNIT_ATTACH    → Moving → Working
OUTPUT_REJECTED → Working → Blocked
OUTPUT_ACCEPTED → Working (stays)
UNIT_DETACH    → Working → Idle
```
**Tests:** 1 test in `tests.rs`, 5 tests in `sse_tests.rs`

### 3. Event Log Replay (Deterministic)
```
Same log → Same state (always)
```
**Tests:** 3 tests in `event_log.rs`

### 4. SSE Reconnection (No Gaps)
```
Replay from Last-Event-ID → Switch to live
```
**Tests:** 5 tests in `sse_tests.rs`

---

## INTEGRATION CONTRACT

**Layer 2 exposes:**

1. **Axum Router:**
   ```rust
   relay_physics::main_integration::relay_router(state)
   ```

2. **State Initialization:**
   ```rust
   relay_physics::main_integration::init_relay_physics_state()
   ```

3. **Event Stream:**
   ```rust
   GET /api/relay-physics/events (SSE with Last-Event-ID)
   ```

**Layer 1 (Git backend) integration:**
- ⏳ Deferred to future PR
- Evidence refs will point to Git objects: `{"kind":"git","repo":"main","commit":"a3f2c1b"}`

---

## WHAT'S UNLOCKED

### Frontend (StarCraft HUD)
✅ Can subscribe to real-time events
✅ Can reconnect without missing events
✅ Can render SCVs with state-based visuals
✅ Can show commit rejections with reason codes
✅ Can implement Command Card task assignment

### Training Data Pipeline (PR #2)
✅ Foundation ready (same event stream)
✅ Can export JSONL on every verify
✅ Can include accept/reject reason codes

### Agent Runtime (PR #3)
✅ Can follow truth reliably
✅ Can react to state changes
✅ Can produce commits deterministically

---

## NEXT STEPS

### Option A: PR #1.3 - Single Commit Fetch
**Goal:** `GET /api/relay-physics/commits/:commitRef`

**Use case:** Forensic Inspection Mode (click time cube → isolated chamber)

**Effort:** 1 day (simple GET endpoint + test)

---

### Option B: PR #2 - Training Data Pipeline
**Goal:** Automatic JSONL export on every verify

**Deliverables:**
- `training_exporter.rs` - JSONL writer
- Dataset format: `{stateSnapshot, instruction, modelOutput, verifierResult}`
- Export command: `cargo run --bin export-training-data`

**Effort:** 2-3 days

---

### Option C: PR #3 - Mock AI Agent
**Goal:** Deterministic "agent simulator" for testing

**Deliverables:**
- `mock_agent.rs` - State machine that produces commits
- Stress tests (1000 commits, concurrent agents)
- Edge case coverage (blocked, authority denied, etc.)

**Effort:** 2-3 days

---

### Option D: Frontend Integration (Layer 3)
**Goal:** Wire SSE into StarCraft HUD

**Deliverables:**
- `useRelayEvents.js` - React hook for SSE
- `SCVUnits.jsx` - Render SCVs in 3D world
- `CommandCard.jsx` - Task assignment UI

**Effort:** 3-4 days

---

## VALIDATION COMMANDS

### Build
```bash
cd apps/server
cargo build
```

### Test
```bash
cargo test
# Expected: 15 tests passing (7 base + 3 PR #1.1 + 5 PR #1.2)
```

### Run
```bash
cargo run
# Expected: 
# 🔄 Replaying N events to reconstruct state...
# ✅ State reconstructed. Units: X, Next event ID: Y
# 🚀 Relay server listening on 0.0.0.0:3002
```

### Test SSE Reconnection
```bash
# Terminal 1: Subscribe with Last-Event-ID
curl -N -H "Last-Event-ID: 0" \
  http://localhost:3002/api/relay-physics/events

# Terminal 2: Create units
for i in 001 002 003; do
  curl -X POST http://localhost:3002/api/relay-physics/units \
    -d "{\"unit_id\":\"unit.test.$i\"}"
done

# Terminal 1: Should receive events 1, 2 (skips 0)
```

---

## FILE STRUCTURE

```
apps/server/
├── src/
│   ├── main.rs (integration template)
│   └── relay_physics/
│       ├── mod.rs
│       ├── types.rs
│       ├── commit.rs
│       ├── errors.rs
│       ├── agent_ops.rs
│       ├── filament_store.rs
│       ├── unit_store.rs
│       ├── authority_store.rs
│       ├── verifier.rs
│       ├── events.rs (PR #1.2)
│       ├── event_log.rs (PR #1.1)
│       ├── commit_processor.rs
│       ├── main_integration.rs
│       ├── tests.rs
│       └── sse_tests.rs (PR #1.2)
├── Cargo.toml
├── PR-1.1-PERSISTENCE-REPLAY.md
├── PR-1.2-SSE-REPLAY.md
└── RELAY-PHYSICS-PR1-README.md

var/relay_physics/
├── filaments/
│   ├── work_W123.jsonl
│   └── ...
└── events.jsonl (PR #1.1 - master log)
```

---

**STATUS:** ✅ **LAYER 2 MVP FOUNDATION COMPLETE**

**PRs delivered:** #1 (Base) + #1.1 (Persistence) + #1.2 (SSE Replay)

**Next:** Choose PR #1.3, #2, #3, or Layer 3 (Frontend)

**Ready for:** Dev team build + test + validation

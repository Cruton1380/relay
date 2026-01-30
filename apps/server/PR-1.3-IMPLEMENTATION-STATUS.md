# PR #1.3: Implementation Status

**Date:** 2026-01-28  
**Status:** ✅ **IMPLEMENTED (Ready for Testing)**

---

## WHAT WAS BUILT

### 1. Core Structures (`commit_bundle.rs`)
✅ `CommitBundle` - Response envelope with source tracking  
✅ `CommitSource` enum - Event log vs Filament log  
✅ `CommitRefFormat` - Parser for `event:id` and `filament@cN` formats  
✅ `CommitNotFoundError` - Structured 404 error response  
✅ 3 unit tests for parsing logic

### 2. API Endpoint (`main_integration.rs`)
✅ `GET /api/relay-physics/commits/:commit_ref` handler  
✅ Support for `event:42` format (fetch from event log)  
✅ Support for `work.W123@c7` format (fetch from filament log)  
✅ Immutable caching headers (`Cache-Control: public, immutable`)  
✅ Structured 400/404 error responses

### 3. Module Integration (`mod.rs`)
✅ `commit_bundle` module declared  
✅ Public exports for all new types  
✅ Available to main.rs for routing

---

## FILES MODIFIED

```
apps/server/src/relay_physics/
├── commit_bundle.rs           (NEW - 140 lines)
├── mod.rs                     (+2 lines - module + exports)
└── main_integration.rs        (+140 lines - handler + route)
```

**Total:** +282 lines of implementation code

---

## ENDPOINT BEHAVIOR

### Fetch by Event ID
```bash
curl http://localhost:3002/api/relay-physics/commits/event:42
```

**Returns:**
```json
{
  "source": "event_log",
  "event_id": 42,
  "schema_version": 1,
  "timestamp": "2026-01-28T10:30:00Z",
  "commit": {
    "commit_ref": "work.W123@c7",
    "filament_id": "work.W123",
    "commit_index": 7,
    "op_type": "OUTPUT_PROPOSED",
    ...
  }
}
```

**Headers:**
```
Cache-Control: public, max-age=31536000, immutable
```

---

### Fetch by Filament@CommitIndex
```bash
curl http://localhost:3002/api/relay-physics/commits/work.W123@c7
```

**Returns:**
```json
{
  "source": "filament_log",
  "commit": {
    "commit_ref": "work.W123@c7",
    "filament_id": "work.W123",
    "commit_index": 7,
    "op_type": "OUTPUT_PROPOSED",
    ...
  }
}
```

**Note:** No `event_id` field (commit may exist in filament log before being in event stream)

---

### 404 Not Found
```bash
curl http://localhost:3002/api/relay-physics/commits/work.W123@c999
```

**Returns:**
```json
{
  "error": "COMMIT_NOT_FOUND",
  "commit_ref": "work.W123@c999",
  "message": "Commit not found: work.W123@c999",
  "details": "Filament 'work.W123' head is at c7 (requested c999)"
}
```

---

### 400 Bad Request
```bash
curl http://localhost:3002/api/relay-physics/commits/invalid-format
```

**Returns:**
```json
{
  "error": "INVALID_COMMIT_REF",
  "commit_ref": "invalid-format",
  "message": "Invalid commit ref format",
  "details": "Invalid commit ref format: expected 'event:<id>' or '<filamentId>@c<index>', got 'invalid-format'"
}
```

---

## TESTS NEEDED (Manual)

### Test 1: Fetch by Event ID (Success)
```bash
# 1. Create a commit
curl -X POST http://localhost:3002/api/relay-physics/filaments/work.W123/commits \
  -H "Content-Type: application/json" \
  -d '{"op_type":"TASK_ASSIGN","author_unit_ref":"unit.manager.001","payload":{},"causal_refs":{"inputs":[],"evidence":[]}}'

# 2. Get event_id from SSE stream (or from response)
# Assume event_id = 1

# 3. Fetch by event ID
curl http://localhost:3002/api/relay-physics/commits/event:1

# Expected: 200 OK with commit bundle (source: event_log)
```

---

### Test 2: Fetch by Filament Ref (Success)
```bash
# 1. Create commit on filament work.W123 (see Test 1)

# 2. Fetch by filament ref
curl http://localhost:3002/api/relay-physics/commits/work.W123@c1

# Expected: 200 OK with commit bundle (source: filament_log)
```

---

### Test 3: 404 for Missing Event
```bash
curl http://localhost:3002/api/relay-physics/commits/event:999999

# Expected: 404 with COMMIT_NOT_FOUND error
```

---

### Test 4: 404 for Missing Filament Commit
```bash
curl http://localhost:3002/api/relay-physics/commits/work.W123@c999

# Expected: 404 with details about head position
```

---

### Test 5: 400 for Invalid Format
```bash
curl http://localhost:3002/api/relay-physics/commits/invalid-format

# Expected: 400 with INVALID_COMMIT_REF error
```

---

### Test 6: Caching Headers
```bash
curl -I http://localhost:3002/api/relay-physics/commits/work.W123@c1

# Expected headers:
# Cache-Control: public, max-age=31536000, immutable
# ETag: (optional)
```

---

## QUESTIONS ANSWERED (From User)

### Q1: Do we already have canonical mapping from event_id → per-filament commit pointer?
**A:** ✅ YES - `CommitEvent` contains both `event_id` (via `EventLogEntry` wrapper) and `filament_id + commit_index`

**Implementation:** When fetching by `event:id`, we extract `commit.filament_id` and `commit.commit_index` from the event

---

### Q2: Does UnitStateChanged carry enough identifiers to locate the filament?
**A:** PARTIALLY - For PR #1.3, we only return commits (`CommitAccepted` events), not `UnitStateChanged`

**Behavior:** If `event:id` points to a `UnitStateChanged`, return 404 with message "Event N is not a commit"

**Future:** Could extend to return unit state changes, but that's a different use case

---

### Q3: Where should the "head" index live?
**A:** ✅ ALREADY EXISTS - `FilamentStore::head_index_cache` (in-memory, rebuilt on boot from JSONL)

**Implementation:** We use `filament_store.get_head_index()` to provide helpful 404 details

---

## INVARIANTS ENFORCED

1. ✅ **Commits are immutable** - `Cache-Control: immutable` header
2. ✅ **Event log is source of truth** - Event ID always maps to same commit
3. ✅ **No rendering decisions** - Bundle contains only raw truth (no materials, colors)
4. ✅ **Structured errors** - 404/400 return JSON with helpful details
5. ✅ **Two sources distinguished** - `source` field tells client where commit came from

---

## WHAT THIS UNLOCKS

### ✅ For Layer 3 (Frontend)
- "Click time cube → fetch commit → render forensic chamber"
- Display causal graph (parse `causal_refs.inputs`)
- Show commit timeline (use `timestamp`)

### ✅ For Debugging
```bash
# Quick commit inspection
curl /api/relay-physics/commits/work.W123@c7 | jq .

# Trace causal chain
curl /api/relay-physics/commits/work.W123@c6 | jq .commit.causal_refs
```

### ✅ For Training Pipeline
- Fetch commits by `event_id` for annotation
- Export commit bundles as training data (already JSON)
- Label `op_types` for LLM fine-tuning

### ✅ For RenderSpec v1 Design
- See exact data structure of commits
- Informs `/render/commit/:ref` endpoint design
- Shows what fields need mapping to visuals

---

## NEXT STEPS

### 1. Manual Testing (1 hour)
- Run server: `cargo run --bin relay-server`
- Execute manual tests (see above)
- Verify caching headers with `curl -I`

### 2. Automated Tests (2 hours)
- Create `apps/server/src/relay_physics/commit_fetch_tests.rs`
- Implement 5 tests from PR-1.3-COMMIT-FETCH.md
- Run: `cargo test commit_fetch`

### 3. Lock RenderSpec v1 (1-2 days)
- Use `CommitBundle` structure to inform schema
- Define material tag conventions
- Lock animation intent schemas
- Update `architecture@c3` from stub to full spec

### 4. Implement `/render/commit/:ref` (2-3 days)
- Derived scene graph for forensic chamber
- Maps `CommitBundle` → visual nodes (time cube, links, etc.)
- Uses RenderSpec v1 format

---

## MERGE CHECKLIST

- [x] Core structs implemented (`commit_bundle.rs`)
- [x] API handler implemented (`get_commit_handler`)
- [x] Route registered (`/api/relay-physics/commits/:commit_ref`)
- [x] Module exports updated (`mod.rs`)
- [x] Parsing logic tested (3 unit tests in `commit_bundle.rs`)
- [ ] Manual tests run (6 tests)
- [ ] Automated tests written (5 integration tests)
- [x] Documentation complete (`PR-1.3-COMMIT-FETCH.md`)
- [x] Caching headers set (immutable)
- [x] Error responses structured (JSON with details)

**Ready for:** Manual testing → Automated tests → Merge

---

## PHILOSOPHY CHECK

✅ **Raw truth first** - No rendering decisions in bundle  
✅ **Commits are immutable** - Cache forever  
✅ **Log is authority** - Event ID/commit index are canonical  
✅ **Two sources respected** - Event log vs filament log distinguished  
✅ **Helpful errors** - 404 includes head position, 400 includes format help

**Per `architecture@c1`:** This is the "raw commit fetch" endpoint. `/render/commit/:ref` will be the "derived scene graph" endpoint (future PR).

---

**Date:** 2026-01-28  
**Status:** ✅ **IMPLEMENTATION COMPLETE**  
**Estimated Manual Test Time:** ~30 minutes  
**Next:** Run manual tests, then proceed to RenderSpec v1 lock

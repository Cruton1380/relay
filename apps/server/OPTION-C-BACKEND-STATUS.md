# Option C - Backend First: Implementation Status

**Last Updated:** 2026-01-28  
**Reference:** `architecture@c5` (RenderSpec v1 Locked), `architecture@c9` (HUD/Globe Model)

---

## ✅ COMPLETED MILESTONES

### PR #1.1: Deterministic Replay ✅
- Event log with JSONL persistence
- Replay reconstructs identical state
- Monotonic event IDs

### PR #1.2: SSE with Replay Support ✅
- `Last-Event-ID` header support
- Replay-then-stream pattern
- Configurable buffer capacity

### PR #1.3: Single Commit Fetch ✅
- Forensic inspection endpoint
- Support for event ID and filament@commit formats
- Error handling for invalid refs

### PR #5: RenderSpec v1 Locked ✅
- Schema defined in `renderspec.rs`
- Generator in `renderspec_generator.rs`
- 3 render endpoints implemented

### PR #6: Buildings Store ✅ (NEW)
- Building types (vendor, partnership, civic, logistics, community)
- Geographic anchors (lat/lng/altitude)
- Catalog data (products with production/delivery times)
- Event sourcing (`BUILDING_REGISTERED`, `BUILDING_UPDATED`)
- Seed file with 3 demo buildings
- REST API endpoints (`GET /buildings`, `POST /buildings`)
- RenderSpec output (building nodes)
- Deterministic replay support

---

## 📋 RENDER ENDPOINTS (OPTION C)

### 1. GET /api/relay-physics/render/world
**Purpose:** Full world scene (globe + units + filaments + buildings)

**Status:** ✅ IMPLEMENTED + UPDATED (PR #6)

**Output:** RenderSpec v1 with building nodes

**Cache:** `Cache-Control: private, max-age=60`

**Updated in PR #6:**
- Now includes building nodes from `BuildingStore`
- Buildings have `geo_anchor` positions
- Catalog data in `props`

---

### 2. GET /api/relay-physics/render/commit/:ref
**Purpose:** Forensic chamber scene (single commit inspection)

**Status:** ✅ IMPLEMENTED

**Ref formats:**
- Event ID: `/render/commit/42`
- Filament commit: `/render/commit/work.W123@c5`

**Output:** RenderSpec v1 with timebox node

**Cache:** `Cache-Control: private, max-age=300`

---

### 3. GET /api/relay-physics/render/filament/:id
**Purpose:** Filament thread scene (polyline + timeboxes)

**Status:** ✅ IMPLEMENTED

**Example:** `/render/filament/work.W123`

**Output:** RenderSpec v1 with filament polyline + commit timeboxes

**Cache:** `Cache-Control: private, max-age=60`

---

## 🏗️ BUILT COMPONENTS

### Core Files (PR #1-5)
- ✅ `renderspec.rs` - RenderSpec v1 data structures
- ✅ `renderspec_generator.rs` - Scene generation logic
- ✅ `main_integration.rs` - Axum router + handlers

### New Files (PR #6)
- ✅ `types.rs` - Building types (BuildingId, BuildingType, GeoAnchor, CatalogItem, Building)
- ✅ `building_store.rs` - Building storage + operations
- ✅ `building_seeder.rs` - Seed loader
- ✅ `events.rs` - Building events (BuildingRegistered, BuildingUpdated)
- ✅ `relay/fixtures/buildings_seed.json` - Demo world (3 buildings)

---

## 🔒 DETERMINISM ENFORCEMENT

### Locked Rules
1. ✅ No randomness in position generation (hash-based or geo-based)
2. ✅ No `Date.now()` in RenderSpec generation
3. ✅ Deterministic commit ordering (by commitIndex)
4. ✅ Deterministic filament polyline construction
5. ✅ Deterministic building positions (from geo_anchor)
6. ✅ Deterministic catalog ordering (sorted by product_id)

**Key property:** Same truth state → Same RenderSpec → Same visual output

---

## 🧪 MANUAL TESTING

### Test 1: World Scene with Buildings
```bash
curl http://localhost:3002/api/relay-physics/render/world | jq .
```

**Expected output:**
```json
{
  "schema_version": "relay-render-v1",
  "generated_from": {...},
  "nodes": [
    {"kind": "globe", ...},
    {"kind": "building", "id": "building.apple_store.nyc_001", ...},
    {"kind": "building", "id": "building.acme_hq.sf", ...},
    {"kind": "building", "id": "building.drone_depot.lax", ...},
    {"kind": "unit", ...}
  ]
}
```

### Test 2: List Buildings
```bash
curl http://localhost:3002/api/relay-physics/buildings | jq .
```

**Expected:** Array of 3 buildings with catalog data

### Test 3: Register Building
```bash
curl -X POST http://localhost:3002/api/relay-physics/buildings \
  -H "Content-Type: application/json" \
  -d '{
    "building_id": "building.test.001",
    "building_type": "vendor",
    "geo_anchor": {"lat": 34.0522, "lng": -118.2437, "altitude": 10.0},
    "catalog": [],
    "status": "active",
    "props": {}
  }'
```

**Expected:** `BuildingRegistered` event + 201 Created

### Test 4: Commit Scene
```bash
# First, append a commit to get an ID
curl -X POST http://localhost:3002/api/relay-physics/filaments/work.W123/commits \
  -H "Content-Type: application/json" \
  -d '{
    "op_type": "TASK_ASSIGNED",
    "payload": {"task": "test"},
    "author_unit_ref": "unit.manager.001",
    "causal_refs": {"inputs": [], "evidence": []}
  }'

# Then render it
curl http://localhost:3002/api/relay-physics/render/commit/work.W123@c0 | jq .
```

**Expected:** Timebox node for that commit

### Test 5: Filament Scene
```bash
curl http://localhost:3002/api/relay-physics/render/filament/work.W123 | jq .
```

**Expected:** Filament polyline + timeboxes for all commits

---

## 🎯 NEXT ENHANCEMENTS

### Immediate (After PR #6 Tests Pass)
- [ ] **PR #7: Task Store** - Build queue (shopping as production)
  - Task nodes in RenderSpec
  - Production queue visualization
  - Link tasks to buildings

- [ ] **PR #8: Shipment Store** - Drones in flight
  - Shipment nodes in RenderSpec
  - Route interpolation
  - Animation intents for movement

### Medium-Term
- [ ] Causal link visualization (inputs → outputs)
- [ ] Authority chain rendering (delegation paths)
- [ ] Encrypted filament material distinction
- [ ] Timebox verification state (accepted/rejected/disputed)

### Long-Term (Frontend Integration)
- [ ] React components for building panels
- [ ] Three.js renderers for buildings/shipments
- [ ] Task bar (build queue) UI
- [ ] HUD gauges (5 coordination resources)

---

## 📚 DOCUMENTATION

### Implementation Specs
- `apps/server/PR-6-BUILDINGS-STORE.md` - PR #6 detailed spec
- `relay/PR-6-COMPLETION-REPORT.md` - Implementation results

### Architecture References
- `relay/filaments/architecture/0005_renderspec_v1_locked.md` - RenderSpec schema
- `relay/filaments/architecture/0009_personal_hud_physical_globe.md` - HUD/Globe model

### Vision Documents
- `relay/RELAY-BUILDINGS-EXPLAINED-FOR-AI.md` - Full explanation for external AIs
- `NEXT-SESSION-QUICKSTART-PR6.md` - Session resumption guide

---

## 🎮 THE STARCRAFT MODEL

> "StarCraft wasn't 'a game with economics.' It was an economic operating system disguised as a war game."

**Relay implements the StarCraft model for coordination:**

| StarCraft | Relay | Purpose |
|-----------|-------|---------|
| Barracks | Vendor building | Produces units |
| Production queue | Task bar | Visible commitments |
| Minerals/gas | Coordination gauges | Legible constraints |
| Supply cap | Commitment capacity | Bounded ambition |
| Units | Tasks/shipments | Embodied work |
| Minimap | 3D globe | Shared world |
| HUD | Personal HUD | My state |

**Key insight:** Combat was camouflage. Coordination was the real mechanic.

---

## 🔒 LOCKED INVARIANTS (CUMULATIVE)

**From c0-c5:** 30 invariants (filaments, commits, units, RenderSpec)  
**From c6-c8:** 12 invariants (economic primitives, gauges, DDI)  
**From c9/PR#6:** 12 invariants (buildings, catalog, determinism)

**Total:** 54 locked invariants

---

## ✅ SUCCESS CRITERIA

| Criterion | Status |
|-----------|--------|
| RenderSpec v1 schema defined | ✅ |
| 3 render endpoints implemented | ✅ |
| Deterministic position generation | ✅ |
| No randomness in output | ✅ |
| Manual tests documented | ✅ |
| Buildings as world entities | ✅ |
| Event sourcing for buildings | ✅ |
| Seed file with demo world | ✅ |
| API endpoints for buildings | ✅ |

**Overall Status:** ✅ OPTION C - BACKEND COMPLETE (PR #1-6)

---

## 🚀 NEXT MOVE

**After PR #6 manual tests pass:**

Say: `"PR #6 tests pass — start PR #7"`

**We'll then implement:**
- Task Store (build queue)
- Shopping as unit production
- Task progression states
- Link to buildings

**Estimate:** 4-6 hours

---

**Status:** ✅ READY FOR TESTING  
**Blockers:** None  
**Waiting on:** User to run 5 manual tests

---

**END OF STATUS REPORT**

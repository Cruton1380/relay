# PR #6 Buildings Store - Session Quickstart

**Date:** 2026-01-28  
**Status:** ✅ IMPLEMENTATION COMPLETE - AWAITING TESTING  
**Next:** Run manual tests, then proceed to PR #7

---

## ⚡ 30-SECOND SUMMARY

**What we built:** Buildings as physical world entities on globe (vendor, partnership, civic, logistics, community) with catalog data, geographic coordinates, event sourcing, and RenderSpec v1 output.

**Why it matters:** Buildings anchor filaments to reality. They're coordination surfaces where production/shipment/verification filaments originate and resolve.

**Status:** Code complete, tests written, ready for manual testing.

---

## 📁 FILES TO REVIEW

### New Files (8)
```
apps/server/src/relay_physics/
├── building_store.rs           ✅ Building storage + operations
├── building_seeder.rs          ✅ Seed loader

relay/fixtures/
└── buildings_seed.json         ✅ Demo world (3 buildings)

relay/
├── PR-6-COMPLETION-REPORT.md   📄 Full implementation report
└── RELAY-BUILDINGS-EXPLAINED-FOR-AI.md  📄 Vision + philosophy

apps/server/
└── PR-6-BUILDINGS-STORE.md     📄 Implementation spec
```

### Modified Files (5)
```
apps/server/src/relay_physics/
├── types.rs                    ✅ Added building types
├── events.rs                   ✅ Added building events
├── mod.rs                      ✅ Added building modules
├── renderspec_generator.rs     ✅ Added create_building_node()
└── main_integration.rs         ✅ Added building store + endpoints
```

---

## 🧪 MANUAL TESTS (RUN THESE NEXT)

### Start Server
```bash
cd apps/server
cargo run
```

### Test 1: List Buildings
```bash
curl http://localhost:3002/api/relay-physics/buildings | jq .
```
**Expected:** 3 buildings (Apple Store, ACME HQ, Drone Hub)

### Test 2: Render World with Buildings
```bash
curl http://localhost:3002/api/relay-physics/render/world | jq '.nodes[] | select(.kind=="building")'
```
**Expected:** Building nodes with geo_anchor, catalog, etc.

### Test 3: Register New Building
```bash
curl -X POST http://localhost:3002/api/relay-physics/buildings \
  -H "Content-Type: application/json" \
  -d '{
    "building_id": "building.test.001",
    "building_type": "vendor",
    "geo_anchor": {"lat": 34.0522, "lng": -118.2437, "altitude": 10.0},
    "owner_ref": "unit.test.admin",
    "catalog": [],
    "status": "active",
    "props": {}
  }'
```
**Expected:** `BuildingRegistered` event

### Test 4: Verify Deterministic Replay
1. Stop server
2. Restart server
3. List buildings again
4. **Expected:** Same 3 buildings (replayed from `events.jsonl`)

---

## 🔑 KEY DECISIONS LOCKED

### 1. Buildings = Spatial Anchors for Filaments
> "Buildings are the spatial anchors where filaments attach to reality: a vendor building anchors production filaments, a logistics hub anchors shipment filaments, a civic building anchors verification and jury filaments."

### 2. Catalog in Props (Not Separate Nodes)
- Catalog items are NOT world objects (no transform)
- Catalog stays in `building.props.catalog[]`
- Products are UI data, not spatial entities

### 3. StarCraft Model
> "StarCraft wasn't 'a game with economics.' It was an economic operating system disguised as a war game. The combat was camouflage."

**Relay removes the violence, keeps the physics.**

### 4. HUD = Personal, Globe = Shared
- **HUD:** My resources, my tasks, my units
- **Globe:** Physical world, buildings, shipments
- **Never mix them.**

---

## ❓ OPEN QUESTIONS FOR USER

### Q1: Building Authority Model
Should building ownership be:
- A. Direct ownership (`owner_ref: UnitId`) ← **CURRENT**
- B. Authority delegation (PR #4/5)
- C. Multi-sig governance (PR #5)

**Recommendation:** Start with A, migrate to B.

### Q2: Building Discovery
How should users find buildings?
- A. Spatial search
- B. Type filter
- C. Semantic search
- D. All of above

**Recommendation:** D, but defer to frontend.

### Q3: Building Verification
How verify building legitimacy?
- A. Authority delegation
- B. Civic verification (juries)
- C. Stake requirement (escrow)
- D. Combination

**Recommendation:** D, but this is PR #9+ concern.

---

## 🚀 NEXT STEPS

### After PR #6 Tests Pass
**Proceed to PR #7: Task Store** (build queue)

**Deliverables:**
- `TaskStore` with `TASK_CREATE`, `TASK_PROGRESS`, `TASK_COMPLETE` events
- Task nodes in RenderSpec
- Link tasks to buildings (production source)
- Task progression states

**Estimate:** 4-6 hours

### Then PR #8: Shipment Store (drones)

**Deliverables:**
- `ShipmentStore` with position tracking
- Shipment nodes in RenderSpec
- Route interpolation
- Animation intents

**Estimate:** 6-8 hours

---

## 📚 READ THESE DOCUMENTS

### For Technical Context
- `apps/server/PR-6-BUILDINGS-STORE.md` - Implementation spec
- `relay/PR-6-COMPLETION-REPORT.md` - What we built

### For Vision/Philosophy
- `relay/RELAY-BUILDINGS-EXPLAINED-FOR-AI.md` - Full explanation of buildings + filaments
- `relay/filaments/architecture/0009_personal_hud_physical_globe.md` - HUD/Globe model

### For External AI (ChatGPT, Claude)
- `relay/RELAY-BUILDINGS-EXPLAINED-FOR-AI.md` - Share this with other AIs

---

## 🎯 BOTTOM LINE

**PR #6 is code-complete.**

Buildings are spatial anchors. Filaments are temporal evolution. Together they form the space-time fabric of coordination.

**Run the 4 manual tests. If they pass, PR #6 is locked and we proceed to PR #7.**

---

**Status:** ✅ READY FOR TESTING  
**Blockers:** None  
**Next move:** User runs manual tests

---

## 🔖 QUICK COMMANDS

```bash
# Start server
cd apps/server && cargo run

# Test buildings list
curl http://localhost:3002/api/relay-physics/buildings | jq .

# Test render world
curl http://localhost:3002/api/relay-physics/render/world | jq '.nodes[] | select(.kind=="building")'

# Register building
curl -X POST http://localhost:3002/api/relay-physics/buildings \
  -H "Content-Type: application/json" \
  -d '{"building_id": "building.test.001", "building_type": "vendor", "geo_anchor": {"lat": 34.0522, "lng": -118.2437}, "catalog": [], "status": "active", "props": {}}'
```

---

**When tests pass, say: `"PR #6 tests pass — start PR #7"`**

---

**END OF QUICKSTART**

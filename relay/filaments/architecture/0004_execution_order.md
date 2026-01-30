# COMMIT D: Execution Order Decision

**Filament:** `architecture`  
**Commit Index:** 3  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** ROADMAP_LOCK

---

## DECISION LOCKED

**Execute in order: PR #1.3 (commit fetch) → RenderSpec v1 (schema lock) → /render/* endpoints → Layer 3 frontend.**

---

## RATIONALE

### Why PR #1.3 First (Single Commit Fetch)

**Endpoint:** `GET /api/relay-physics/commits/:commitRef`

**Effort:** ~4 hours (small, surgical)

**Why now:**
1. **Completes Layer 2 "truth data" API** (events + commits + units)
2. **Immediately useful** for debugging and forensics
3. **Informs RenderSpec design** - shows exactly what data Layer 3 needs
4. **Natural dependency** - `/render/commit/:ref` will use this internally

**What it unlocks:**
- "Click time cube → open forensic chamber" (Layer 3 UX)
- Debugging commit rejections (full causal graph visible)
- Training data annotation (commit → LLM fine-tuning)
- Audit trails (immutable commit history)

**Status:** ⏭️ NEXT (immediately after architecture filament commits)

---

### Why RenderSpec v1 Second (Schema Lock)

**Effort:** ~1-2 days (design + documentation)

**Why after PR #1.3:**
- PR #1.3 shows **real data shapes** (commits, units, filaments)
- Better informed schema design (not speculative)
- Can validate schema against actual commit fetch responses

**What it locks:**
- JSON format for scene graphs (nodes, transforms, materials, animation intents)
- Material tag conventions (semantic, not artistic)
- Animation intent schemas
- Extensibility rules (forward compatibility)

**What it unlocks:**
- Layer 2 → Layer 3 contract is stable
- Multiple frontends can implement independently
- Cinematic pipeline can consume same format

**Status:** ⏭️ AFTER PR #1.3

---

### Why /render/* Endpoints Third

**Endpoints:**
- `GET /api/relay-physics/render/world` - Complete scene graph
- `GET /api/relay-physics/render/commit/:ref` - Forensic chamber scene
- `GET /api/relay-physics/render/filament/:id` - Single filament geometry

**Effort:** ~3-4 days (depends on RenderSpec complexity)

**Why after RenderSpec v1:**
- Schema is locked (no guessing)
- Can implement against stable contract
- Tests validate against schema

**What it unlocks:**
- Layer 2 "render-ready data" API is complete
- Layer 3 can start building against real endpoints (not mocks)

**Status:** ⏭️ AFTER RenderSpec v1

---

### Why Layer 3 Frontend Last

**Technology:** React + Three.js (react-three-fiber)

**Effort:** ~1-2 weeks (iterative, visual experimentation)

**Why last:**
- All Layer 2 APIs are stable (no moving targets)
- Can focus purely on rendering/interaction (not backend fixes)
- Fast iteration (JavaScript, not Rust)

**What it delivers:**
- StarCraft-style HUD (globe + SCVs + filaments)
- Real-time event stream visualization (SSE → animations)
- Forensic inspection mode (click cube → chamber)
- User interaction (camera control, unit selection)

**Status:** ⏭️ AFTER /render/* endpoints

---

## ALTERNATIVE CONSIDERED (REJECTED)

### RenderSpec v1 First, Then PR #1.3

**Why rejected:**
- **Speculative design** - Don't know exact data needs yet
- **Risk of rework** - Schema might not match reality
- **Delays forensic capability** - PR #1.3 is useful NOW

**Better to:** Let PR #1.3 inform RenderSpec (data-driven design)

---

### Layer 3 Frontend Before RenderSpec

**Why rejected:**
- **No stable contract** - Layer 2→3 interface changes mid-development
- **Tight coupling** - Frontend makes assumptions about Layer 2 data shapes
- **Harder to support multiple frontends** - Each frontend would diverge

**Better to:** Lock schema first, then all frontends implement to same spec

---

## EXECUTION TIMELINE (ESTIMATED)

```
Week 1:
  Day 1: ✅ Architecture filament commits (this document)
  Day 1-2: PR #1.3 (commit fetch endpoint)
  
Week 2:
  Day 3-5: RenderSpec v1 schema design + documentation
  
Week 3-4:
  Day 6-12: /render/* endpoints implementation
  
Week 5-6:
  Day 13-26: Layer 3 frontend (React + Three.js)
    - SSE subscription hook
    - Scene graph renderer (Three.js)
    - Camera controls
    - Unit selection/interaction
    - Forensic chamber mode
```

**Total:** ~6 weeks to full visual demo

**Milestones:**
- ✅ Week 1: Layer 2 "data API" complete (events + commits + units)
- ⏭️ Week 2: Layer 2→3 contract locked (RenderSpec v1)
- ⏭️ Week 4: Layer 2 "render API" complete (/render/* endpoints)
- ⏭️ Week 6: Layer 3 visual demo (StarCraft HUD)

---

## PARALLEL WORK OPPORTUNITIES

While one person works on PR #1.3 → RenderSpec → /render/* (backend-focused), another can:

1. **Design visual language** (not blocked by backend)
   - Filament glow rules
   - Time cube materials
   - SCV animations
   - Color palettes

2. **Prototype Layer 3 with mock data** (parallel to RenderSpec)
   - React + Three.js setup
   - Camera controls
   - Basic scene rendering
   - Replace mocks when /render/* endpoints are ready

3. **Training pipeline** (PR #2)
   - Event log → JSONL export
   - LLM fine-tuning format
   - Annotation tooling

---

## LOCKED INVARIANTS

1. **PR #1.3 comes before RenderSpec v1** (data informs schema)
2. **RenderSpec v1 comes before /render/* endpoints** (schema drives implementation)
3. **All Layer 2 APIs stable before Layer 3 starts** (no moving targets)
4. **Visual language can be designed in parallel** (not blocked by backend)

---

## CAUSAL REFS

- **Inputs:**
  - `architecture@c0` (Layer split)
  - `architecture@c1` (Render endpoints)
  - `architecture@c2` (SSE locks)
- **Authority:** system.architect
- **Evidence:**
  - PR #1.1, #1.2 complete (Layer 2 event stream works)
  - RenderSpec stub created (`architecture@c3`)

---

**Status:** LOCKED  
**Next:** Implement PR #1.3 (commit fetch endpoint)  
**Rationale:** Small, high-value, informs all downstream work.

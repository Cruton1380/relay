# Next Session Quick Start

**Last Updated:** 2026-01-28  
**Status:** Layer 2 ~70% complete, ready for `/render/*` or Layer 3

---

## CURRENT STATE (WHERE WE LEFT OFF)

### ✅ COMPLETED
1. **PR #1.2** - SSE truth stream (with Q1-Q5 audit fixes)
2. **PR #1.3** - Single commit fetch (`GET /commits/:ref`)
3. **Architecture Filament** - 6 commits locked (c0-c5)
4. **RenderSpec v1** - Complete Layer 2→3 contract locked

### ⏭️ NEXT
- **Option A:** Implement `/render/*` endpoints (backend)
- **Option B:** Start Layer 3 frontend (Three.js)
- **Option C:** Both in parallel (recommended)

---

## DECISION POINT

**You need to choose:** A, B, or C

### Option A: Backend First (~5 days)
Implement Layer 2 render endpoints

**Pros:**
- Complete Layer 2 API surface
- Backend-driven development
- Can test independently

**Cons:**
- No visual feedback until Layer 3 built
- ~1 week before visuals

**Tasks:**
1. Implement `GET /render/world`
2. Implement `GET /render/commit/:ref`
3. Implement `GET /render/filament/:id`

---

### Option B: Frontend First (~1-2 weeks)
Build Three.js renderer with mock data

**Pros:**
- Visual feedback immediately
- Fast iteration on visual language
- Can demo to stakeholders

**Cons:**
- Uses mock data until `/render/*` ready
- May need adjustments when real API arrives

**Tasks:**
1. Setup React + Three.js
2. Build RenderSpec v1 renderer
3. Subscribe to SSE stream
4. Implement StarCraft HUD

---

### Option C: Parallel (~1 week to convergence) **← RECOMMENDED**
Split work: backend team implements `/render/*`, frontend team builds renderer

**Pros:**
- Fastest to visual demo
- Both teams productive
- Converge when `/render/world` ready

**Cons:**
- Requires coordination
- Need 2+ people

**Tasks:**
- **Backend:** Implement `/render/world`
- **Frontend:** Build renderer against fixtures

---

## KEY FILES TO READ FIRST

### If Starting Backend (Option A/C)
```
1. relay/RENDERSPEC-V1-LOCKED.md (summary)
2. relay/filaments/architecture/0005_renderspec_v1_locked.md (full spec)
3. relay/fixtures/renderspec_v1_canonical.json (example)
4. apps/server/PR-1.3-IMPLEMENTATION-STATUS.md (reference)
```

### If Starting Frontend (Option B/C)
```
1. relay/RENDERSPEC-V1-LOCKED.md (summary + Three.js guide)
2. relay/fixtures/renderspec_v1_canonical.json (canonical example)
3. relay/fixtures/renderspec_v1_minimal.json (minimal valid scene)
4. apps/server/PR-1.2-COMPLETE.md (SSE subscription guide)
```

### For Architecture Context
```
1. relay/SESSION-SUMMARY-2026-01-28.md (what was built)
2. relay/filaments/architecture/README.md (decision index)
3. relay/filaments/architecture/0000_arch_split.md (Layer 2→3 separation)
```

---

## QUICK COMMANDS

### Test Current State
```bash
# Build
cd apps/server
cargo build

# Run server
cargo run

# Test PR #1.3 (commit fetch)
curl http://localhost:3002/api/relay-physics/commits/event:1

# Test SSE stream
curl -N http://localhost:3002/api/relay-physics/events
```

---

### Git Workflow (Commit Today's Work)
```bash
# 1. Commit architecture filament
git add relay/filaments/
git commit -m "ARCH: Create architecture filament (commits 0-5)"

# 2. Commit PR #1.2 polish
git add apps/server/PR-1.2-*.md apps/server/src/relay_physics/
git commit -m "fix(sse): PR #1.2 final polish (Q1-Q5 audit)"

# 3. Commit PR #1.3
git add apps/server/PR-1.3-*.md apps/server/src/relay_physics/
git commit -m "feat(api): PR #1.3 - Single commit fetch"

# 4. Commit RenderSpec v1
git add relay/
git commit -m "SPEC: Lock RenderSpec v1 (Layer 2→3 contract)"

# Push
git push origin main
```

---

## ARCHITECTURE REFERENCE (QUICK)

### Layer 2 Endpoints (Current)
```
✅ GET /api/relay-physics/events              (SSE stream)
✅ GET /api/relay-physics/commits/:ref        (forensic)
✅ GET /api/relay-physics/units               (state)
✅ POST /api/relay-physics/filaments/:id/commits
✅ GET /api/relay-physics/filaments/:id/commits

⏭️ GET /api/relay-physics/render/world        (scene graph)
⏭️ GET /api/relay-physics/render/commit/:ref  (forensic chamber)
⏭️ GET /api/relay-physics/render/filament/:id (thread detail)
```

### RenderSpec v1 Node Kinds
```
unit       - SCV agent
filament   - Thread/channel (polyline geometry)
timebox    - Commit cube
scar       - Rejection mark
anchor     - Filament endpoint
globe      - World container
label      - Text overlay
```

### Material Tag Examples
```
scv_working           - Unit working state
encrypted_channel     - Filament with encryption
commit_accepted       - Timebox accepted state
scar_authority_denied - Rejection mark
causal_link_input     - Causal dependency arrow
```

---

## QUESTIONS TO ASK CLAUDE (NEXT SESSION)

### If Option A (Backend)
1. "Implement `GET /render/world` endpoint that returns RenderSpec v1"
2. "Generate scene graph from current units + filaments state"
3. "Add golden fixture validation tests"

### If Option B (Frontend)
1. "Setup React + Three.js with react-three-fiber"
2. "Build RenderSpec v1 renderer that interprets `renderspec_v1_canonical.json`"
3. "Implement material mapping (semantic tags → Three.js materials)"

### If Option C (Parallel)
**Backend:** "Implement `/render/world` endpoint per RenderSpec v1"  
**Frontend:** "Build Three.js renderer against `renderspec_v1_canonical.json`"

---

## CONTEXT SUMMARY FOR CLAUDE

**What We Have:**
- Layer 2 "truth API" complete (events, commits, units)
- Architecture filament with 6 locked commits
- RenderSpec v1 fully specified
- PR #1.3 implemented (commit fetch)

**What We Need:**
- `/render/*` endpoints (backend) OR
- Three.js renderer (frontend) OR
- Both in parallel

**Philosophy Locked:**
- Layer 2 outputs data (semantic tags)
- Layer 3 renders pixels (shaders, materials)
- Deterministic (same truth → same scene)
- History is authority (log is source of truth)

---

## TLDR (30-SECOND RECAP)

**Status:** Layer 2 ~70% done. SSE stream works. Commit fetch works. RenderSpec v1 locked.

**Next:** Choose A (backend `/render/*`), B (frontend Three.js), or C (parallel).

**Recommendation:** Option C (parallel) for fastest visual demo.

**Key Files:**
- `relay/RENDERSPEC-V1-LOCKED.md` (contract)
- `relay/fixtures/renderspec_v1_canonical.json` (example)
- `relay/SESSION-SUMMARY-2026-01-28.md` (full context)

**Git:** 4 commits ready to push (see Git Workflow above)

---

**Last Session:** 2026-01-28  
**Next Session:** Choose A, B, or C and execute  
**Goal:** Visual demo within 1-2 weeks

# COMMIT A: Render Responsibility Split

**Filament:** `architecture`  
**Commit Index:** 0  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** ARCH_DECISION

---

## INVARIANT LOCKED

**Layer 2 outputs RenderSpec data structures. Layer 3 renders pixels. Layer 1 is git-only.**

---

## DECISION

### Layer 1 (Git Backend) - Storage Substrate
**Responsibilities:**
- Git operations (clone, pull, push, merge)
- File storage (repos, objects, trees)
- Evidence lookups (git refs, commit history)
- Merge conflict resolution

**NOT responsible for:**
- ❌ Rendering
- ❌ Real-time events
- ❌ Coordination physics
- ❌ State derivation

**Status:** Owned by external dev team. Layer 2 treats as external dependency.

---

### Layer 2 (Relay Physics) - Truth/Coordination Substrate
**Responsibilities:**
- ✅ Deterministic event stream (SSE with replay)
- ✅ Truth storage (event log, filament JSONL)
- ✅ Physics/coordination rules (verification pipeline)
- ✅ State derivation (unit states, filament heads)
- ✅ **Render-ready data structures** (scene graphs, geometry params)
- ✅ **Animation intents** (pulse, glow, twist, decay) keyed by `event_id`
- ✅ Forensic bundles (commit history, topology)

**NOT responsible for:**
- ❌ Real-time 3D rendering
- ❌ GPU work
- ❌ Image/video generation
- ❌ Game engine embedding
- ❌ Artistic/visual interpretation

**Technology:** Rust + Axum + JSONL + SSE

---

### Layer 3 (Frontend) - Rendering/Interaction Substrate
**Responsibilities:**
- ✅ **Rendering** (Three.js, WebGL, game engines)
- ✅ Interpret Layer 2's scene graphs → pixels
- ✅ Map semantic tags → visual language (glow rules, materials, animations)
- ✅ Subscribe to SSE and animate state changes
- ✅ User interaction (StarCraft HUD, forensic inspection)
- ✅ Camera control, lighting, post-processing

**NOT responsible for:**
- ❌ Truth storage
- ❌ Coordination rules
- ❌ Event replay logic (reads from Layer 2)

**Technology (Recommended):** React + Three.js (react-three-fiber)

**Alternative:** Electron, Rust + Bevy, Unreal/Unity (for film)

---

## BOUNDARY CONTRACT

### Layer 2 → Layer 3 Interface

**Format:** JSON scene graphs + SSE event stream

**Example Endpoints:**
```
GET /api/relay-physics/events              → SSE stream (real-time)
GET /api/relay-physics/render/world        → Globe + SCVs + filaments
GET /api/relay-physics/render/commit/:ref  → Forensic chamber scene
GET /api/relay-physics/render/filament/:id → Single filament geometry
GET /api/relay-physics/commits/:ref        → Raw commit data (forensic)
```

**Data Structure Philosophy:**
- Layer 2 outputs **semantic descriptions** (not artistic choices)
- Example: `{"material": "encrypted_channel"}` (semantic tag)
- Layer 3 maps to visuals: `encrypted_channel → twisted rope with rotation`

---

## WHY THIS SPLIT

### ✅ Separation of Concerns
- Truth/physics logic in Rust (deterministic, verifiable)
- Rendering in JavaScript (fast iteration, visual experimentation)

### ✅ Multiple Frontends
- Web client (Three.js)
- Desktop client (Electron)
- Cinematic pipeline (Unreal/Unity/Blender)
- All consume same Layer 2 API

### ✅ Testing Independence
- Layer 2 tests: determinism, replay, verification
- Layer 3 tests: visual regression, interaction, performance

### ✅ Deployment Independence
- Layer 2 = backend service (Rust binary)
- Layer 3 = static assets + client-side JS

---

## ANTI-PATTERNS REJECTED

### ❌ Rendering in Layer 1 (Git Backend)
**Why rejected:** Git backend is storage, not rendering. Mixing concerns.

### ❌ Rendering in Layer 2 (Relay Physics)
**Why rejected:** 
- Huge dependency burden (GPU libs, game engines)
- Deployment complexity (headless servers, GPU drivers)
- Breaks clean layering
- Can't support multiple frontend targets

### ❌ Truth Storage in Layer 3 (Frontend)
**Why rejected:** Frontend state is ephemeral. Truth must be server-side.

---

## MIGRATION PATH

### Current State (2026-01-28)
- ✅ Layer 2 event stream (PR #1.1, #1.2)
- ✅ Layer 2 state derivation (units, filaments)
- ⏭️ Layer 2 render endpoints (next: PR #1.3, then `/render/*`)
- ⏭️ Layer 3 frontend (React + Three.js)

### Next Steps
1. Complete Layer 2 "data API" (PR #1.3 - commit fetch)
2. Define RenderSpec v1 schema (lock Layer 2→3 contract)
3. Implement `/render/*` endpoints (Layer 2 outputs scene graphs)
4. Build Layer 3 renderer (Three.js interprets scene graphs)

---

## LOCKED INVARIANTS

1. **Layer 2 never renders pixels** (only outputs data)
2. **Layer 3 never stores truth** (only reads from Layer 2)
3. **Scene graphs are semantic, not artistic** (tags, not RGB values)
4. **Multiple frontends supported** (web, desktop, film)

---

## CAUSAL REFS

- **Inputs:** None (foundational decision)
- **Authority:** system.architect (bootstrap)
- **Evidence:** 
  - PR #1.1, #1.2 implementation (Layer 2 works without rendering)
  - Three.js ecosystem maturity (Layer 3 rendering proven)

---

**Status:** LOCKED  
**Supersedes:** None (foundational)  
**Rationale:** Clean separation enables parallel development, multiple frontends, and testability.

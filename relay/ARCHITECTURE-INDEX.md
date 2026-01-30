# Relay Architecture Index - Complete Reference

**Version:** 1.0  
**Last Updated:** 2026-01-28  
**Total Commits:** 10 (`architecture@c0` through `architecture@c9`)  
**Status:** All commits locked ✅

---

## QUICK NAVIGATION

- [Architecture Timeline](#architecture-timeline)
- [Dependency Graph](#dependency-graph)
- [By Domain](#by-domain)
- [Implementation Status](#implementation-status)
- [Quick Reference Cards](#quick-reference-cards)

---

## ARCHITECTURE TIMELINE

### Phase 1: Rendering Substrate (c0-c5)

#### `architecture@c0` - Render Responsibility Split
**Date:** 2026-01-28 10:00  
**File:** `0000_arch_split.md`  
**Invariant:** Layer 2 outputs RenderSpec data structures. Layer 3 renders pixels. Layer 1 is git-only.

**Key Decision:** Clear boundary between truth (Layer 2) and visuals (Layer 3).

**Dependencies:** None (bootstrap)

---

#### `architecture@c1` - Render Endpoints Contract
**Date:** 2026-01-28 10:15  
**File:** `0001_render_endpoints.md`  
**Invariant:** Layer 2 exposes 4 endpoint families: events (SSE), commits (forensic), units (state), render (scene graphs).

**Key Decision:** Locked API surface for Layer 2.

**Dependencies:** `c0`  
**Evidence:** PR-1.2-SSE-REPLAY.md

---

#### `architecture@c2` - SSE Truth Stream Locks
**Date:** 2026-01-28 10:30  
**File:** `0002_sse_truth_stream_locks.md`  
**Invariant:** The SSE event stream enforces: no phantom events, truth stream completeness, deterministic replay.

**Key Decision:** Event IDs from append-only log, lag closes connection, persist failure = fatal.

**Dependencies:** `c0`, `c1`  
**Evidence:** PR-1.1-PERSISTENCE-REPLAY.md, PR-1.2-SSE-REPLAY.md

---

#### `architecture@c3` - RenderSpec v1 Stub
**Date:** 2026-01-28 10:45  
**File:** `0003_renderspec_v1_stub.json`  
**Invariant:** JSON scene graph format for Layer 2→3 contract (stub, to be completed).

**Key Decision:** Reserved schema shape.

**Dependencies:** `c0`, `c1`  
**Status:** SUPERSEDED by `c5`

---

#### `architecture@c4` - Execution Order Decision
**Date:** 2026-01-28 11:00  
**File:** `0004_execution_order.md`  
**Invariant:** Execute in order: PR #1.3 (commit fetch) → RenderSpec v1 (schema lock) → /render/* endpoints → Layer 3 frontend.

**Key Decision:** Roadmap sequence.

**Dependencies:** `c0`, `c1`, `c2`, `c3`  
**Evidence:** PR-1.2-COMPLETE.md

---

#### `architecture@c5` - RenderSpec v1 Locked
**Date:** 2026-01-28 12:00  
**File:** `0005_renderspec_v1_locked.md`  
**Invariant:** RenderSpec v1 is the canonical JSON format for Layer 2→3 scene graphs. Material tags are semantic strings. Geometry is deterministic.

**Key Decision:** Polyline filaments, semantic materials, animation intents, no randomness.

**Dependencies:** `c0`, `c1`, `c3`  
**Evidence:** PR-1.3-COMMIT-FETCH.md, renderspec_v1_canonical.json  
**Supersedes:** `c3`

---

### Phase 2: Economic & Coordination Substrate (c6-c8)

#### `architecture@c6` - Economic Primitives Foundation
**Date:** 2026-01-28 14:00  
**File:** `0006_economic_primitives_foundation.md`  
**Invariant:** Money=transferable authority; Credit=visible delegation; Commitment≠Payment; Truth is substrate for all economic coordination.

**Key Decisions:**
- Money = Transferable authority tokens (no fractional reserve)
- Credit = Visible authority delegation (transparent)
- Voting ≠ Money (attention signals, not scarcity)
- Commitment primitives (stake reputation, not cash)
- Central banks = transparent issuers
- Banking = services, not gatekeepers

**Dependencies:** `c0`, `c2`  
**Evidence:** authority_store.rs, agent_ops.rs

---

#### `architecture@c7` - Coordination Gauges (StarCraft HUD Model)
**Date:** 2026-01-28 16:00  
**File:** `0007_coordination_gauges_hud.md`  
**Invariant:** Relay shows constraints not wealth. Multiple orthogonal gauges (legitimacy, commitments, escrow, time, disputes) replace single currency.

**Key Decisions:**
- 5 canonical gauges (legitimacy, commitments, escrow, time, disputes)
- StarCraft HUD model (show what you can do next)
- No single power metric (orthogonal dimensions)
- Decay animation (visible legitimacy drain)
- Scars are permanent (no memory hole)

**Dependencies:** `c6`  
**Evidence:** StarCraft UI design (1998)

---

#### `architecture@c8` - Delegated Decaying Influence (Fourth Primitive)
**Date:** 2026-01-28 17:00  
**File:** `0008_delegated_decaying_influence.md`  
**Invariant:** Authority is borrowed continuously. Influence decays unless renewed. Voting becomes relationship not decision.

**Key Decisions:**
- Delegated influence = temporary authority capacity
- Decay functions (linear, exponential, step, logistic)
- Influence cost model (action → cost mapping)
- No fractional reserve (full-reserve delegation)
- Spending is visible (forensic audit trails)
- The hinge concept: "We vote on you this much, you decide how to use our voting power"

**Dependencies:** `c6`, `c7`  
**Evidence:** Liquid democracy critique, Time-based authority decay

---

#### `architecture@c9` - Personal HUD + Physical Globe (StarCraft Model)
**Date:** 2026-01-28 18:00  
**File:** `0009_personal_hud_physical_globe.md`  
**Invariant:** HUD is personal (my state). Globe is shared (physical world). Shopping is production. Shipments are units. Buildings are physical entities.

**Key Decisions:**
- HUD = Personal (my resources, my tasks, my units, my notifications)
- Globe = Shared (buildings, other players, shipments, world state)
- Shopping = Unit production (StarCraft barracks/factory UI, not web checkout)
- Build queue = Task bar (active tasks with progress)
- Shipments = Drones (animated units flying on globe)
- Buildings = Physical entities (vendors, partnerships, civic, logistics, community)
- Dual-use interface (personal consumer + company procurement)

**Dependencies:** `c5`, `c7`  
**Evidence:** StarCraft UI design, RTS game mechanics, E-commerce logistics

---

## DEPENDENCY GRAPH

```
c0 (Render Split)
 ├─> c1 (Endpoints)
 │    ├─> c2 (SSE Locks)
 │    │    └─> c6 (Economic)
 │    │         ├─> c7 (HUD Gauges)
 │    │         │    └─> c8 (DDI)
 │    │         └─> [Future: PR #2-5]
 │    ├─> c3 (RenderSpec Stub)
 │    │    └─> c5 (RenderSpec Locked) [supersedes c3]
 │    │         └─> [Option C Backend]
 │    └─> c4 (Execution Order)
 └─> [Future: Layer 3 Frontend]
```

---

## BY DOMAIN

### Rendering Architecture
- **c0** - Layer responsibilities
- **c1** - Endpoint contracts
- **c2** - SSE invariants
- **c3** - RenderSpec stub (superseded)
- **c4** - Execution roadmap
- **c5** - RenderSpec locked

**Status:** ✅ Complete (backend implemented, frontend pending)

---

### Economic Architecture
- **c6** - Economic primitives (money, credit, voting, commitments)

**Status:** ✅ Architecture locked, implementation pending (PR #2-5)

---

### Coordination Architecture
- **c7** - StarCraft HUD gauges (5 orthogonal dimensions)
- **c8** - Delegated decaying influence (DDI)

**Status:** ✅ Architecture locked, implementation pending (PR #5 + HUD)

---

## IMPLEMENTATION STATUS

### ✅ Implemented (Code Exists)

#### Layer 2 Backend (Relay Physics)
```
apps/server/src/relay_physics/
├── types.rs                     (c0, c1)
├── commit.rs                    (c0, c1)
├── errors.rs                    (c0, c1)
├── agent_ops.rs                 (c0, c1)
├── verifier.rs                  (c0, c1, c2)
├── filament_store.rs            (c0, c1)
├── unit_store.rs                (c0, c1)
├── authority_store.rs           (c0, c1, c6)
├── event_log.rs                 (c2)
├── events.rs                    (c2)
├── commit_processor.rs          (c0, c1, c2)
├── commit_bundle.rs             (c1, c4)
├── renderspec.rs                (c5)
├── renderspec_generator.rs      (c5)
└── main_integration.rs          (c0-c5)
```

**Lines:** ~4,000 lines Rust

---

### ⏭️ Ready to Implement (Architecture Locked)

#### Economic Primitives (PR #2-5)
```
apps/server/src/relay_physics/
├── currency_store.rs            (c6) - PR #2
├── vote_store.rs                (c6, c7) - PR #3
├── commitment_store.rs          (c6, c7) - PR #4
├── delegation_store.rs          (c6, c7, c8) - PR #5
├── decay_engine.rs              (c8) - PR #5
└── influence_costs.rs           (c8) - PR #5
```

**Estimated lines:** ~2,500 lines Rust

---

#### HUD Components (Phase 5)
```
apps/client-web/src/
├── components/
│   ├── CoordinationGauges.tsx      (c7)
│   ├── DelegatedInfluenceGauge.tsx (c8)
│   ├── GaugeBar.tsx                (c7)
│   └── GaugeTooltip.tsx            (c7)
└── hooks/
    ├── useCoordinationGauges.ts    (c7)
    └── useDelegatedInfluence.ts    (c8)
```

**Estimated lines:** ~1,500 lines TypeScript/React

---

## QUICK REFERENCE CARDS

### Card 1: Rendering Substrate
**Commits:** `c0-c5`  
**Key Concept:** Layer 2 outputs **data**, Layer 3 renders **pixels**

**API Surface:**
- `GET /events` - SSE stream (replay + live)
- `GET /commits/:ref` - Forensic fetch
- `GET /units` - State view
- `GET /render/world` - Scene graph
- `GET /render/commit/:ref` - Forensic chamber
- `GET /render/filament/:id` - Thread detail

**Status:** ✅ Backend implemented, frontend pending

---

### Card 2: Economic Primitives
**Commit:** `c6`  
**Key Concept:** Truth is substrate for all economic coordination

**The 4 Primitives:**
1. Authority Tokens (Money) - Scarce, transferable, zero-sum
2. Attention Signals (Voting) - Abundant, non-transferable, positive-sum
3. Reputation (Merit) - Scarce, non-transferable, merit-based
4. Commitments (Promises) - Binding, visible, enforceable

**Operations:**
- `CURRENCY_*` - Issue, transfer, burn, delegate
- `VOTE_*` - Cast, refresh, retract, decay
- `REPUTATION_*` - Earn, slash
- `COMMITMENT_*` - Promise, fulfill, breach

**Status:** ✅ Architecture locked, implementation pending (PR #2-5)

---

### Card 3: Coordination Gauges
**Commit:** `c7`  
**Key Concept:** Relay shows **constraints**, not **wealth**

**The 5 Gauges:**
1. 🟦 Legitimacy - Votes / support (decaying)
2. 🟨 Commitment Capacity - Obligation bandwidth (capped)
3. 🟩 Escrowed Resources - Money / tokens (locked)
4. 🟪 Time Pressure - Countdowns / cooldowns (universal)
5. 🟥 Risk / Scars - Disputes / failures (permanent)

**Visual Model:** StarCraft HUD (show what you can do next)

**Status:** ✅ Architecture locked, implementation pending (HUD components)

---

### Card 4: Delegated Decaying Influence
**Commit:** `c8`  
**Key Concept:** Authority is borrowed continuously, not granted once

**How It Works:**
1. Voters express support → Agent gains influence
2. Influence decays over time → Unless renewed
3. Agent spends influence to act → Visible, attributable
4. Influence exhausts → Agent must regain support

**Decay Functions:**
- Linear (default) - Constant rate
- Exponential - Accelerating decay
- Step - Drops at intervals
- Logistic - S-curve

**Influence Costs:**
- Comment: 0%
- Propose: 5%
- Approve: 10-20%
- Allocate escrow: 20-50%
- Override dispute: 50-80%
- Emergency: 80-100%

**Status:** ✅ Architecture locked, implementation pending (PR #5)

---

## LOCKED INVARIANTS (COMPLETE LIST)

### Rendering (c0-c5)
1. ✅ Layer 2 outputs data, Layer 3 renders pixels
2. ✅ Event IDs from append-only log (deterministic)
3. ✅ Lag closes connection (force replay)
4. ✅ Persist failure = fatal (no phantom events)
5. ✅ RenderSpec material tags are semantic strings
6. ✅ Filaments are polyline segments (deterministic)
7. ✅ No randomness in rendering (same truth → same visual)

### Economic (c6)
8. ✅ Money = Transferable authority tokens
9. ✅ Credit = Visible authority delegation (no fractional reserve)
10. ✅ Voting ≠ Money (attention vs scarcity)
11. ✅ Commitment ≠ Payment (reputation, not cash)
12. ✅ All economic actions = commits (transparent)
13. ✅ No phantom money (full-reserve accounting)
14. ✅ Central banks = transparent issuers
15. ✅ Banking = services, not gatekeepers

### Coordination (c7)
16. ✅ Relay shows constraints, not wealth
17. ✅ 5 orthogonal gauges (no single power metric)
18. ✅ Legitimacy decays (requires maintenance)
19. ✅ Commitments are capped (prevents infinite promises)
20. ✅ Escrow is downstream (money doesn't buy votes)
21. ✅ Time is universal (no pay-to-skip)
22. ✅ Scars are permanent (visible history)

### Delegated Influence (c8)
23. ✅ Authority is borrowed continuously
24. ✅ Influence decays unless renewed
25. ✅ Influence is scoped (domain-specific)
26. ✅ Spending is visible (forensic trails)
27. ✅ No double-spending (causal refs)
28. ✅ No fractional reserve (full-reserve delegation)
29. ✅ Influence ≠ money (can't be bought/sold/hoarded)
30. ✅ Exhaustion is explicit (below threshold → delegation ends)

### Personal HUD / Globe (c9)
31. ✅ HUD is personal (my state, not world state)
32. ✅ HUD is viewer-scoped (filtered by identity)
33. ✅ Globe is shared (physical world map)
34. ✅ Buildings are physical entities
35. ✅ Shopping is production (not web checkout)
36. ✅ Build queue is taskbar (visible progress)
37. ✅ Shipments are units (drones on globe)
38. ✅ Dual-use interface (personal + company)
39. ✅ Resources are personal gauges (my legitimacy, my commitments)
40. ✅ Layer 2 outputs building/task/shipment nodes
41. ✅ Layer 3 renders StarCraft UI
42. ✅ Physical logistics (tangible coordination)

**Total:** 42 locked invariants

---

## FILE MANIFEST

### Architecture Filaments
```
relay/filaments/architecture/
├── README.md                                   (Index)
├── 0000_arch_split.md                         (c0)
├── 0001_render_endpoints.md                   (c1)
├── 0002_sse_truth_stream_locks.md             (c2)
├── 0003_renderspec_v1_stub.json               (c3, superseded)
├── 0004_execution_order.md                    (c4)
├── 0005_renderspec_v1_locked.md               (c5)
├── 0006_economic_primitives_foundation.md     (c6)
├── 0007_coordination_gauges_hud.md            (c7)
└── 0008_delegated_decaying_influence.md       (c8)
```

### Documentation
```
relay/
├── ECONOMIC-SUBSTRATE-LOCKED.md               (Quick answers)
├── COORDINATION-SUBSTRATE-GOLD-STANDARD.md    (Complete spec)
├── RENDERSPEC-V1-LOCKED.md                    (Rendering guide)
├── SESSION-SUMMARY-2026-01-28.md              (Rendering session)
├── SESSION-SUMMARY-2026-01-28-ECONOMIC.md     (Economic session)
└── ARCHITECTURE-INDEX.md                      (This file)
```

### Fixtures
```
relay/fixtures/
├── renderspec_v1_canonical.json               (c5)
└── renderspec_v1_minimal.json                 (c5)
```

---

## NEXT STEPS ROADMAP

### Immediate (1-2 hours)
**Test render endpoints** (`/render/*`)
- Manual testing with curl
- Validate RenderSpec v1 output
- Fix any issues

**Start phrase:** `"Resume Option C - backend testing"`

---

### Short-Term (4-6 hours each)
**Implement economic primitives** (PR #2-5)
- PR #2: Currency filaments
- PR #3: Voting filaments
- PR #4: Commitment filaments
- PR #5: Delegation filaments + decay engine

**Start phrase:** `"Start PR #2 - Currency Filaments"`

---

### Medium-Term (8-10 hours)
**Build HUD components** (React + Three.js)
- 5 coordination gauges
- Delegated influence gauge
- SSE live updates
- Animations (decay, spending, renewal)

**Start phrase:** `"Start Phase 5 - HUD Components"`

---

### Long-Term (TBD)
**Layer 3 frontend** (React + Three.js)
- 3D globe rendering
- SCV unit rendering
- Filament polylines
- Timebox visualization
- Forensic chamber

**Start phrase:** `"Start Layer 3 - Three.js Frontend"`

---

## PHILOSOPHY SUMMARY

**Before Relay:**
- Money = opaque trust
- Voting = one-time decision
- Authority = granted once
- Banking = gatekeepers

**After Relay:**
- Money = transparent truth
- Voting = continuous relationship
- Authority = borrowed legitimacy
- Banking = coordination services

**The transformation:**
- Truth becomes the substrate
- Coordination becomes transparent
- Authority requires maintenance
- Legitimacy is continuous

---

**Status:** ✅ Architecture complete (10 commits locked)  
**Next:** Choose implementation path (render testing, economic primitives, or HUD)  
**Total Specification:** ~35,000 lines (architecture + docs + code)

---

**End of Architecture Index.**

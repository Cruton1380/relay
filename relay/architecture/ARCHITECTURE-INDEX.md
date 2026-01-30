# RELAY ARCHITECTURE INDEX

**Purpose:** Complete index of all locked architecture commits  
**Last Updated:** 2026-01-29  
**Total Commits:** 17 (c0-c16)  
**Total Invariants:** 107

---

## 🎯 HOW TO USE THIS INDEX

**Each architecture commit lists:**
1. **What it locks** (immutable invariants)
2. **What it enables** (new capabilities)
3. **What it forbids** (anti-patterns)
4. **Dependencies** (what it builds on)
5. **Depended by** (what builds on it)

**This is physics, not narrative. Dependencies form a DAG (Directed Acyclic Graph).**

---

## 📐 ARCHITECTURE COMMITS (c0-c16)

### **c0: Filament Physics Foundation**
**File:** `filaments/0000_filament_physics.md`

**Locks:**
- Filaments are append-only value identities
- Filaments never mutate, only extend
- Filaments conserve value and history
- Filaments change only via explicit deformation
- Zoom reveals temporal spacing, not new semantics

**Enables:**
- Temporal integrity
- Audit trails
- Replayability
- History preservation

**Forbids:**
- History mutation
- Silent deletion
- Retroactive edits
- Value non-conservation

**Dependencies:** None (foundational)

**Depended by:** c1, c2, c3, c10, c12, c16, c17

---

### **c1: Commit Semantics**
**File:** `filaments/0001_commit_semantics.md`

**Locks:**
- Commits are execution, not tooling
- Commits require causal_refs
- No hidden operations layer
- Git backend enforces commit-only updates

**Enables:**
- Event sourcing
- Causality tracking
- Deployment = commit

**Forbids:**
- Silent operations
- Hidden CI layer
- Retroactive changes

**Dependencies:** c0

**Depended by:** c2, c5, c16

---

### **c2: Replayability & Determinism**
**File:** `filaments/0002_replayability.md`

**Locks:**
- Same commits → same state (always)
- Replay must be deterministic
- No randomness in state derivation
- Timestamps are inputs, not state mutators

**Enables:**
- Audit verification
- Testing reproducibility
- Conflict resolution via replay

**Forbids:**
- Non-deterministic state
- Random IDs
- Time-dependent geometry

**Dependencies:** c0, c1

**Depended by:** c5, c16

---

### **c3: Truth Substrate**
**File:** `filaments/0003_truth_substrate.md`

**Locks:**
- Layer 1 (Git) = storage
- Layer 2 (Relay Physics) = truth/coordination
- Layer 3 (Frontend) = rendering/interaction
- Clear separation of concerns

**Enables:**
- Independent layer development
- Rendering without truth mutation

**Forbids:**
- UI-driven truth
- Rendering logic in truth layer

**Dependencies:** c0, c1

**Depended by:** c5, c9

---

### **c4: [Reserved for future use]**

---

### **c5: RenderSpec v1**
**File:** `filaments/0005_renderspec_v1.md`

**Locks:**
- Deterministic rendering (no randomness)
- Semantic materials (not RGB)
- Stable IDs (hash-based or filament-derived)
- No unknown keys
- Filament timeboxes have commit_refs

**Enables:**
- Layer 2 → Layer 3 contract
- Multi-client rendering
- Visual verification

**Forbids:**
- RGB hex codes
- Random geometry
- Time-dependent positions
- Material invention

**Dependencies:** c0, c2, c3

**Depended by:** c9, c13, c14, c15

---

### **c6: Economic Primitives**
**File:** `filaments/0006_economic_primitives.md`

**Locks:**
- Authority Tokens (Money)
- Attention Signals (Voting)
- Reputation (Merit)
- Commitments (Promises)
- Credit
- Commitment without Money

**Enables:**
- Resource tracking
- Escrow
- Value transfer

**Forbids:**
- Hidden value creation
- Unbacked promises

**Dependencies:** c0

**Depended by:** c7, c8

---

### **c7: Coordination Gauges (StarCraft HUD)**
**File:** `filaments/0007_coordination_gauges.md`

**Locks:**
- 5 canonical gauges: Legitimacy, Commitment Capacity, Escrowed Resources, Time/Cooldown Pressure, Risk/Scar/Dispute Pressure
- Gauges are orthogonal constraints (not currencies)
- HUD is personal (not global)

**Enables:**
- Personal capacity tracking
- Constraint visibility
- Operational clarity

**Forbids:**
- Global dashboards
- Hidden constraints
- Infinite capacity

**Dependencies:** c6, c9

**Depended by:** c9, c12

---

### **c8: Delegated Decaying Influence**
**File:** `filaments/0008_delegated_decaying_influence.md`

**Locks:**
- Voting = force lending (not truth production)
- Influence decays over time
- Votes are revocable
- Scope-bound (not global)
- Rewards duration, not spikes

**Enables:**
- Continuous democracy
- Temporary authority
- Map control (not ballots)

**Forbids:**
- Permanent voting power
- Global authority
- Instant finalization

**Dependencies:** c6

**Depended by:** c13

---

### **c9: Personal HUD + Physical Globe (StarCraft Model)**
**File:** `filaments/0009_personal_hud_physical_globe.md`

**Locks:**
- HUD = personal state (my resources, my tasks)
- Globe = shared physical world (buildings, units, shipments)
- Shopping = unit production (not checkout)
- Shipments = animated drones
- Buildings = physical entities

**Enables:**
- Dual-view interface
- Personal vs shared separation
- Logistics visualization

**Forbids:**
- Global dashboards
- Abstract shopping flows
- Hidden logistics

**Dependencies:** c5, c7

**Depended by:** c10, c13, c14, c15

---

### **c10: Ontological Foundation**
**File:** `filaments/0010_ontological_foundation.md`

**Locks:**
- Users ARE filament trees (not accounts)
- Identity IS a filament (evolves over time)
- Buildings ARE space tiles (accumulate history)
- Buildings ARE units (tradable, governable)
- Proximity channels (property of space + presence)
- Sacred invariant: No filament may collapse into scalar

**Enables:**
- Identity evolution
- Building governance
- Spatial coordination
- Exit without punishment

**Forbids:**
- Global reputation scores
- Universal currency
- Single trust scores
- Permanent authority

**Dependencies:** c0, c9

**Depended by:** c11, c13, c14, c15, c16

---

### **c11: Root AI Coherence Layer**
**File:** `filaments/0011_root_ai_coherence.md`

**Locks:**
- Root AI maintains unbranched logic
- Specialized agents (3-5 max) operate on branches
- Agents consult Root, not each other directly
- Root = coherence layer (not supreme intelligence)
- Root decisions are commits

**Enables:**
- Multi-agent coordination
- Context stability
- Scope coherence

**Forbids:**
- Agent drift
- Peer-to-peer chaos
- Silent authority

**Dependencies:** c10

**Depended by:** c12

---

### **c12: Root AI Cognitive Substrate**
**File:** `filaments/0012_root_ai_cognitive_substrate.md`

**Locks:**
- Conversations ARE filaments (append-only commits)
- Agent training IS deterministic compilation (Training Packs)
- Steering IS explicit branch selection (Logic Branches + Profiles)
- Learning IS append-only artifacts (Outcome Records)
- Root HUD = Compiler + Router + Auditor
- Profiles + Gates + Traces > raw weights
- Humans operate at header level (zoom to proof when needed)
- Gates enforce physics (auto violation detection)

**Enables:**
- Conversation history
- Agent onboarding from commits
- Visible AI steering
- Violation detection

**Forbids:**
- Hidden AI learning
- Keyword stuffing to steer
- Silent weight changes
- Context loss

**Dependencies:** c11, c10

**Depended by:** c16, c17, c18

---

### **c13: Rule-Based Zones (Geographic + Contextual Constraint Regions)**
**File:** `filaments/0013_rule_based_zones.md`

**Locks:**
- Rules spatially + contextually bound to zones
- No global rules by default
- Territory → ConstraintRegion
- Boundaries → Explicit transitions
- Enforcement → Structural modes (Hard/Soft/Deferred/Info)
- Hierarchical voting (one level up)
- Transition warnings + grace periods
- No retroactive/discretionary enforcement

**Enables:**
- Predictable public order
- Legitimate protest + functional transit
- Context-appropriate behavior
- Machine-verifiable legality

**Forbids:**
- Global behavior rules
- Selective policing
- Vibe-based enforcement
- Implicit norms

**Dependencies:** c10, c9, c5

**Depended by:** c14, c15, c17

---

### **c14: Presence, Sessions, and Co-Located Play (Social Layer)**
**File:** `filaments/0014_presence_sessions_colocated_play.md`

**Locks:**
- Presence = progressive identity disclosure (per viewer/zone/session)
- Sessions = first-class spatial objects (consent required)
- Triple binding: Physical proximity + Cognitive channel + Graphics anchor
- Local + remote share cognitive space
- Public visibility ≠ identity exposure
- Building beacons anchor world shards
- Visitor arrival = progressive trust unlock

**Enables:**
- Gaming as coordination
- Collaborative work as sessions
- Social presence as physics
- Global + local unity

**Forbids:**
- Fixed presence profiles
- Forced identity exposure
- Local vs remote separation
- Privacy all-or-nothing

**Dependencies:** c13, c10, c9, PR#9

**Depended by:** c15, c17

---

### **c15: Global Conflict & LOD Reality Rendering**
**File:** `filaments/0015_global_conflict_lod_rendering.md`

**Locks:**
- All critical force objects MUST exist as ForceUnits
- LOD governs detail, not existence
- Lenses govern visibility
- Military force → StarCraft units
- Damage → DamageScar objects
- No hidden force
- No retroactive movement
- No invisible damage
- No global threat scalar

**Enables:**
- Early escalation detection
- Damage accountability
- Civilian warnings
- Global transparency demands

**Forbids:**
- Hidden military assets
- Deniable buildup
- Opaque escalation
- Contested damage (without audit)

**Dependencies:** c13, c10, c9, c5

**Depended by:** c16

---

### **c16: Universal Audit & Assurance Layer (Relay GAPP / NIST)**
**File:** `filaments/0016_universal_audit_assurance.md`

**Locks:**
- Audit = continuous immune system
- MaterialityRules define reality requirements
- RealityCoverageMap detects blind spots
- Assertions require evidence
- Findings are scars (never deleted)
- Opinions are scoped (not global)
- Independence provable
- No self-certification
- No silent omission
- Audit applies universally (all domains)

**Enables:**
- Continuous verification
- Material omission detection
- Cross-domain audit
- Traditional auditor compatibility

**Forbids:**
- Periodic-only audit
- Self-certification
- Global audit scores
- Hidden audit process
- Finding deletion

**Dependencies:** ALL (c0-c15)

**Depended by:** c17

---

## 📊 DEPENDENCY GRAPH (VISUAL)

```
c0 (Filaments) ─────┬─────────────────────────┐
                    │                         │
c1 (Commits) ───────┼─────────┐              │
                    │         │              │
c2 (Replay) ────────┼─────┐   │              │
                    │     │   │              │
c3 (Layers) ────────┼──┐  │   │              │
                    │  │  │   │              │
c5 (RenderSpec) ────┼──┴──┴───┴──┐           │
                    │             │           │
c6 (Economics) ─────┼──┐          │           │
                    │  │          │           │
c7 (Gauges) ────────┼──┴──┐       │           │
                    │     │       │           │
c8 (Voting) ────────┼──┐  │       │           │
                    │  │  │       │           │
c9 (StarCraft) ─────┼──┴──┴───┐   │           │
                    │         │   │           │
c10 (Ontology) ─────┴────┐    │   │           │
                         │    │   │           │
c11 (Root AI) ───────────┼────┼───┼───┐       │
                         │    │   │   │       │
c12 (Cognitive) ─────────┴──┐ │   │   │       │
                            │ │   │   │       │
c13 (Zones) ────────────────┴─┴───┴───┼───┐   │
                                      │   │   │
c14 (Presence) ───────────────────────┴───┼───┤
                                          │   │
c15 (Conflict) ───────────────────────────┴───┤
                                              │
c16 (Audit) ──────────────────────────────────┘
```

---

## 🔄 REALM CONVERSIONS COMPLETE

**Locked conversions (17/20):**
1. Space → Globe build space (c0, c9)
2. Time → Commit order (c0, c2)
3. Value → Filament thickness (c6)
4. Identity → Filament trees (c10)
5. Action → Execution queues (c7)
6. Knowledge → Artifacts/Filaments (c12)
7. Legitimacy → History-weighted credibility (c8)
8. Cognition → Profiles + Gates + Traces (c12)
9. Territory → Constraint Regions (c13)
10. Boundaries → Explicit transitions (c13)
11. Context → Contextual constraints (c13)
12. Enforcement → Structural modes (c13)
13. Presence → Progressive disclosure (c14)
14. Force → ForceUnit objects (c15)
15. Conflict → Lens-visible patterns (c15)
16. Damage → DamageScar objects (c15)
17. Verification → Audit objects (c16)

**Remaining (future):**
18. Deadlines → TimeWindow objects
19. Pricing → External reference
20. Organizations → CompositeIdentity

---

## 🎯 QUICK REFERENCE BY DOMAIN

### **Identity & Access:**
- c10 (Users as filament trees)
- PR#9 (Identity filaments implementation)
- c14 (Presence tiers)

### **Coordination & Work:**
- c7 (Gauges)
- PR#7 (Tasks)
- c12 (SCVs)

### **Governance:**
- c8 (Voting)
- c13 (Zones)

### **Economics:**
- c6 (Primitives)
- c9 (StarCraft model)

### **Military & Conflict:**
- c15 (Force units, LOD, lenses)

### **Verification:**
- c16 (Universal audit)

### **AI & Cognition:**
- c11 (Root AI)
- c12 (Cognitive substrate)

### **Social:**
- c14 (Presence, sessions)

---

## 📋 ARCHITECTURE SUMMARY TABLE

| Commit | Title | Key Invariants | Deps | Status |
|--------|-------|----------------|------|--------|
| c0 | Filament Physics | Append-only, immutable, conserved | None | 🔒 |
| c1 | Commit Semantics | Commits = execution | c0 | 🔒 |
| c2 | Replayability | Deterministic replay | c0,c1 | 🔒 |
| c3 | Truth Substrate | Layer separation | c0,c1 | 🔒 |
| c5 | RenderSpec v1 | Deterministic rendering | c0,c2,c3 | 🔒 |
| c6 | Economic Primitives | Money, voting, reputation | c0 | 🔒 |
| c7 | Coordination Gauges | 5 orthogonal constraints | c6,c9 | 🔒 |
| c8 | Delegated Influence | Force lending | c6 | 🔒 |
| c9 | StarCraft Model | HUD personal, globe shared | c5,c7 | 🔒 |
| c10 | Ontological Foundation | Users = trees, no scalars | c0,c9 | 🔒 |
| c11 | Root AI Coherence | Unbranched root logic | c10 | 🔒 |
| c12 | Cognitive Substrate | Conversations = filaments | c11,c10 | 🔒 |
| c13 | Rule-Based Zones | Rules spatially bound | c10,c9,c5 | 🔒 |
| c14 | Presence & Sessions | Progressive disclosure | c13,c10,c9 | 🔒 |
| c15 | Conflict & LOD | Force units visible | c13,c10,c9,c5 | 🔒 |
| c16 | Universal Audit | Continuous verification | ALL | 🔒 |

---

**For detailed specs, read individual architecture files in `architecture/filaments/`**

**For dependencies, see DAG above or CONTEXT-TABLE.json**

**END OF ARCHITECTURE INDEX**

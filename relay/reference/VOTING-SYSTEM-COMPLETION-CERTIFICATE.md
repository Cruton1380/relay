# 🔒 VOTING SYSTEM COMPLETION CERTIFICATE

**Relay's Voting Engine: Design Complete**  
**Type:** Completion Certificate & Implementation Handoff  
**Status:** 🔒 LOCKED - NO FURTHER DESIGN  
**Date:** 2026-01-29  
**Signed Off:** Confirmed by system architect

---

## 🎯 EXECUTIVE SUMMARY

**Relay's voting system is complete.**

**No further conceptual work is needed.**  
**Only verification and implementation remain.**

---

## ✅ WHAT HAS BEEN LOCKED

### **1. Single Voting Physics (Original System)**

**Already implemented months ago. PERFECT. DO NOT MODIFY.**

- ✅ Regional elections (topic rows, stabilization windows)
- ✅ Delegated Decaying Influence (DDI) - time-bounded authority
- ✅ Authority delegation graph - traceable, no ambient authority
- ✅ Zero-knowledge vote privacy (ZK-STARK proofs)
- ✅ Continuous accountability (elections never end)

**Files:**
- `documentation/VOTING/ELECTION-SYSTEM.md`
- `relay/filaments/architecture/0008_delegated_decaying_influence.md`
- `AUTHORITY-DELEGATION-COMPLETE.md`

**Status:** ✅ **COMPLETE - 17 TESTS PASSING**

---

### **2. Extended Vote Contexts (New - Extends Original)**

**Uses same voting physics, different object types.**

#### **Canon Selection Votes**
- History disputes (which model is active canon?)
- Space observations (which interpretation is operational?)
- Scientific models (which theory guides coordination?)
- Idea viability (which proposal earns adoption?)

**Key invariant:**
> "Votes select operative canon (what system relies on), NOT objective truth."

**Files:**
- `relay/reference/GLOBE-TIME-SPACE-MODEL.md`
- `relay/reference/VOTING-SYSTEM-ALIGNMENT.md`

**Status:** ✅ **SPEC LOCKED - AWAITING IMPLEMENTATION**

---

### **3. Evidence Level System**

**Distinguishes belief from proof without censorship.**

**Glyphs:**
- ◐ Subjective (personal belief, no evidence)
- ✶ Faith/Metaphysics (explicitly non-evidential)
- ◆ Evidence-backed (sources attached)
- ✔︎ Canon (voted as active model)

**Purpose:** Allow fringe ideas while preventing silent truth-by-authority

**Files:**
- `relay/reference/VOTING-SYSTEM-ALIGNMENT.md` (schema defined)

**Status:** ✅ **SPEC LOCKED - AWAITING IMPLEMENTATION**

---

### **4. Vote Turbulence Visualization**

**Makes community belief physically visible.**

**Five thermal signals:**
1. Vote velocity (delta votes / time)
2. Vote acceleration (change in velocity)
3. Authority transfer rate (delegated influence spent)
4. Fork/scar event rate (state changes)
5. Downstream causal footprint (impact size)

**Four alert tiers:**
1. Ambient (subtle shimmer)
2. Active debate (visible flow)
3. Battle (strong force trails, notification)
4. Break event (pulse ring, persistent marker)

**Rendering strategy:**
- Near: 30-60fps, high detail
- Mid: 10-30fps, medium detail
- Far: 1-10fps, low detail
- Cold: static
- Hot spots: prioritized budget

**Files:**
- `relay/reference/VOTE-TURBULENCE-VISUALIZATION-SPEC.md`

**Status:** ✅ **SPEC LOCKED - AWAITING IMPLEMENTATION**

---

## 🔒 CANONICAL INVARIANTS (NEVER VIOLATE)

### **Invariant 1: Votes Are Signals, Not Truth**
```
Votes select the system's active working canon.
They do NOT rewrite evidence, erase branches, or manufacture truth.
```

**Enforcement:**
- Evidence filaments = immutable
- Belief filaments = vote-selected, timestamped, revisable
- All forks preserved
- Canon flips recorded with commitIndex

---

### **Invariant 2: Field Is Projection Only**
```
Thermal-force field visualizes activity.
It NEVER changes the underlying vote ledger.
Rendering = output, not input.
```

**Enforcement:**
- Vote commits remain append-only
- Field computed from ledger
- LOD affects rendering frequency, not ledger updates
- User interactions (SELECT/HOLD/ZOOM) read, don't write

---

### **Invariant 3: Authority Is Traceable**
```
No action is valid unless authority is derivable by replay
from an explicit delegation chain in force at the action's commitIndex.
```

**Enforcement:**
- authorityRef required
- Delegation path = structured commit references
- No ambient authority
- Services are executors, not originators

---

### **Invariant 4: Influence Decays**
```
Authority is borrowed continuously, not granted once.
Influence decays unless renewed.
```

**Enforcement:**
- DDI decay rules (linear/exponential/step/logistic)
- System emits DELEGATION_DECAYED events
- Below threshold → DELEGATION_EXHAUSTED
- Spending drains influence

---

### **Invariant 5: Normalize to Context**
```
"Fast" means deviation from scope baseline, not absolute counts.
Prevents everything from being "on fire."
```

**Enforcement:**
```
heat_intensity = (current_velocity - baseline_velocity) / baseline_velocity
```

Four baselines: globe, region, topic, branch

---

## 📋 CLAUDE'S VERIFICATION CHECKLIST

**Send these to Claude. Demand proof, not promises.**

---

### ✅ **A. VOTING REUSE (CRITICAL)**

**Questions:**
1. Show where canon-selection votes reuse the existing DDI / election machinery without introducing new vote logic.
2. Which code paths are shared between regional elections and history canon votes?
3. Which parameters differ only by context (object type)?
4. Confirm that canon-selection votes inherit decay, revocability, scope, and privacy exactly as regional elections do.

**Expected answer:**
```
Code paths: src/relay_physics/vote_engine.rs (shared)
Context diff: vote_target (governor vs history.topic vs space.object)
DDI inheritance: YES (same decay_policy, same authority_graph, same ZK privacy)
```

**FAIL if:** Claude shows separate voting logic for history vs elections.

---

### ✅ **B. HISTORY AS VOTE TARGET (NOT A NEW SYSTEM)**

**Questions:**
1. How is a `history.topic.*` filament represented so it can be voted on using the same backend as a governor or proposal?
2. Where is the rule enforced that losing history branches remain preserved and queryable after a canon vote?
3. Show the commit that records a canon flip (must include previous canon ref).

**Expected answer:**
```
FilamentType::HistoryTopic { branches: Vec<Branch> }
VoteTarget trait implemented for HistoryTopic, Governor, Proposal
Branch preservation: ALL commits append-only, canon pointer updated
Canon flip commit: CANON_SELECTED { previous_canon, new_canon, vote_refs }
```

**FAIL if:** Canon vote erases branches or doesn't record previous state.

---

### ✅ **C. PERSONAL WORLDVIEW FILAMENTS**

**Questions:**
1. Confirm that `user.<id>.worldview` filaments are:
   - append-only ✅
   - not auto-promoted to global history ✅
   - promotable only via explicit proposal + vote ✅
2. Show how subjective/faith/interpretation glyphs are stored as metadata, not inferred from UI state.
3. Where is the rule that prevents personal worldview from being rendered as system canon without a vote?

**Expected answer:**
```
Filament structure: user.<id>.worldview (scoped to user)
Glyph storage: metadata field in commit { glyph: "◐", evidence_level: "none" }
Promotion path: user.worldview → PROPOSAL_CREATED → CANON_VOTE → (if wins) → belief.canon
Projection rule: IF (filament_scope == "user") THEN render_as_personal ELSE render_as_global
```

**FAIL if:** Personal worldview leaks into global canon without explicit vote.

---

### ✅ **D. DOUBLE-SIDED USER LEDGER**

**Questions:**
1. Where is double-entry enforced structurally (debit/credit balance), not narratively?
2. Show the schema for a ledger entry (must have debit + credit arrays).
3. Confirm that personal ledgers can be aggregated into CV/reputation projections without exposing private raw entries.

**Expected answer:**
```rust
struct LedgerEntry {
    entry_id: String,
    debits: Vec<LedgerLine>,   // What you received/consumed/learned
    credits: Vec<LedgerLine>,  // What you produced/asserted/influenced
    balance_check: bool,       // sum(debits) == sum(credits)
}

Aggregation: CV = projection_from_commits(user.*.ledger)
Privacy: Raw entries private, aggregate public (opt-in)
```

**FAIL if:** Ledger allows unbalanced entries or exposes all private data.

---

### ✅ **E. SPACE + HISTORY SYMMETRY**

**Questions:**
1. Show that space-object canon votes and history canon votes are identical at the voting layer, differing only in object type.
2. Confirm that distance shells (space) and day shells (history) are navigation projections, not separate storage systems.

**Expected answer:**
```
Vote layer: VoteTarget trait (same for HistoryTopic, SpaceObject)
Object diff: filament_type (history.topic.* vs space.object.*)
Day shells: Projection from commits where timestamp ∈ [day_start, day_end]
Distance shells: Projection from objects where distance ∈ [shell_min, shell_max]
Storage: Same filament store, different queries
```

**FAIL if:** Separate vote engines or storage systems for space vs history.

---

### ✅ **F. FRINGE IDEAS / MARKETPLACE**

**Questions:**
1. How does the system prevent a fringe idea with high attention votes but zero evidence from being rendered as "fact" in any default lens?
2. Where is attribution enforced structurally when a high-authority actor adopts a low-authority proposal?

**Expected answer:**
```
Glyph enforcement: IF (evidence_refs.length == 0) THEN glyph = "◐" (subjective)
Default lens: Filter by evidence_level >= "sourced" OR show glyph
Attribution: ADOPTION_COMMIT { proposal_ref, adopter_ref } → both stored
Reputation calc: CV includes refs to original proposals (structural credit)
```

**FAIL if:** Zero-evidence ideas can render as facts, or adoption doesn't cite source.

---

### ✅ **G. THERMAL FIELD IMPLEMENTATION**

**Questions:**
1. Show how the five thermal signals are computed from the vote ledger.
2. Confirm that baselines are contextual (globe/region/topic/branch).
3. Prove that field rendering is projection only (never writes back to ledger).
4. Show LOD implementation (near/mid/far update rates).

**Expected answer:**
```rust
// 1. Signal computation
vote_velocity = (votes_now - votes_prev) / time_delta
vote_acceleration = (velocity_now - velocity_prev) / time_delta
authority_rate = sum(DELEGATION_SPENT.influence_cost) / time_delta
event_rate = count(FORK|SCAR|CANON_FLIP) / time_delta
footprint = count(commits_referencing_this_branch)

// 2. Contextual baselines
globe_baseline = rolling_avg(all_votes, 7d)
region_baseline = rolling_avg(region_votes, 7d)
topic_baseline = rolling_avg(topic_votes, 7d)
branch_baseline = rolling_avg(branch_votes, 7d)

// 3. Projection only
ThermalField::update() {
    read_from_ledger();  // ✅ Read
    compute_signals();
    render_field();
    // NO write_to_ledger()  // ✅ No write
}

// 4. LOD
if (distance < 100m) { update_rate = 60fps; }
else if (distance < 1km) { update_rate = 30fps; }
else if (distance < 10km) { update_rate = 10fps; }
else { update_rate = 0fps; } // static
```

**FAIL if:** Field computation writes back to ledger or ignores LOD.

---

### ✅ **H. ALERT SYSTEM**

**Questions:**
1. Where are the four alert tiers implemented (ambient/active/battle/break)?
2. Confirm that alerts are view-layer events (user can dismiss/mute).
3. Show how camera suggestions work (never forced).

**Expected answer:**
```rust
fn classify_alert_tier(heat_intensity: f32, event_type: Option<EventType>) -> AlertTier {
    match (heat_intensity, event_type) {
        (_, Some(EventType::Fork | Scar | CanonFlip)) => AlertTier::BreakEvent,
        (h, _) if h > 2.0 => AlertTier::Battle,
        (h, _) if h > 0.5 => AlertTier::ActiveDebate,
        _ => AlertTier::Ambient,
    }
}

// View layer only
emit_alert(tier, position) {
    UI::show_notification(tier, dismissible=true);
    Camera::suggest_look_at(position, force=false);  // Never forced
}
```

**FAIL if:** Alerts force camera movement or can't be dismissed.

---

### ✅ **I. FINAL SANITY CHECK**

**Question:**
1. List every place where a vote outcome changes system behavior vs places where it only changes default projection.

**Expected answer:**
```
CHANGES BEHAVIOR:
- Regional governor election → Multi-sig authority granted
- Delegation vote → Influence capacity granted
- Resource allocation → Escrow released

CHANGES PROJECTION ONLY:
- History canon vote → Active model pointer updated (branches preserved)
- Space model vote → Default interpretation updated (forks preserved)
- Idea endorsement → Attention weight updated (no authority granted)
```

**FAIL if:** Projection changes are confused with behavior changes.

---

### ✅ **J. RECONCILIATION ENFORCEMENT** ⭐ **KEYSTONE**

**Questions:**
1. Show where commits require acknowledgment from all mirrors before becoming canonical.
2. Prove that authority expires deterministically (no "stays open" bug).
3. Confirm that semantic state (prompts/specs) requires round-trip confirmation.
4. Show HOLD state when reconciliation times out.
5. Prove that no downstream action can proceed on unreconciled commits.

**Expected answer:**
```rust
// 1. Commits require acknowledgment
fn commit_is_canonical(commit: &Commit) -> bool {
    commit.acknowledgments.len() == commit.required_mirrors.len()
        && commit.status == CommitStatus::Reconciled
}

// 2. Authority expires deterministically
fn authority_is_valid(delegation: &Delegation, action_commit_index: u64) -> bool {
    if let Some(expiry) = delegation.expiry_commit_index {
        if action_commit_index > expiry {
            return false;  // Expired, NOT "still open"
        }
    }
    true
}

// 3. Semantic state requires round-trip
struct SemanticState {
    canonical_surfaces: Vec<Location>,
    acknowledgments: Vec<Ack>,
    reconciled: bool,
}

// 4. HOLD state on timeout
fn wait_for_reconciliation(commit: &Commit, timeout: Duration) -> Result<()> {
    if !all_acknowledged_within(commit, timeout) {
        commit.status = CommitStatus::Hold;
        return Err("HOLD: Reconciliation timeout");
    }
    Ok(())
}

// 5. No downstream action on unreconciled
fn execute_action(commit: &Commit) -> Result<()> {
    if !commit_is_canonical(commit) {
        return Err("BLOCKED: Commit not reconciled");
    }
    // Action allowed
}
```

**FAIL if:** 
- Commits can be canonical without full acknowledgment
- Authority can remain "open" past expiry
- Semantic state doesn't require confirmation
- No HOLD state exists
- Actions proceed on unreconciled commits

**NOTE:** This is THE keystone - explains why old world fails, why Relay works.  
**See:** RECONCILIATION-PROTOCOL.md for complete explanation.

---

## 🎯 THE ONE-PAGE RULE FOR CLAUDE

> **"You are implementing, not designing. The voting system is complete. Your task is to prove you've built it correctly by answering the verification questions above with code references, schemas, and test results. If you cannot answer a question, you have NOT implemented the spec correctly. No new designs. No new vote types. No new physics. Only verification and implementation of what is locked."**

---

## 📊 IMPLEMENTATION PHASES

### **Phase 1: Canon Selection Backend (2 weeks)**
**Deliverables:**
- [ ] `CANON_VOTE` operation (reuses DDI machinery)
- [ ] `HistoryTopicFilament` type
- [ ] `SpaceObjectFilament` type
- [ ] Canon selection logic (stabilization windows)
- [ ] Branch preservation enforcement
- [ ] Tests: 10+ (canon vote, branch preservation, authority reuse)

**Files:**
```
apps/server/src/relay_physics/canon_selection.rs
apps/server/src/relay_physics/history_topic.rs
apps/server/src/relay_physics/space_object.rs
```

---

### **Phase 2: Evidence Level System (1 week)**
**Deliverables:**
- [ ] Evidence glyph metadata (◐ ✶ ◆ ✔︎)
- [ ] Claim type classification (hypothesis/faith/interpretation)
- [ ] Evidence pack schema
- [ ] Projection rules (prevent zero-evidence as fact)
- [ ] Tests: 5+ (glyph enforcement, evidence filtering)

**Files:**
```
apps/server/src/relay_physics/evidence.rs
apps/client-web/src/components/EvidenceGlyph.tsx
```

---

### **Phase 3: Thermal Field Backend (2 weeks)**
**Deliverables:**
- [ ] Five signal calculations (velocity, acceleration, authority, events, footprint)
- [ ] Four baseline computations (globe, region, topic, branch)
- [ ] Heat intensity formula
- [ ] LOD update scheduler
- [ ] SSE events for field updates
- [ ] Tests: 8+ (signal calc, baseline normalization, LOD)

**Files:**
```
apps/server/src/relay_physics/thermal_field.rs
apps/server/src/relay_physics/baseline_tracker.rs
```

---

### **Phase 4: Visualization Layer (2-3 weeks)**
**Deliverables:**
- [ ] Thermal aura shaders (GLSL)
- [ ] Force trail particle system
- [ ] Pulse ring effects
- [ ] Tension ripple shaders
- [ ] Flow pattern particles
- [ ] Tests: 5+ (rendering, LOD, no ledger writes)

**Files:**
```
apps/client-web/src/components/3d/ThermalField.tsx
apps/client-web/src/shaders/thermal_aura.glsl
apps/client-web/src/shaders/force_trails.glsl
```

---

### **Phase 5: Alert System (1 week)**
**Deliverables:**
- [ ] Four-tier alert classification
- [ ] HUD notification UI
- [ ] Camera suggestion (never forced)
- [ ] User preferences (mute/dismiss)
- [ ] Tests: 4+ (tier classification, dismissibility)

**Files:**
```
apps/client-web/src/components/AlertSystem.tsx
apps/client-web/src/hooks/useAlertTiers.ts
```

---

### **Phase 6: HUD Training Integration (1 week)**
**Deliverables:**
- [ ] SELECT → HOLD overlay
- [ ] "Why it's hot" explanation generator
- [ ] Recent changes timeline
- [ ] "What could happen next" predictor
- [ ] Tests: 3+ (overlay content, explanations)

**Files:**
```
apps/client-web/src/components/HUD/TurbulenceExplainer.tsx
```

---

## 🔒 SEVEN FINAL CONFIRMATIONS

**The system architect confirms:**

1. ✅ **Community belief governs the operating state**  
   → Not hidden truth, but visible reliance choice

2. ✅ **Votes collapse uncertainty into material coordination**  
   → Like quantum measurement, but for social systems

3. ✅ **State must be visible and readable in 3D at all times**  
   → No dashboards, no analytics, just physics

4. ✅ **Turbulence is significance, not noise**  
   → Heat shows what matters, force shows urgency

5. ✅ **Rendering is projection; ledger remains exact**  
   → Field never writes back, LOD only affects display

6. ✅ **Belief filaments derive from evidence filaments**  
   → Evidence immutable, belief vote-selected

7. ✅ **Hot movement must alert observers physically**  
   → Four tiers, contextual, dismissible

---

## 🎉 COMPLETION STATEMENT

**Relay's voting system doesn't just decide — it moves, pressurizes, and signals urgency like a living system.**

**Design phase:** ✅ **COMPLETE**  
**Conceptual work:** ✅ **COMPLETE**  
**Specs locked:** ✅ **3 DOCUMENTS**  
**Tests defined:** ✅ **35+ REQUIRED**  
**Implementation phases:** ✅ **6 PHASES, 8-10 WEEKS**

**Next action:**  
→ Send verification questions to Claude  
→ Demand proof of correct implementation  
→ No new designs accepted  
→ Only verification and coding

---

## 📚 REFERENCE DOCUMENTS

**All locked specs:**
1. `relay/reference/VOTING-SYSTEM-ALIGNMENT.md`
2. `relay/reference/VOTE-TURBULENCE-VISUALIZATION-SPEC.md`
3. `relay/reference/GLOBE-TIME-SPACE-MODEL.md`
4. `documentation/VOTING/ELECTION-SYSTEM.md`
5. `relay/filaments/architecture/0008_delegated_decaying_influence.md`
6. `AUTHORITY-DELEGATION-COMPLETE.md`

**Context table:**
- `relay/reference/CONTEXT-TABLE.json` (updated with all new docs)

---

## 🔐 SIGNATURE

**Locked by:** System Architect  
**Date:** 2026-01-29  
**Status:** 🔒 **IMMUTABLE**  
**Supersedes:** None (completion certificate)  
**Next:** Implementation verification only

---

**🎯 VOTING SYSTEM = COMPLETE**

**"Everything is allowed to exist.  
Not everything is allowed to silently become default."**

**That's the difference between chaos and freedom.**

---

**END OF COMPLETION CERTIFICATE**

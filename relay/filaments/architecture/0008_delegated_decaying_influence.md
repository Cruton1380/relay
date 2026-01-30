# COMMIT 8: Delegated Decaying Influence (The Fourth Primitive)

**Filament:** `architecture`  
**Commit Index:** 8  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** COORDINATION_PRIMITIVE

---

## INVARIANT LOCKED

**Authority is not granted once. It is borrowed continuously.**

Relay encodes this as physics, not ethics.

---

## THE HINGE CONCEPT

> "We vote on you this much, you decide how to use our voting power."

This is not metaphorical. This is a **delegation primitive** — and it's the missing bridge between voting, authority, and action.

---

## PART 1: WHAT IS DELEGATED DECAYING INFLUENCE?

### Precise Definition

**Delegated Decaying Influence (DDI)** is:
- **Time-bounded** (expires automatically)
- **Purpose-scoped** (limited to specific domains)
- **Decay-governed** (fades unless renewed)
- **Temporary authority capacity** (not permanent delegation)

**In one sentence:**  
Voters lend legitimacy to an agent, who can spend that legitimacy to act on their behalf, but only while support is actively maintained.

---

### What It Is NOT

❌ **Not permanent delegation**  
Authority expires automatically (no eternal representatives)

❌ **Not transferable tokens**  
You can't sell or trade delegated influence

❌ **Not representatives "owning" votes**  
The influence decays back to zero without renewal

❌ **Not liquid democracy**  
Votes aren't transferred; capacity is granted

---

## PART 2: THE CORE MECHANISM (STEP-BY-STEP)

### Step 1: A Group Expresses Support

```
100 voters → Vote for Agent Alice
```

**Commits:**
- `VOTE_CAST` (from each voter)
- Target: `unit.alice.001`
- Weight: 1.0 (or variable)

---

### Step 2: Support Crystallizes Into Influence

```
100 votes → Alice gains 100 influence capacity
```

**Not as:**
- Money (Alice doesn't "own" anything)
- Ownership (Alice doesn't control voters)

**But as:**
- **Temporary authority capacity** (Alice can act on voters' behalf)
- **Borrowed legitimacy** (Alice is trusted, not empowered)

---

### Step 3: Influence Decays Over Time

```
Day 0: 100 influence
Day 1: 98 influence (-2% per day)
Day 7: 87 influence
Day 30: 55 influence
```

**Unless renewed:**
- Voters cast `VOTE_REFRESH` commits
- Influence resets to 100 (or increases)

**Why decay is essential:**
- Without decay: delegation becomes capture
- Without decay: early movers dominate forever
- Without decay: authority ossifies

**With decay:**
- Influence requires maintenance
- Authority must remain aligned with current support
- Legitimacy is continuously tested, not assumed

**Decay turns voting from a decision into a relationship.**

---

### Step 4: The Recipient Spends Influence

Alice can spend influence to:
- **Propose actions** (create commits)
- **Prioritize tasks** (reorder work queues)
- **Unblock commits** (override lower-priority blocks)
- **Allocate escrow** (release funds under conditions)
- **Act on behalf of the group** (execute decisions)

**All spending is visible and attributable:**
```
"Alice spent 12% influence to approve PR #42"
"Alice spent 5% influence to allocate $10K to Task X"
```

**Constraints:**
- ✅ Alice cannot spend more than current influence
- ✅ Each action has an influence cost (prevents spam)
- ✅ Spending drains influence (must be replenished)
- ✅ All spending is logged (forensic audit trail)

---

### Step 5: Influence Exhausts (or Expires)

```
Alice spends 95% influence → Only 5% remains
Alice cannot act (insufficient influence)
```

**Two paths:**
1. **Voters renew support** → Alice regains influence
2. **Voters withdraw support** → Alice loses authority

**This is not representation. This is borrowed legitimacy.**

---

## PART 3: WHY THIS IS DIFFERENT FROM LIQUID DEMOCRACY

| Property | Liquid Democracy | Relay DDI |
|----------|------------------|-----------|
| **Votes** | Transferred | Capacity granted |
| **Duration** | Permanent until revoked | Automatically decays |
| **Delegation** | Binary (all or nothing) | Continuous scalar (0-100%) |
| **Action** | Delegate votes for you | Delegate authority to act |
| **Auditability** | Hard to trace | Fully logged + replayable |
| **Capture** | High risk (ossified power) | Low risk (decay forces renewal) |
| **Revocation** | Reactive (must explicitly revoke) | Proactive (decay by default) |

**Key difference:**  
Liquid democracy **transfers votes**.  
Relay DDI **grants temporary authority capacity**.

---

## PART 4: FILAMENT SCHEMA (DELEGATION FILAMENT)

### Filament Type: `Delegation`

```rust
FilamentType::Delegation {
    delegation_type: DelegationType,
    decay_policy: DecayPolicy,
}

enum DelegationType {
    VoteInfluence,       // Voting-derived authority
    CommitmentAuthority, // Promise-derived authority
    ResourceControl,     // Escrow-derived authority
}

struct DecayPolicy {
    decay_function: DecayFunction,
    decay_rate: f32,          // e.g., 0.02 = 2% per day
    half_life: Option<Duration>, // Alternative to rate
    min_threshold: f32,       // Below this, influence = 0
}

enum DecayFunction {
    Linear,       // Constant rate
    Exponential,  // Accelerating decay
    Step,         // Drops at intervals
    Logistic,     // Slow → fast → slow (S-curve)
}
```

---

### Commit Operations

#### 1. `DELEGATION_CREATED`

```json
{
  "filament_id": "delegation.alice.001",
  "op_type": "DELEGATION_CREATED",
  "author_unit_ref": "unit.voter_pool.001",
  "payload": {
    "from": "unit.voter_pool.001",
    "to": "unit.alice.001",
    "amount": 100.0,
    "scope": "climate.policy.001",
    "decay_function": "Linear",
    "decay_rate": 0.02,
    "duration_max": "30d",
    "revocable": true
  },
  "causal_refs": {
    "inputs": [
      "vote.alice@c1",
      "vote.alice@c2",
      "..." 
    ],
    "authority_ref": "unit.voter_pool.001@vote_power"
  }
}
```

---

#### 2. `DELEGATION_REFRESHED`

```json
{
  "filament_id": "delegation.alice.001",
  "op_type": "DELEGATION_REFRESHED",
  "author_unit_ref": "unit.voter_pool.001",
  "payload": {
    "new_amount": 120.0,
    "reset_decay": true
  },
  "causal_refs": {
    "inputs": ["delegation.alice.001@c1"]
  }
}
```

**Effect:** Resets decay timer or increases influence.

---

#### 3. `DELEGATION_DECAYED` (System Event)

```json
{
  "filament_id": "delegation.alice.001",
  "op_type": "DELEGATION_DECAYED",
  "author_unit_ref": "system.decay_engine",
  "payload": {
    "previous_amount": 100.0,
    "new_amount": 98.0,
    "decay_applied": 2.0,
    "time_elapsed": "1d"
  },
  "causal_refs": {
    "inputs": ["delegation.alice.001@c1"]
  }
}
```

**Frequency:** System emits decay events periodically (deterministic).

---

#### 4. `DELEGATION_SPENT`

```json
{
  "filament_id": "delegation.alice.001",
  "op_type": "DELEGATION_SPENT",
  "author_unit_ref": "unit.alice.001",
  "payload": {
    "action": "APPROVE_PR",
    "action_ref": "climate.policy.001@c42",
    "influence_cost": 12.0,
    "remaining_influence": 86.0,
    "reason": "Approve PR #42: Carbon Tax Implementation"
  },
  "causal_refs": {
    "inputs": ["delegation.alice.001@c3"],
    "authority_ref": "delegation.alice.001"
  }
}
```

**Effect:** Drains influence by `influence_cost`.

---

#### 5. `DELEGATION_EXHAUSTED`

```json
{
  "filament_id": "delegation.alice.001",
  "op_type": "DELEGATION_EXHAUSTED",
  "author_unit_ref": "system.delegation_engine",
  "payload": {
    "final_amount": 0.0,
    "reason": "Decayed to zero"
  },
  "causal_refs": {
    "inputs": ["delegation.alice.001@c10"]
  }
}
```

**Effect:** Alice can no longer act using this delegation.

---

## PART 5: DECAY MATH (WHY LINEAR ≠ EXPONENTIAL)

### Linear Decay (Constant Rate)

```
Influence(t) = Initial - (rate * t)
```

**Example:** 100 influence, -2% per day
```
Day 0: 100
Day 1: 98
Day 7: 86
Day 30: 40
Day 50: 0
```

**Pros:**
- ✅ Predictable (easy to understand)
- ✅ Fair (everyone decays at same rate)

**Cons:**
- ❌ No urgency (constant pressure)

**Use case:** Default for most delegations

---

### Exponential Decay (Accelerating)

```
Influence(t) = Initial * e^(-rate * t)
```

**Example:** 100 influence, half-life = 7 days
```
Day 0: 100
Day 7: 50
Day 14: 25
Day 21: 12.5
Day 30: 5.7
```

**Pros:**
- ✅ Creates urgency (fast decline near end)
- ✅ Never reaches exactly zero (asymptotic)

**Cons:**
- ❌ Harder to predict
- ❌ Can feel punitive

**Use case:** High-stakes, time-sensitive delegations

---

### Step Decay (Drops at Intervals)

```
Influence(t) = Initial * floor(1 - t / interval)
```

**Example:** 100 influence, drops 20% every 7 days
```
Day 0-6: 100
Day 7-13: 80
Day 14-20: 60
Day 21-27: 40
Day 28-34: 20
Day 35+: 0
```

**Pros:**
- ✅ Clear milestones (renewal dates obvious)
- ✅ Less stressful (grace periods)

**Cons:**
- ❌ Gameable (act just before drop)

**Use case:** Long-term strategic delegations

---

### Logistic Decay (S-Curve)

```
Influence(t) = Initial / (1 + e^(rate * (t - midpoint)))
```

**Example:** 100 influence, inflection at 15 days
```
Day 0-10: 100 → 95 (slow decay)
Day 10-20: 95 → 50 (rapid decay)
Day 20-30: 50 → 5 (slow decay)
```

**Pros:**
- ✅ Natural (mimics trust erosion)
- ✅ Balances urgency + grace period

**Cons:**
- ❌ Complex (harder to explain)

**Use case:** Adaptive, relationship-based delegations

---

### **LOCKED DECISION: Default = Linear, Optional Others**

- **Default:** Linear decay (simple, predictable)
- **Optional:** Exponential, Step, Logistic (for advanced use cases)
- **Configurable:** Decay rate, half-life, min threshold

---

## PART 6: INFLUENCE COST MODEL (ACTION → COST MAPPING)

### Principle: More Impactful Actions Cost More Influence

| Action | Influence Cost | Rationale |
|--------|---------------|-----------|
| **Comment on proposal** | 0% | No authority needed (free speech) |
| **Vote on proposal** | 0% | Voting is free (legitimacy signal) |
| **Propose new action** | 5% | Commitment to see it through |
| **Approve PR** | 10-20% | High-stakes decision |
| **Allocate escrow** | 20-50% | Financial responsibility |
| **Override dispute** | 50-80% | Breaking consensus |
| **Emergency action** | 80-100% | All-in commitment |

**Cost formula:**
```rust
influence_cost = base_cost * scope_multiplier * urgency_multiplier

where:
  base_cost = action type (5%, 10%, 20%, etc.)
  scope_multiplier = 1.0 (local) to 3.0 (global)
  urgency_multiplier = 1.0 (normal) to 2.0 (emergency)
```

---

### Example: Alice Approves a PR

```
Action: Approve PR #42
Base cost: 10%
Scope: Local (climate.policy.001) → multiplier = 1.0
Urgency: Normal → multiplier = 1.0
Total cost: 10% * 1.0 * 1.0 = 10%

Alice's influence: 100 → 90 (after approval)
```

---

## PART 7: HARD LIMITS (PREVENTING OVER-LEVERAGE)

### Limit 1: No Double-Spending

**Rule:** You cannot spend influence on multiple actions simultaneously if total cost > current influence.

**Example:**
```
Alice has 50% influence
Alice tries to approve PR #42 (30%) AND PR #43 (30%)
Result: BLOCKED (30% + 30% = 60% > 50%)
```

**Enforcement:** Commits must reference current delegation state (causal ref).

---

### Limit 2: No Fractional Reserve

**Rule:** Delegated influence cannot be "loaned out" or re-delegated at leverage.

**Example:**
```
Alice has 100% delegated influence from voters
Alice CANNOT re-delegate 200% to Bob (fractional reserve)
Alice can only re-delegate up to 100% (full reserve)
```

**Enforcement:** `DELEGATION_CREATED` must reference source delegation (no phantom influence).

---

### Limit 3: Scope Boundaries

**Rule:** Influence is scoped to specific domains. Cannot be used outside scope.

**Example:**
```
Alice has 100% influence in "climate.policy.001"
Alice CANNOT use this to approve actions in "defense.budget.002"
```

**Enforcement:** `DELEGATION_SPENT` must have `action_ref` within delegation scope.

---

### Limit 4: Minimum Influence Threshold

**Rule:** Below a minimum threshold (e.g., 5%), influence rounds to zero.

**Why:** Prevents "zombie delegations" with 0.01% influence.

**Example:**
```
Alice's influence decays to 3%
System: DELEGATION_EXHAUSTED (below 5% threshold)
Alice must regain significant support to act again
```

---

## PART 8: STARCRAFT HUD INTEGRATION

### Delegated Influence Gauge (New)

```
┌─────────────────────────────────────────────────────┐
│ AGENT: Alice (unit.alice.001)                       │
│                                                     │
│ 🟦 Legitimacy      ████████░░  82% (1,240 votes)   │
│    └─ Decay: -2%/day | Next renewal: 23h          │
│                                                     │
│ 🔵 Delegated Influence  ██████░░░░  60% (decaying) │
│    └─ Scope: climate.policy.001                    │
│    └─ Sources: 100 voters → 120 influence granted  │
│    └─ Spent: 60% used (24% on PR approvals)       │
│    └─ Decay: -2%/day (renew to restore)           │
│                                                     │
│ 🟨 Commitments     ███░░░░░░  3 / 8 active         │
│                                                     │
│ [ Spend Influence ] [ Request Renewal ] [ Resign ] │
└─────────────────────────────────────────────────────┘
```

**Key visual features:**
- ✅ Influence bar drains over time (visible decay animation)
- ✅ Chunk visibly disappears when spent (e.g., approve action → -10%)
- ✅ Color shifts as influence nears zero (green → yellow → red)
- ✅ Hover shows breakdown (sources, spending history, decay rate)

---

### Spending Animation

```
User clicks: "Approve PR #42"

1. Modal appears:
   ┌─────────────────────────────────┐
   │ Approve PR #42                  │
   │                                 │
   │ Influence cost: 12%             │
   │ Your influence: 60% → 48%       │
   │                                 │
   │ [ Confirm ] [ Cancel ]          │
   └─────────────────────────────────┘

2. On confirm:
   - Influence bar animates down (60% → 48%)
   - Toast notification: "Approved PR #42 (-12% influence)"
   - SSE event: DELEGATION_SPENT

3. HUD updates:
   - Influence bar shows new value (48%)
   - Spending history shows: "PR #42: -12%"
```

---

## PART 9: WHY THIS REPLACES MONEY FOR DECISION-MAKING

### Traditional Money-Based Governance

**Question:** "Who can afford to decide?"

**Problems:**
- ❌ Richest decides (plutocracy)
- ❌ Money can be hoarded (concentration)
- ❌ Money can be hidden (opacity)
- ❌ Money can be bought (corruption)

---

### Relay Delegated Influence

**Question:** "Who is currently trusted to decide — and how much?"

**Advantages:**
- ✅ You can't hoard influence (it decays)
- ✅ You can't buy it (it's granted by voters)
- ✅ You can't hide its use (all spending logged)
- ✅ You can't keep it without ongoing support (decay forces renewal)

**Result:** Far closer to democratic legitimacy than cash.

---

## PART 10: COMPARISON TO OTHER SYSTEMS

| System | Authority Model | Relay DDI Equivalent |
|--------|----------------|---------------------|
| **Representative Democracy** | Elected once, serves term | Continuous decay, must maintain support |
| **Liquid Democracy** | Transfer votes | Grant temporary influence |
| **Plutocracy** | Money = power | Influence ≠ money (can't buy) |
| **Meritocracy** | Earned credentials | Commitment capacity (track record) |
| **Anarchy** | No delegation | Optional (can choose not to delegate) |

**Key difference:** Relay doesn't choose one model. It makes all models **explicit tradeoffs** using orthogonal gauges.

---

## PART 11: LOCKED INVARIANTS (DELEGATED INFLUENCE)

1. ✅ **Influence is borrowed, not granted** (temporary authority)
2. ✅ **Influence decays** (requires continuous renewal)
3. ✅ **Influence is scoped** (limited to specific domains)
4. ✅ **Spending is visible** (all actions logged)
5. ✅ **No double-spending** (causal refs prevent over-commitment)
6. ✅ **No fractional reserve** (full-reserve delegation only)
7. ✅ **Influence ≠ money** (can't be bought, sold, or hoarded)
8. ✅ **Exhaustion is explicit** (below threshold → delegation ends)

---

## PART 12: IMPLEMENTATION ROADMAP

### Phase 1: Basic Delegation (MVP)
**Deliverables:**
1. `DELEGATION_CREATED`, `DELEGATION_SPENT`, `DELEGATION_EXHAUSTED` commits
2. Linear decay calculation (system events)
3. Influence balance tracking (derived from commits)

**Files:**
- `apps/server/src/relay_physics/delegation_store.rs`
- `apps/server/src/relay_physics/agent_ops.rs` (add operations)

**Time:** 4-6 hours

---

### Phase 2: Decay Engine
**Deliverables:**
1. System service that emits `DELEGATION_DECAYED` events
2. Configurable decay functions (linear, exponential, step)
3. Min threshold enforcement (auto-exhaustion)

**Files:**
- `apps/server/src/relay_physics/decay_engine.rs`

**Time:** 3-4 hours

---

### Phase 3: Influence Cost Model
**Deliverables:**
1. Action → cost mapping (configurable)
2. Scope multiplier (local vs global)
3. Urgency multiplier (normal vs emergency)

**Files:**
- `apps/server/src/relay_physics/influence_costs.rs`

**Time:** 2-3 hours

---

### Phase 4: HUD Integration
**Deliverables:**
1. Delegated influence gauge (React component)
2. Spending animation (visual feedback)
3. Renewal requests (voters can see requests)

**Files:**
- `apps/client-web/src/components/DelegatedInfluenceGauge.tsx`

**Time:** 3-4 hours

---

## PART 13: CAUSAL REFS

- **Inputs:** `architecture@c6` (Economic primitives), `architecture@c7` (Coordination gauges)
- **Authority:** system.architect
- **Evidence:**
  - Liquid democracy literature (critique of permanent delegation)
  - Time-based authority decay (precedent: OAuth tokens, session timeouts)
  - Democratic legitimacy theory (continuous consent vs one-time election)

---

## NEXT STEPS

### Immediate
1. Add `FilamentType::Delegation` to type system
2. Implement basic delegation commits (`DELEGATION_CREATED`, `DELEGATION_SPENT`)
3. Add decay calculation (system events)

### Short-Term
1. Build decay engine (automated `DELEGATION_DECAYED` events)
2. Add influence cost model (action → cost mapping)
3. Build HUD gauge (visual representation)

---

**Status:** LOCKED  
**Supersedes:** None (fourth coordination primitive)  
**Next:** Implement delegation filament + decay engine

---

**Philosophy:**  
Authority is not granted once. It is borrowed continuously.  
Relay encodes this as **physics**, not **ethics**.  
**Voting becomes a relationship, not a decision.**

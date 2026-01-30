# Relay Coordination Substrate - GOLD STANDARD DOCUMENTATION

**Version:** 1.0  
**Date:** 2026-01-28  
**Status:** Architecture Locked (`architecture@c6`, `c7`, `c8`)  
**Purpose:** Comprehensive implementation guide for Relay's coordination primitives

---

## TABLE OF CONTENTS

1. [Executive Summary](#executive-summary)
2. [The Four Coordination Primitives](#the-four-coordination-primitives)
3. [The StarCraft HUD Model](#the-starcraft-hud-model)
4. [Delegated Decaying Influence](#delegated-decaying-influence)
5. [Implementation Roadmap](#implementation-roadmap)
6. [Visual Design Specifications](#visual-design-specifications)
7. [Technical Architecture](#technical-architecture)
8. [Testing & Validation](#testing--validation)
9. [Philosophical Foundations](#philosophical-foundations)

---

## EXECUTIVE SUMMARY

### What Is Relay's Coordination Substrate?

Relay is not a blockchain. It is not a payment system. It is not a voting platform.

**Relay is a coordination substrate** — a truthful, deterministic, traceable foundation for human and AI cooperation.

### The Core Transformation

**Traditional systems:**
- Money answers: "Who can afford to decide?"
- Voting answers: "Who was chosen once?"
- Authority answers: "Who was granted power?"

**Relay:**
- **Money** = Transferable authority tokens (transparent, full-reserve)
- **Voting** = Attention signals (abundant, positive-sum)
- **Authority** = Borrowed legitimacy (temporary, decaying)
- **Commitments** = Staked reputation (binding, visible)

### The Key Insight

**Truth is the substrate. Coordination primitives are data on that substrate.**

---

## THE FOUR COORDINATION PRIMITIVES

### 1. Authority Tokens (Money)

**Definition:** Scarce, transferable, zero-sum resources.

**Properties:**
- ✅ Finite supply (no fractional reserve)
- ✅ Transferable (can be sent)
- ✅ Zero-sum (my loss = your gain)
- ✅ Transparent (all transactions visible)

**Filament Type:** `FilamentType::Currency`

**Operations:**
- `CURRENCY_ISSUE` - Mint new tokens
- `CURRENCY_TRANSFER` - Send tokens
- `CURRENCY_BURN` - Destroy tokens
- `CURRENCY_DELEGATE` - Lend tokens (credit)
- `CURRENCY_REVOKE` - Cancel loan

**Use cases:**
- Pay for compute (gas fees)
- Escrow for high-stakes decisions
- Community currencies
- Central bank digital currencies

**StarCraft analog:** Minerals / Gas (spendable resources)

---

### 2. Attention Signals (Voting)

**Definition:** Abundant, non-transferable, positive-sum coordination signals.

**Properties:**
- ✅ Unlimited (not scarce)
- ✅ Non-transferable (can't be bought/sold)
- ✅ Positive-sum (both can gain)
- ✅ Decaying (requires maintenance)

**Filament Type:** `FilamentType::Vote`

**Operations:**
- `VOTE_CAST` - Signal support
- `VOTE_REFRESH` - Renew support
- `VOTE_RETRACT` - Withdraw support
- `VOTE_DECAY` (system) - Automatic decay

**Use cases:**
- Signal legitimacy (who has support?)
- Prioritize tasks (what's important?)
- Allocate attention (where to focus?)

**StarCraft analog:** Control / Influence (map presence)

**Key difference from money:** Voting doesn't deplete your capacity. You can vote for many things.

---

### 3. Reputation (Merit)

**Definition:** Scarce, non-transferable, merit-based authority.

**Properties:**
- ✅ Scarce (hard to earn)
- ✅ Non-transferable (soulbound)
- ✅ Merit-based (earned through actions)
- ✅ Persistent (track record matters)

**Filament Type:** `FilamentType::Reputation`

**Operations:**
- `REPUTATION_EARN` - Gain reputation
- `REPUTATION_SLASH` - Lose reputation (penalty)
- `REPUTATION_AUDIT` - Inspect history

**Use cases:**
- Commitment capacity (how many promises you can make)
- Trust signals (who is reliable?)
- Authority thresholds (who can propose high-stakes actions?)

**StarCraft analog:** Unit experience / rank

---

### 4. Commitments (Promises)

**Definition:** Binding, visible, enforceable obligations.

**Properties:**
- ✅ Binding (can't be erased)
- ✅ Visible (all commitments public)
- ✅ Enforceable (penalties for breach)
- ✅ Time-bounded (deadlines)

**Filament Type:** `FilamentType::Commitment`

**Operations:**
- `COMMITMENT_PROMISE` - Make binding promise
- `COMMITMENT_FULFILL` - Complete promise
- `COMMITMENT_BREACH` - Fail to complete (penalty)
- `COMMITMENT_REVOKE` - Cancel (with penalty)

**Use cases:**
- Service level agreements (uptime guarantees)
- Delivery promises (deadlines)
- Code review commitments
- Maintenance obligations

**Penalties:**
- Reputation slash (lose trust)
- Financial burn (destroy escrowed tokens)
- Authority revoke (lose delegated power)
- Public shame (visible scar)

**Key innovation:** Stake reputation or time, not just money.

---

## THE STARCRAFT HUD MODEL

### Why StarCraft HUD?

StarCraft already solved the "multiple resources" problem 25 years ago.

**StarCraft doesn't show a wallet. It shows what you can do next.**

Relay does the same: **coordination resources as orthogonal gauges, not one currency.**

---

### The Five Canonical Gauges

#### 🟦 GAUGE 1: LEGITIMACY (Votes / Support)

**What it shows:**  
Current, maintained support for an entity, proposal, or filament.

**Visual:**
```
🟦 Legitimacy     ████████░░  82% (1,240 votes, -2%/day)
   └─ Next renewal: 23h
```

**Properties:**
- Non-spendable (doesn't deplete)
- Decaying (requires renewal)
- Scoped (local / regional / global)

**Color coding:**
- 🟢 Green: 70-100% (strong support)
- 🟡 Yellow: 30-70% (moderate support)
- 🔴 Red: 0-30% (weak support)

**Events that affect:**
- `VOTE_CAST` → +legitimacy
- `VOTE_RETRACT` → -legitimacy
- `VOTE_DECAY` → slow drain
- `COMMITMENT_BREACH` → reputation hit

---

#### 🟨 GAUGE 2: COMMITMENT CAPACITY

**What it shows:**  
How many active commitments an agent can carry (obligation bandwidth).

**Visual:**
```
🟨 Commitments    ███░░░░░░  3 / 8 active
   └─ "Fix bug #42" (ends 2d), "Review PR" (4h)
```

**Properties:**
- Scarce (limited slots)
- Merit-based (increases with fulfillments)
- Hard cap (prevents infinite promises)

**Color coding:**
- 🟢 Green: 0-60% used (capacity available)
- 🟡 Yellow: 60-90% used (stretched)
- 🔴 Red: 90-100% used (overcommitted)

**Events that affect:**
- `COMMITMENT_PROMISE` → uses slot
- `COMMITMENT_FULFILL` → frees slot (+1 max eventually)
- `COMMITMENT_BREACH` → loses max capacity

---

#### 🟩 GAUGE 3: ESCROWED RESOURCES

**What it shows:**  
Real-world scarce resources locked under governance rules.

**Visual:**
```
🟩 Escrow         ⛓ $1.2M locked (2 conditions)
   └─ Release: 80% vote OR 2026-02-15
```

**Properties:**
- Scarce (finite)
- Transferable (can move)
- Zero-sum (locked = unavailable)
- Conditional (rules govern release)

**Color coding:**
- 🟢 Green: < $100K (low stakes)
- 🟡 Yellow: $100K - $1M (medium stakes)
- 🔴 Red: > $1M (high stakes)

**Events that affect:**
- `ESCROW_LOCK` → freezes tokens
- `ESCROW_RELEASE` → unlocks tokens
- `CURRENCY_BURN` → destroys tokens

---

#### 🟪 GAUGE 4: TIME / COOLDOWN PRESSURE

**What it shows:**  
Irreversible time commitments, vote durations, revocation windows.

**Visual:**
```
🟪 Time           ⏳ Vote ends in 2d 4h 23m
   └─ Cooldown: Next proposal in 6h
```

**Properties:**
- Universal (affects everyone)
- Non-transferable (can't buy time)
- Deterministic (countdowns precise)

**Color coding:**
- 🟢 Green: > 7 days remaining
- 🟡 Yellow: 1-7 days remaining
- 🔴 Red: < 1 day remaining

**Events that affect:**
- `VOTE_OPEN` → starts countdown
- `VOTE_CLOSE` → ends countdown
- `COOLDOWN_ACTIVE` → blocks action

---

#### 🟥 GAUGE 5: RISK / SCAR / DISPUTE PRESSURE

**What it shows:**  
Unresolved disputes, rejected commits, authority challenges.

**Visual:**
```
🟥 Disputes       ⚠ 1 active (click to inspect)
   └─ Scar: OUTPUT_REJECTED at commit@c42
```

**Properties:**
- Permanent (scars don't fade)
- Visible (can't be hidden)
- Attributable (linked to commits)
- Forensic (click to inspect)

**Color coding:**
- 🟢 Green: 0 scars (clean history)
- 🟡 Yellow: 1-3 scars (some friction)
- 🔴 Red: 4+ scars (contested actor)

**Events that affect:**
- `CommitRejected` → creates scar
- `DISPUTE_OPEN` → raises warning
- `DISPUTE_RESOLVED` → marks resolved (scar remains)

---

### HUD Layout (Complete)

```
┌──────────────────────────────────────────────────────────┐
│ 🌍 GLOBAL CHANNEL: climate.policy.001                    │
│                                                          │
│ 🟦 Legitimacy       ████████░░  82% (1,240 votes)       │
│    └─ Decay: -2%/day | Next renewal: 23h               │
│                                                          │
│ 🟨 Commitments      ███░░░░░░  3 / 8 active             │
│    └─ "Fix bug #42" (ends 2d), "Review PR" (4h)        │
│                                                          │
│ 🟩 Escrow           ⛓ $1.2M locked (2 conditions)       │
│    └─ Release: 80% vote OR 2026-02-15                  │
│                                                          │
│ 🟪 Time             ⏳ Vote ends in 2d 4h 23m            │
│    └─ Cooldown: Next proposal in 6h                    │
│                                                          │
│ 🟥 Disputes         ⚠ 1 active (click to inspect)       │
│    └─ Scar: OUTPUT_REJECTED at commit@c42              │
│                                                          │
│ [ View Filament ] [ Forensic Chamber ] [ Vote Now ]     │
└──────────────────────────────────────────────────────────┘
```

**Position:** Top-right corner (always visible)  
**Size:** 400px wide, auto height  
**Updates:** Live via SSE (no refresh needed)

---

## DELEGATED DECAYING INFLUENCE

### The Core Concept

> "We vote on you this much, you decide how to use our voting power."

This is **not metaphorical**. It's a first-class primitive.

---

### What Is DDI?

**Delegated Decaying Influence (DDI)** is:
- **Time-bounded** authority (expires automatically)
- **Purpose-scoped** power (limited to specific domains)
- **Decay-governed** legitimacy (fades unless renewed)
- **Borrowed capacity** (not permanent delegation)

**In one sentence:**  
Voters lend legitimacy to an agent, who can spend that legitimacy to act on their behalf, but only while support is actively maintained.

---

### How It Works (5 Steps)

#### Step 1: Voters Express Support

```
100 voters → Vote for Agent Alice
```

**Commits:** 100x `VOTE_CAST` (target: `unit.alice.001`)

---

#### Step 2: Support Crystallizes Into Influence

```
100 votes → Alice gains 100 influence capacity
```

**Not:**
- Money (Alice doesn't "own" anything)
- Ownership (Alice doesn't control voters)

**But:**
- Temporary authority capacity
- Borrowed legitimacy

---

#### Step 3: Influence Decays Over Time

```
Day 0: 100 influence
Day 1: 98 influence (-2% per day)
Day 7: 87 influence
Day 30: 55 influence
```

**Unless renewed:** Voters cast `VOTE_REFRESH` commits.

**Why decay matters:**
- Without decay → delegation becomes capture
- With decay → authority must be maintained

**Decay turns voting from a decision into a relationship.**

---

#### Step 4: Agent Spends Influence

Alice can spend influence to:
- Approve actions (e.g., merge PR)
- Allocate escrow (e.g., release funds)
- Override disputes (e.g., break deadlock)

**All spending is visible:**
```
"Alice spent 12% influence to approve PR #42"
```

**Constraints:**
- ✅ Cannot spend more than current influence
- ✅ Each action has a cost (prevents spam)
- ✅ Spending drains influence (must be replenished)
- ✅ All spending is logged (forensic audit)

---

#### Step 5: Influence Exhausts (or Expires)

```
Alice spends 95% → Only 5% remains
Alice cannot act (insufficient influence)
```

**Two paths:**
1. Voters renew support → Alice regains influence
2. Voters withdraw support → Alice loses authority

---

### Influence Cost Model

| Action | Influence Cost | Rationale |
|--------|---------------|-----------|
| **Comment** | 0% | No authority needed |
| **Vote** | 0% | Voting is free |
| **Propose** | 5% | Commitment required |
| **Approve PR** | 10-20% | High-stakes decision |
| **Allocate escrow** | 20-50% | Financial responsibility |
| **Override dispute** | 50-80% | Breaking consensus |
| **Emergency action** | 80-100% | All-in commitment |

**Formula:**
```
cost = base_cost * scope_multiplier * urgency_multiplier
```

---

### Decay Functions

#### Linear Decay (Default)
```
Influence(t) = Initial - (rate * t)
```

**Example:** -2% per day
```
Day 0: 100%
Day 7: 86%
Day 30: 40%
Day 50: 0%
```

**Pros:** Predictable, fair  
**Use case:** Default for most delegations

---

#### Exponential Decay
```
Influence(t) = Initial * e^(-rate * t)
```

**Example:** Half-life = 7 days
```
Day 0: 100%
Day 7: 50%
Day 14: 25%
Day 30: 5.7%
```

**Pros:** Creates urgency  
**Use case:** Time-sensitive high-stakes decisions

---

#### Step Decay
```
Influence(t) = Initial * floor(1 - t / interval)
```

**Example:** -20% every 7 days
```
Day 0-6: 100%
Day 7-13: 80%
Day 14-20: 60%
Day 35+: 0%
```

**Pros:** Clear renewal deadlines  
**Use case:** Long-term strategic delegations

---

### HUD Integration (Delegated Influence Gauge)

```
┌──────────────────────────────────────────────────────────┐
│ AGENT: Alice (unit.alice.001)                            │
│                                                          │
│ 🔵 Delegated Influence  ██████░░░░  60% (decaying)      │
│    └─ Scope: climate.policy.001                         │
│    └─ Sources: 100 voters → 120 influence granted       │
│    └─ Spent: 60% used (24% on PR approvals)            │
│    └─ Decay: -2%/day (renew to restore)                │
│                                                          │
│ Recent spending:                                         │
│    • PR #42 approved: -12% (2 hours ago)                │
│    • Budget allocation: -18% (1 day ago)                │
│                                                          │
│ [ Spend Influence ] [ Request Renewal ] [ Resign ]      │
└──────────────────────────────────────────────────────────┘
```

**Key visuals:**
- ✅ Bar drains over time (visible decay animation)
- ✅ Chunk disappears when spent (e.g., -10% on action)
- ✅ Color shifts near zero (green → yellow → red)
- ✅ Hover shows breakdown (sources, history, decay rate)

---

## IMPLEMENTATION ROADMAP

### Phase 1: Economic Primitives (PR #2)
**Time:** 4-6 hours

**Deliverables:**
1. `FilamentType::Currency`
2. `CURRENCY_ISSUE`, `CURRENCY_TRANSFER`, `CURRENCY_BURN`, `CURRENCY_DELEGATE`
3. Balance tracking (derived from commits)
4. Unit tests

**Files:**
- `apps/server/src/relay_physics/currency_store.rs`
- `apps/server/src/relay_physics/agent_ops.rs` (add operations)

---

### Phase 2: Voting Primitives (PR #3)
**Time:** 3-4 hours

**Deliverables:**
1. `FilamentType::Vote`
2. `VOTE_CAST`, `VOTE_REFRESH`, `VOTE_RETRACT`, `VOTE_DECAY`
3. Legitimacy calculation (vote count + decay)
4. Unit tests

**Files:**
- `apps/server/src/relay_physics/vote_store.rs`

---

### Phase 3: Commitment Primitives (PR #4)
**Time:** 4-5 hours

**Deliverables:**
1. `FilamentType::Commitment`
2. `COMMITMENT_PROMISE`, `COMMITMENT_FULFILL`, `COMMITMENT_BREACH`
3. Capacity tracking (active / max)
4. Penalty enforcement
5. Unit tests

**Files:**
- `apps/server/src/relay_physics/commitment_store.rs`

---

### Phase 4: Delegated Influence (PR #5)
**Time:** 6-8 hours

**Deliverables:**
1. `FilamentType::Delegation`
2. `DELEGATION_CREATED`, `DELEGATION_SPENT`, `DELEGATION_EXHAUSTED`
3. Decay engine (system events)
4. Influence cost model
5. Unit tests

**Files:**
- `apps/server/src/relay_physics/delegation_store.rs`
- `apps/server/src/relay_physics/decay_engine.rs`
- `apps/server/src/relay_physics/influence_costs.rs`

---

### Phase 5: HUD Components (Frontend)
**Time:** 8-10 hours

**Deliverables:**
1. `CoordinationGauges.tsx` (5 gauges)
2. `DelegatedInfluenceGauge.tsx` (DDI gauge)
3. SSE event listeners (live updates)
4. Animations (bar fills, decay drains)

**Files:**
- `apps/client-web/src/components/CoordinationGauges.tsx`
- `apps/client-web/src/components/DelegatedInfluenceGauge.tsx`
- `apps/client-web/src/hooks/useCoordinationGauges.ts`

---

## VISUAL DESIGN SPECIFICATIONS

### Color Palette

```css
/* Gauge colors */
--legitimacy-color: #4A90E2;      /* Blue */
--commitment-color: #F5A623;      /* Orange */
--escrow-color: #50C878;          /* Green */
--time-color: #9B59B6;            /* Purple */
--dispute-color: #E74C3C;         /* Red */
--influence-color: #3498DB;       /* Light blue */

/* Status colors */
--status-healthy: #2ECC71;        /* Green */
--status-warning: #F39C12;        /* Yellow */
--status-danger: #E74C3C;         /* Red */

/* Decay animation */
--decay-gradient: linear-gradient(90deg, 
  var(--legitimacy-color), 
  rgba(74, 144, 226, 0.3)
);
```

---

### Typography

```css
/* Gauge labels */
.gauge-label {
  font-family: 'SF Pro Display', -apple-system, sans-serif;
  font-size: 14px;
  font-weight: 600;
  color: #2C3E50;
}

/* Gauge values */
.gauge-value {
  font-family: 'SF Mono', 'Monaco', monospace;
  font-size: 18px;
  font-weight: 700;
  color: #34495E;
}

/* Gauge details */
.gauge-detail {
  font-family: 'SF Pro Text', -apple-system, sans-serif;
  font-size: 12px;
  color: #7F8C8D;
}
```

---

### Animations

#### Decay Animation (Slow Drain)
```css
@keyframes decay-drain {
  from {
    width: 100%;
  }
  to {
    width: calc(100% - 2%); /* -2% per day */
  }
}

.gauge-bar.decaying {
  animation: decay-drain 86400s linear infinite;
}
```

---

#### Spending Animation (Instant Drop)
```css
@keyframes spend-drop {
  0% {
    width: 60%;
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
  100% {
    width: 48%; /* Dropped 12% */
    opacity: 1;
  }
}

.gauge-bar.spending {
  animation: spend-drop 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}
```

---

#### Renewal Animation (Flash + Fill)
```css
@keyframes renewal-flash {
  0%, 100% {
    box-shadow: 0 0 0 rgba(74, 144, 226, 0);
  }
  50% {
    box-shadow: 0 0 20px rgba(74, 144, 226, 0.8);
  }
}

.gauge-bar.renewing {
  animation: renewal-flash 0.8s ease-in-out;
}
```

---

## TECHNICAL ARCHITECTURE

### Layer 2 (Relay Physics) - Backend

```
apps/server/src/relay_physics/
├── currency_store.rs          (Authority tokens)
├── vote_store.rs              (Attention signals)
├── commitment_store.rs        (Promises)
├── delegation_store.rs        (DDI)
├── decay_engine.rs            (System events)
├── influence_costs.rs         (Action costs)
└── gauge_calculator.rs        (Derive HUD values)
```

---

### Layer 3 (Frontend) - UI

```
apps/client-web/src/
├── components/
│   ├── CoordinationGauges.tsx    (5 gauges)
│   ├── DelegatedInfluenceGauge.tsx (DDI)
│   ├── GaugeBar.tsx              (Reusable bar)
│   └── GaugeTooltip.tsx          (Hover details)
├── hooks/
│   ├── useCoordinationGauges.ts  (SSE listener)
│   └── useDelegatedInfluence.ts  (DDI state)
└── styles/
    └── gauges.css                (Visual styles)
```

---

### SSE Event Flow

```
Backend (Layer 2):
1. User action → Commit created
2. EventLog.append() → Assigns event_id
3. EventBus.emit_with_id() → Broadcasts event
4. SSE stream → Sends to connected clients

Frontend (Layer 3):
5. SSE listener → Receives event
6. Gauge calculator → Updates gauge values
7. React state → Triggers re-render
8. Animations → Visual feedback
```

---

## TESTING & VALIDATION

### Unit Tests (Backend)

```rust
#[test]
fn test_currency_transfer() {
    // Alice transfers 50 REL to Bob
    // Verify: Alice balance decreases, Bob balance increases
}

#[test]
fn test_vote_decay() {
    // Alice receives 100 votes
    // Fast-forward 7 days
    // Verify: Legitimacy = 86% (linear decay -2%/day)
}

#[test]
fn test_delegation_spending() {
    // Alice has 100 influence
    // Alice spends 12% on action
    // Verify: Influence = 88%
}

#[test]
fn test_commitment_capacity() {
    // Alice makes 3 promises (capacity 8)
    // Verify: Active = 3/8
    // Alice tries to make 6 more promises
    // Verify: BLOCKED (3 + 6 > 8)
}
```

---

### Integration Tests (End-to-End)

```typescript
test('Delegation flow', async () => {
  // 1. 100 voters cast votes for Alice
  await Promise.all(
    voters.map(v => api.post('/vote', { target: 'unit.alice.001' }))
  );
  
  // 2. Verify Alice gains influence
  const gauges = await api.get('/gauges/unit.alice.001');
  expect(gauges.delegated_influence).toBe(100);
  
  // 3. Alice spends influence to approve PR
  await api.post('/approve', { pr: 42, influence_cost: 12 });
  
  // 4. Verify influence drained
  const gauges2 = await api.get('/gauges/unit.alice.001');
  expect(gauges2.delegated_influence).toBe(88);
  
  // 5. Fast-forward 7 days (simulate decay)
  await timeTravel(7 * 24 * 60 * 60);
  
  // 6. Verify decay applied
  const gauges3 = await api.get('/gauges/unit.alice.001');
  expect(gauges3.delegated_influence).toBeCloseTo(75.36); // 88 * 0.98^7
});
```

---

### Visual Tests (HUD)

```typescript
test('Gauge animations', async () => {
  // 1. Render HUD with 60% influence
  render(<CoordinationGauges influence={60} />);
  
  // 2. Trigger spending action
  fireEvent.click(screen.getByText('Approve PR'));
  
  // 3. Verify bar animation (60% → 48%)
  await waitFor(() => {
    const bar = screen.getByTestId('influence-bar');
    expect(bar).toHaveStyle('width: 48%');
  });
  
  // 4. Verify toast notification
  expect(screen.getByText('Spent 12% influence')).toBeInTheDocument();
});
```

---

## PHILOSOPHICAL FOUNDATIONS

### Core Principles (Locked)

1. **Truth is the substrate**
   - All coordination primitives are data on an immutable, traceable foundation
   - No hidden state, no phantom money, no secret authority

2. **Orthogonal dimensions**
   - Money ≠ votes ≠ reputation ≠ commitments
   - Each dimension requires different strategies
   - No single "win condition"

3. **Decay as physics**
   - Authority is borrowed, not granted
   - Legitimacy requires maintenance
   - Power ossifies without renewal

4. **Visibility as accountability**
   - All actions are commits (logged, auditable)
   - Scars are permanent (no memory hole)
   - Spending is attributable (forensic trails)

5. **Constraints as design**
   - Commitment capacity prevents infinite promises
   - Time pressure prevents rushed decisions
   - Influence costs prevent spam

---

### Why This Matters

**Traditional systems choose one:**
- Markets → money
- Democracies → votes
- Bureaucracies → hierarchy

**Relay introduces a fourth thing:**
- **Votes that become temporary, decaying authority capacity**

That's not a tweak to democracy. It's a new coordination primitive.

It lets groups say:
- "We trust you this much"
- "For this scope"
- "For this long"
- "And we're watching"

**No elections. No coups. No bailouts. Just continuous legitimacy.**

---

## CONCLUSION

Relay's coordination substrate is now fully specified:

✅ **Economic primitives locked** (`architecture@c6`)  
✅ **StarCraft HUD model locked** (`architecture@c7`)  
✅ **Delegated decaying influence locked** (`architecture@c8`)

**All major architecture decisions are in filaments.**  
**Future development can reference them directly.**

**Status:** GOLD STANDARD LOCKED ✅  
**Next:** Implement PR #2 (Currency) → PR #3 (Voting) → PR #4 (Commitments) → PR #5 (DDI) → HUD Components

---

**Philosophy:**  
**Gauges, not wallets. Constraints, not wealth. Borrowed legitimacy, not granted power.**

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-28  
**Maintained By:** system.architect  
**Filament Ref:** `architecture@c6`, `architecture@c7`, `architecture@c8`

# COMMIT 7: Coordination Gauges (StarCraft HUD Model)

**Filament:** `architecture`  
**Commit Index:** 7  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** UX_SUBSTRATE

---

## INVARIANT LOCKED

**Relay does not show a wallet. Relay shows constraints.**

StarCraft doesn't show a bank balance. It shows what you can do next.  
Relay does the same: **coordination resources as orthogonal gauges, not one currency.**

---

## PART 1: WHY STARCRAFT HUD IS THE CORRECT METAPHOR

### StarCraft Resource Model (The Gold Standard)

StarCraft doesn't show "money." It shows **constraints that govern action**:

| Resource | Type | Constraint |
|----------|------|-----------|
| **Minerals** | General capacity | What you can build (abundant) |
| **Vespene Gas** | Scarce capacity | What you can upgrade (scarce) |
| **Supply** | Population limit | How many units (coordination cap) |
| **APM / Cooldowns** | Time-based | When you can act (temporal) |

**None of these are moral. They are physics.**

**Key Insight:** These are **orthogonal dimensions of constraint**, not a single scalar "wealth."

---

### Why This Maps to Relay Perfectly

Relay's coordination primitives are exactly like StarCraft resources:

| Relay Primitive | StarCraft Analog | Nature |
|----------------|------------------|--------|
| **Legitimacy** | Control / Influence | Maintained support |
| **Commitment Capacity** | Supply Cap | Obligation bandwidth |
| **Escrowed Resources** | Minerals / Gas | Real-world assets |
| **Time / Cooldowns** | Build Timers | Temporal constraints |
| **Risk / Disputes** | Damage / Debuffs | Governance friction |

**The visual language already exists.** We just need to map it correctly.

---

## PART 2: THE CANONICAL GAUGE SET (LOCKED)

### 🟦 Gauge 1: LEGITIMACY (Votes / Support)

**What it represents:**  
Current, maintained support for an entity, proposal, or filament.

**StarCraft analog:**  
Control / influence / map presence.

**Type:**  
Non-spendable, decaying signal (not a resource you "use up").

**HUD behavior:**
- Rises with positive votes (VOTE_CAST events)
- Decays over time if not renewed (VOTE_REFRESH events)
- Can be scoped (local / regional / global)
- Visual: Filled bar with slow drain animation

**Key properties:**
- ✅ Abundant (not scarce)
- ✅ Non-transferable (can't be bought/sold)
- ✅ Positive-sum (both can gain)
- ✅ Decaying (requires maintenance)
- ❌ Not spendable (only signals, doesn't deplete)

**Example values:**
```
Legitimacy: ████████░░ 82% (1,240 active votes, -2%/day decay)
```

**Commits that affect this:**
- `VOTE_CAST` → +legitimacy
- `VOTE_RETRACT` → -legitimacy
- `VOTE_DECAY` (system) → slow drain
- `COMMITMENT_BREACH` → -legitimacy (reputation hit)

---

### 🟨 Gauge 2: COMMITMENT CAPACITY

**What it represents:**  
How many active commitments an agent/entity can carry (obligation bandwidth).

**StarCraft analog:**  
Supply cap (population limit).

**Type:**  
Scarce, earned capacity (increases with reliability).

**HUD behavior:**
- Shows "used / total" (e.g., 3 / 8 commitments)
- Increases with successful fulfillments (COMMITMENT_FULFILL)
- Decreases with failures (COMMITMENT_BREACH)
- Visual: Progress bar with clear "slots"

**Key properties:**
- ✅ Scarce (limited bandwidth)
- ✅ Non-transferable (can't delegate obligation capacity)
- ✅ Merit-based (earned through reliability)
- ✅ Hard cap (prevents infinite promises)

**Example values:**
```
Commitments: ███░░░░░░ 3 / 8 (5 slots available, +1 per 10 fulfilled)
```

**Commits that affect this:**
- `COMMITMENT_PROMISE` → uses capacity
- `COMMITMENT_FULFILL` → frees capacity + earns +1 max (eventually)
- `COMMITMENT_BREACH` → loses max capacity (-1)
- `COMMITMENT_REVOKE` → frees capacity (penalty if arbitrary)

**Why this is critical:**  
Prevents spam proposals, fake seriousness, infinite promises.  
Replaces "money spent = seriousness" with **bounded obligation capacity**.

---

### 🟩 Gauge 3: ESCROWED RESOURCES (Money, Tokens, Assets)

**What it represents:**  
Real-world scarce resources currently locked under governance rules.

**StarCraft analog:**  
Minerals / Gas (spendable, zero-sum resources).

**Type:**  
Scarce, transferable, zero-sum (traditional money).

**HUD behavior:**
- Shows locked / pending / released amounts
- Often secondary in UI hierarchy (not the primary gauge)
- Visual: Coin icon + amount + lock status

**Key properties:**
- ✅ Scarce (finite)
- ✅ Transferable (can be sent)
- ✅ Zero-sum (my loss = your gain)
- ❌ Never conflated with legitimacy

**Example values:**
```
Escrow: ⛓ $1.2M locked (Release: 2026-02-15 or 80% approval)
```

**Commits that affect this:**
- `CURRENCY_TRANSFER` → moves tokens
- `ESCROW_LOCK` → freezes tokens under conditions
- `ESCROW_RELEASE` → unlocks tokens (if conditions met)
- `CURRENCY_BURN` → destroys tokens

**Key rule:**  
Escrow is **downstream of legitimacy**, never upstream.  
Money doesn't buy votes. Votes decide escrow.

---

### 🟪 Gauge 4: TIME / COOLDOWN PRESSURE

**What it represents:**  
Irreversible time commitments, vote durations, revocation windows.

**StarCraft analog:**  
Ability cooldowns / build timers.

**Type:**  
Temporal constraint (cannot be accelerated with money).

**HUD behavior:**
- Radial timers (circular progress)
- Progress bars (linear countdown)
- "You can't act again until X completes" messages
- Visual: Clock icon + countdown

**Key properties:**
- ✅ Universal (affects everyone equally)
- ✅ Non-transferable (can't buy more time)
- ✅ Zero-sum (one action blocks another)
- ❌ Cannot be accelerated (no "pay to skip")

**Example values:**
```
Time: ⏳ Vote ends in 2d 4h (87 blocks remaining)
```

**Commits that affect this:**
- `VOTE_OPEN` → starts timer
- `VOTE_CLOSE` → ends timer
- `COOLDOWN_ACTIVE` → prevents action
- `COOLDOWN_EXPIRED` → allows action

**Why this matters:**  
Time becomes a **cost**, not money.  
You can't buy your way out of due process.

---

### 🟥 Gauge 5: RISK / SCAR / DISPUTE PRESSURE

**What it represents:**  
Unresolved disputes, rejected commits, authority challenges.

**StarCraft analog:**  
Damage indicators / debuffs / status effects.

**Type:**  
Governance friction (visible, persistent markers).

**HUD behavior:**
- Visual scars on filaments (red marks on timeline)
- Red warning bars (active disputes)
- Hover → forensic detail (click to inspect)
- Visual: Warning triangle + count

**Key properties:**
- ✅ Permanent (scars don't fade)
- ✅ Visible (can't be hidden)
- ✅ Attributable (linked to specific commits)
- ❌ Cannot be removed (only resolved)

**Example values:**
```
Disputes: ⚠ 1 active (CommitRejected: AUTHORITY_DENIED)
```

**Commits that affect this:**
- `CommitRejected` → creates scar
- `DISPUTE_OPEN` → raises warning
- `DISPUTE_RESOLVED` → marks resolved (scar remains)
- `OUTPUT_REJECTED` → blocks progress

**Why this is critical:**  
Makes governance friction **visible**.  
You can't hide past failures or contested decisions.

---

## PART 3: WHY MULTIPLE GAUGES ARE ESSENTIAL

### Single Currency (Traditional Systems)

If you collapse everything into one "currency," you recreate:
- ❌ Plutocracy (richest decides)
- ❌ Pay-to-win governance (buy votes)
- ❌ Hidden power concentration (money = all power)

**Problem:** Money becomes a proxy for all coordination dimensions.

---

### Multiple Gauges (Relay Model)

By separating gauges, you make **tradeoffs explicit**:

| Scenario | Gauges | Interpretation |
|----------|--------|---------------|
| **High legitimacy, low resources** | 🟦 82%, 🟩 $0 | Popular but unfunded |
| **High resources, low legitimacy** | 🟦 12%, 🟩 $1M | Rich but blocked |
| **High commitment load** | 🟨 7/8 | Serious but stretched |
| **Low commitment capacity** | 🟨 0/3 | Unreliable actor |
| **Active disputes** | 🟥 5 scars | Contested history |

**Result:** Truthful coordination. No single dimension dominates.

---

## PART 4: STARCRAFT HUD LAYOUT (CONCRETE VISUAL)

```
┌─────────────────────────────────────────────────────┐
│ 🌍 GLOBAL CHANNEL: climate.policy.001               │
│                                                     │
│ 🟦 Legitimacy      ████████░░  82% (1,240 votes)   │
│    └─ Decay: -2%/day | Next renewal: 23h          │
│                                                     │
│ 🟨 Commitments     ███░░░░░░  3 / 8 active         │
│    └─ "Fix bug #42" (ends 2d), "Review PR" (4h)   │
│                                                     │
│ 🟩 Escrow          ⛓ $1.2M locked (2 conditions)   │
│    └─ Release: 80% vote OR 2026-02-15             │
│                                                     │
│ 🟪 Time            ⏳ Vote ends in 2d 4h 23m        │
│    └─ Cooldown: Next proposal in 6h               │
│                                                     │
│ 🟥 Disputes        ⚠ 1 active (click to inspect)   │
│    └─ Scar: OUTPUT_REJECTED at commit@c42         │
│                                                     │
│ [ View Filament ] [ Forensic Chamber ] [ Vote Now ]│
└─────────────────────────────────────────────────────┘
```

**Key properties:**
- ✅ All gauges visible at once (no hiding)
- ✅ Color-coded (instant recognition)
- ✅ Hover for details (forensic depth)
- ✅ Live updates via SSE (real-time truth)

---

## PART 5: HUD COMPONENT SPECIFICATION

### React Component Structure

```typescript
interface CoordinationGauges {
  legitimacy: LegitimacyGauge;
  commitments: CommitmentGauge;
  escrow: EscrowGauge;
  time: TimeGauge;
  disputes: DisputeGauge;
}

interface LegitimacyGauge {
  current: number;        // 0.0 - 1.0 (percentage)
  vote_count: number;     // Absolute vote count
  decay_rate: number;     // -0.02 = -2% per day
  next_renewal: string;   // ISO timestamp
  trend: "rising" | "falling" | "stable";
}

interface CommitmentGauge {
  active: number;         // Current commitments
  capacity: number;       // Max commitments
  commitments: Commitment[];
  next_capacity_increase: number; // After N fulfillments
}

interface EscrowGauge {
  total_locked: number;   // USD or token amount
  conditions: EscrowCondition[];
  release_date?: string;  // ISO timestamp (if time-based)
  release_threshold?: number; // Vote % (if vote-based)
}

interface TimeGauge {
  vote_ends_at?: string;  // ISO timestamp
  cooldown_until?: string; // ISO timestamp
  blocks_remaining?: number; // Blockchain blocks
}

interface DisputeGauge {
  active_count: number;
  scars: Scar[];          // Permanent markers
  unresolved: Dispute[];  // Pending resolution
}
```

---

### SSE Event Mapping (Live Updates)

```typescript
// SSE events → Gauge updates
eventBus.on("VOTE_CAST", (event) => {
  gauges.legitimacy.current += event.weight;
  gauges.legitimacy.vote_count += 1;
  gauges.legitimacy.trend = "rising";
});

eventBus.on("VOTE_DECAY", (event) => {
  gauges.legitimacy.current -= event.decay_amount;
  gauges.legitimacy.trend = "falling";
});

eventBus.on("COMMITMENT_PROMISE", (event) => {
  gauges.commitments.active += 1;
  gauges.commitments.commitments.push(event.commitment);
});

eventBus.on("COMMITMENT_FULFILL", (event) => {
  gauges.commitments.active -= 1;
  // Maybe increase capacity after N fulfillments
});

eventBus.on("ESCROW_LOCK", (event) => {
  gauges.escrow.total_locked += event.amount;
  gauges.escrow.conditions.push(event.condition);
});

eventBus.on("CommitRejected", (event) => {
  gauges.disputes.active_count += 1;
  gauges.disputes.scars.push({
    commit_ref: event.commit_ref,
    reason: event.reason_code,
    timestamp: event.timestamp,
  });
});
```

---

## PART 6: WHY THIS IS BETTER THAN MONEY (PHILOSOPHICAL)

### Traditional Systems: Money Forces Everything Through Negative Cash Flow

**Question asked:** "Who paid more?"

**Hidden dimensions:**
- Who showed up?
- Who stayed?
- Who committed?
- Who followed through?
- Who has unresolved scars?

**Result:** Money becomes a **false proxy** for all coordination signals.

---

### Relay: Orthogonal Gauges Reveal True Coordination State

**Questions answered explicitly:**
- 🟦 **Legitimacy:** Who has support? (votes)
- 🟨 **Commitments:** Who is reliable? (track record)
- 🟩 **Escrow:** Who has resources? (money)
- 🟪 **Time:** Who is patient? (due process)
- 🟥 **Disputes:** Who is contested? (scars)

**Result:** **Stronger signals for governance** than spending alone.

---

## PART 7: LOCKED DESIGN PRINCIPLES

### Principle 1: No Wallet, Only Constraints

**Traditional:**
```
Balance: $10,000
[Spend] [Transfer] [Withdraw]
```

**Relay:**
```
🟦 Legitimacy: 82%
🟨 Commitments: 3/8
🟩 Escrow: $1.2M locked
🟪 Time: 2d 4h remaining
🟥 Disputes: 1 active
```

**Difference:** Relay shows **what you can do next**, not **what you own**.

---

### Principle 2: Orthogonal Dimensions (No Single Power Metric)

**Bad (single scalar):**
```
Power: ████████░░ 87 / 100
```

**Good (orthogonal gauges):**
```
🟦 Legitimacy: 82%
🟨 Commitments: 3/8
🟩 Escrow: $1.2M
```

**Why:** Different dimensions require different strategies. No single "win condition."

---

### Principle 3: Visible Decay (Authority Requires Maintenance)

**Bad (static):**
```
Votes: 1,240 (permanent)
```

**Good (decaying):**
```
Legitimacy: ████████░░ 82% (1,240 votes, -2%/day)
Next renewal: 23h
```

**Why:** Legitimacy must be continuously earned, not assumed.

---

### Principle 4: Scars Are Permanent (No Memory Hole)

**Bad (erasable history):**
```
Disputes: 0 (all resolved and hidden)
```

**Good (visible scars):**
```
Disputes: ⚠ 1 active, 3 resolved scars
└─ Scar: OUTPUT_REJECTED at commit@c42 (2026-01-15)
```

**Why:** Past failures inform future trust. No reputation laundering.

---

## PART 8: IMPLEMENTATION ROADMAP

### Phase 1: Basic Gauges (MVP)
**Deliverables:**
1. Legitimacy gauge (vote count + decay simulation)
2. Commitment gauge (active / capacity)
3. Escrow gauge (locked amount + conditions)

**File:** `apps/client-web/src/components/CoordinationGauges.tsx`

**Time:** 2-3 hours

---

### Phase 2: Live SSE Updates
**Deliverables:**
1. SSE listener for gauge-affecting events
2. Smooth animations (bar fills, decay drains)
3. Real-time updates (no refresh needed)

**File:** `apps/client-web/src/hooks/useCoordinationGauges.ts`

**Time:** 2-3 hours

---

### Phase 3: Time + Dispute Gauges
**Deliverables:**
1. Time gauge (radial countdown, cooldowns)
2. Dispute gauge (scar markers, forensic links)

**Time:** 2-3 hours

---

### Phase 4: Per-Unit Gauges (SCV-Specific)
**Deliverables:**
1. Each SCV shows mini gauges (hover tooltip)
2. Click SCV → full gauge detail panel

**Time:** 1-2 hours

---

## PART 9: LOCKED INVARIANTS

1. ✅ **Relay shows constraints, not wealth** (gauges, not wallet)
2. ✅ **Gauges are orthogonal** (no single power metric)
3. ✅ **Legitimacy decays** (requires maintenance)
4. ✅ **Commitments are capped** (prevents infinite promises)
5. ✅ **Escrow is downstream** (money doesn't buy votes)
6. ✅ **Time is universal** (no pay-to-skip)
7. ✅ **Scars are permanent** (visible history)
8. ✅ **All gauges are live** (SSE-driven, real-time)

---

## CAUSAL REFS

- **Inputs:** `architecture@c6` (Economic primitives)
- **Authority:** system.architect
- **Evidence:**
  - StarCraft UI design (1998, Blizzard)
  - RTS resource management best practices
  - Governance friction visibility (scar markers)

---

## NEXT STEPS

### Immediate (UI Implementation)
1. Create `CoordinationGauges.tsx` React component
2. Wire to SSE event stream
3. Add to main HUD (top-right corner)

### Short-Term (Backend Support)
1. Add vote decay calculation (system events)
2. Add commitment capacity tracking
3. Add scar persistence (on CommitRejected)

---

**Status:** LOCKED  
**Supersedes:** None (new UX substrate)  
**Next:** Implement HUD component + SSE wiring

---

**Philosophy:**  
StarCraft already solved this 25 years ago.  
Relay just maps coordination primitives to the same visual language.  
**Gauges, not wallets. Constraints, not wealth.**

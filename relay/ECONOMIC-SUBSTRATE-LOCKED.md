# Relay Economic Substrate - LOCKED ✅

**Date:** 2026-01-28  
**Status:** Architecture decisions locked as `architecture@c6`  
**Reference:** `relay/filaments/architecture/0006_economic_primitives_foundation.md`

---

## YOUR QUESTIONS - ANSWERED

### Q1: How will Relay change banking and money?

**Traditional Banking = Opaque Trust Intermediaries**  
Banks keep ledgers secret. You trust them to update balances correctly. Credit creates "phantom money" (fractional reserve).

**Relay Banking = Transparent Truth Substrate**  
Every transaction is a commit (immutable, auditable). Credit is visible authority delegation (no fractional reserve). Anyone can inspect balances.

**What changes:**
- ✅ No hidden leverage (all credit visible)
- ✅ No fractional reserve (full accounting enforced)
- ✅ No double-spend (causal refs + verification)
- ✅ No opacity (all commits public)
- ✅ No intermediaries (direct peer-to-peer)

**Bottom line:** Banking becomes a **service** (credit analysis, insurance), not a **gatekeeper** (ledger control).

---

### Q2: Is banking even needed in the future?

**What banks do that Relay replaces:**
- ✅ Ledger-keeping → Filaments
- ✅ Payment settlement → Commit verification
- ✅ Credit scoring → Authority chain inspection
- ✅ Fraud prevention → Causal proof chains
- ✅ Dispute resolution → Forensic commit inspection

**What banks do that Relay does NOT replace:**
- 🔄 Fiat on/off-ramps (temporary, until Relay-native economy)
- 🔄 Regulatory compliance (KYC/AML - Layer 1 concern)
- 🔄 Insurance (FDIC) → Replaced by transparent risk pools

**Verdict:** Banks as **gatekeepers** are obsolete. Banks as **service providers** (credit analysis, insurance) remain.

---

### Q3: Shouldn't money's relationship with truth be mapped in Relay?

**YES. THIS IS THE CORE INSIGHT.**

**Traditional money is divorced from truth:**
- You trust banks to update ledgers correctly
- Settlement takes days (opacity window)
- Chargebacks reverse transactions (mutable history)
- Double-spend is possible (before settlement)

**Relay money IS truth:**
- Every transaction is a commit (immutable)
- Settlement is instant (commit accepted)
- No reversals (append-only history)
- Double-spend is impossible (causal refs prevent it)

**Key mapping:**
```
Money = Transferable Authority Tokens
Credit = Visible Authority Delegation
Balance = Count(CURRENCY_ISSUE) - Count(CURRENCY_TRANSFER) + Count(CURRENCY_BURN)
Supply = Count(CURRENCY_ISSUE commits)
```

**Every economic action is a commit.** Truth is the substrate.

---

### Q4: Should Relay have a "currency"?

**LOCKED DECISION: OPTION C - PLURAL AUTHORITIES + OPTIONAL NATIVE TOKEN**

**Hybrid approach:**
1. **Relay does NOT require a native currency**
   - Any filament can be monetary
   - Communities issue their own tokens
2. **Relay MAY have an optional native token (REL)**
   - Pay for Layer 2 compute (gas fees)
   - Stake to become a verifier node
   - Boost visibility (attention market)
3. **All currencies are transparent**
   - Supply = count(CURRENCY_ISSUE commits)
   - Exchange rates emerge naturally

**Philosophy:** Relay is **substrate-neutral**. Money is just data.

**Example currencies:**
- `currency.REL` - Relay native token (optional)
- `currency.USD` - US dollar on Relay (central bank issued)
- `currency.ACME` - Community token (anyone can issue)
- `reputation.karma` - Non-transferable merit (soulbound)

---

### Q5: Is voting a new form of currency?

**NO. Voting and money are DIFFERENT primitives.**

| Property | Money (Subtractive) | Voting (Additive) |
|----------|-------------------|------------------|
| **Scarcity** | Yes (finite balance) | No (unlimited votes) |
| **Transferable** | Yes (send to others) | Sometimes (delegation) |
| **Zero-Sum** | Yes (my loss = your gain) | No (both can win) |
| **Coordination** | Market-based | Consensus-based |

**Money = Scarce, transferable, zero-sum** (I give up X to get Y)  
**Voting = Abundant, non-transferable, positive-sum** (I signal X is valuable)

**But they can interact:**
```
Alice has 100 REL (money) + unlimited votes (attention)
Alice votes for Project X (costs 0 REL, signals support)
Project X reaches 1000 votes → Alice decides to fund with 50 REL
```

**Voting is a currency of ATTENTION, not MONEY.**

---

### Q6: Design a monetary filament type properly

**SEE:** `architecture@c6` Part 6 for full design

**Summary:**
```rust
FilamentType::Currency {
    currency_id: "REL" | "USD" | "community.ACME",
    issuance_policy: IssuancePolicy {
        issuer: UnitId,           // Who can mint
        max_supply: Option<u64>,  // Cap (None = unlimited)
        inflation_rate: Option<f32>, // Automated inflation
    },
    transfer_rules: TransferRules {
        transferability: Freely | WhitelistOnly | NonTransferable,
        divisibility: u8,         // Decimal places
        transfer_fee: Option<Fee>, // Optional gas
    },
}
```

**Operations:**
- `CURRENCY_ISSUE` - Mint new tokens
- `CURRENCY_TRANSFER` - Send tokens
- `CURRENCY_BURN` - Destroy tokens
- `CURRENCY_DELEGATE` - Lend tokens (credit)
- `CURRENCY_REVOKE` - Cancel loan

---

### Q7: Explore how credit changes in a Relay world

**Traditional Credit = Opaque, Fractional, Phantom Money**
```
Bank has $1000 → Lends $9000 (fractional reserve)
Total supply: $10,000 (only $1000 actually exists)
```

**Relay Credit = Transparent, Full-Reserve, Provable**
```
Bob has 1000 REL → Delegates 1000 REL authority to Alice
Alice spends 500 REL → Bob's balance: 500 locked, 500 spent
Anyone can verify: Bob's balance commit + Alice's delegation commit
```

**Key transformation:**
- ✅ Credit = visible authority delegation
- ✅ No fractional reserve (full accounting enforced)
- ✅ Transparent leverage (count delegation commits / balance)
- ✅ Conditional loans (max per tx, expiration, revocable)

**Example:**
```json
{
  "op_type": "CURRENCY_DELEGATE",
  "author_unit_ref": "unit.bob.002",
  "payload": {
    "delegatee": "unit.alice.001",
    "amount": "1000.000000",
    "expires_at": "2027-01-28",
    "conditions": ["max_per_tx: 100"]
  },
  "causal_refs": {
    "authority_ref": "unit.bob.002@balance"
  }
}
```

---

### Q8: Map how central banks might plug into this

**Central Bank Functions → Relay Implementation**

| Function | Traditional | Relay Equivalent |
|----------|------------|------------------|
| **Currency Issuance** | Print money | Issue `CURRENCY_ISSUE` commits |
| **Monetary Policy** | Set interest rates | Set delegation fees |
| **Lender of Last Resort** | Bail out banks | Delegate emergency authority |
| **Financial Stability** | Regulate banks | Audit filaments (transparent) |

**Example: Federal Reserve on Relay**
```json
{
  "filament_id": "currency.USD",
  "op_type": "CURRENCY_ISSUE",
  "author_unit_ref": "unit.federal_reserve",
  "payload": {
    "recipient": "unit.treasury",
    "amount": "1000000000.00",
    "reason": "Quantitative easing 2026-Q1"
  },
  "causal_refs": {
    "authority_ref": "congress.authorization@c123",
    "evidence": ["fomc.meeting.2026-01-15"]
  }
}
```

**What changes:**
- ✅ Transparent money printing (all commits visible)
- ✅ Real-time economic data (inspect filaments)
- ✅ Precise policy tools (conditional delegations)
- ❌ No hidden bailouts (delegations are public)
- ❌ No fractional reserve (full accounting enforced)

**Verdict:** Central banks remain, but become **transparent utilities** instead of **opaque gatekeepers**.

---

### Q9: Formalize "commitment without money" as first-class primitive

**Core Insight:** Most coordination doesn't need money, it needs **commitment**.

Traditional systems conflate:
- **Commitment** (binding promise) → Requires money as collateral
- **Payment** (value transfer) → Requires money as medium

In Relay, these are separate:
- **Commitment** = Causal commit with authority_ref (no money needed)
- **Payment** = CURRENCY_TRANSFER commit (money moves)

**Commitment Primitive:**
```rust
FilamentType::Commitment {
    commitment_type: Promise | Obligation | Delegation | Guarantee,
    enforcement: SelfReporting | ThirdPartyArbitration | AutomaticPenalty,
}

enum Penalty {
    ReputationSlash(i32),   // Lose reputation points
    FinancialBurn(u64),      // Burn tokens
    AuthorityRevoke,         // Lose delegated authority
    PublicShame,             // Visible scar on history
}
```

**Examples of commitments that don't need money:**
- ✅ "I'll review your PR by Friday" (time commitment)
- ✅ "I vouch for Alice's work" (reputation commitment)
- ✅ "I'll maintain this server" (service commitment)
- ✅ "I approve this decision" (authority commitment)

**Example:**
```json
{
  "filament_id": "commitment.promise.001",
  "op_type": "COMMITMENT_PROMISE",
  "author_unit_ref": "unit.alice.001",
  "payload": {
    "promise": "Deliver Q1 report by 2026-01-31",
    "beneficiary": "unit.bob.002",
    "deadline": "2026-01-31T23:59:59Z",
    "penalty": {
      "type": "ReputationSlash",
      "amount": -50
    }
  },
  "causal_refs": {
    "authority_ref": "unit.alice.001@reputation"
  }
}
```

**No money involved.** Alice commits by staking **reputation**, not cash.

---

## LOCKED ECONOMIC PRIMITIVES

### 1. Authority Tokens (Money)
```rust
FilamentType::Currency { ... }
Operations: CURRENCY_ISSUE, CURRENCY_TRANSFER, CURRENCY_BURN, CURRENCY_DELEGATE
Properties: Scarce, transferable, zero-sum
```

### 2. Attention Signals (Voting)
```rust
FilamentType::Vote { ... }
Operations: VOTE_CAST, VOTE_CHANGE, VOTE_RETRACT
Properties: Abundant, non-transferable, positive-sum
```

### 3. Reputation (Merit)
```rust
FilamentType::Reputation { ... }
Operations: REPUTATION_EARN, REPUTATION_SLASH
Properties: Scarce, non-transferable, merit-based
```

### 4. Commitments (Promises)
```rust
FilamentType::Commitment { ... }
Operations: COMMITMENT_PROMISE, COMMITMENT_FULFILL, COMMITMENT_BREACH
Properties: Binding, visible, enforceable (penalties)
```

### 5. Authority Delegation (Credit)
```rust
Operations: CURRENCY_DELEGATE, COMMITMENT_DELEGATE
Properties: Temporary, revocable, conditional, transparent
```

---

## LOCKED INVARIANTS

1. ✅ **Money = Transferable Authority Tokens** (currency filaments)
2. ✅ **Credit = Visible Authority Delegation** (no fractional reserve)
3. ✅ **Voting ≠ Money** (attention signals, not scarcity)
4. ✅ **Commitment ≠ Payment** (stake reputation/authority, not cash)
5. ✅ **All Economic Actions = Commits** (transparent, immutable, auditable)
6. ✅ **No Phantom Money** (full-reserve, all tokens accounted for)
7. ✅ **Central Banks = Transparent Issuers** (no hidden printing)
8. ✅ **Banking Functions = Coordination Services** (not gatekeepers)

---

## NEXT STEPS - IMPLEMENTATION ROADMAP

### Immediate (PR #2 - Monetary Filaments)
1. Implement `FilamentType::Currency`
2. Add `CURRENCY_ISSUE`, `CURRENCY_TRANSFER`, `CURRENCY_BURN` operations
3. Add balance tracking (derived from filament commits)
4. Add delegation tracking (credit lines)

### Short-Term (PR #3 - Commitment Primitives)
1. Implement `FilamentType::Commitment`
2. Add `COMMITMENT_PROMISE`, `COMMITMENT_FULFILL`, `COMMITMENT_BREACH` operations
3. Add penalty enforcement (reputation slash, token burn)

### Medium-Term (PR #4 - Voting/Reputation)
1. Implement `FilamentType::Vote`
2. Implement `FilamentType::Reputation`
3. Add voting power calculation (attention markets)
4. Add reputation scoring (merit-based authority)

### Long-Term (PR #5 - Central Bank Integration)
1. Add `currency.USD` filament (central bank issued)
2. Add fiat on/off-ramps (bridge to traditional banking)
3. Add KYC/AML compliance layer (identity substrate)

---

## PHILOSOPHY CHECK

✅ **Per `architecture@c6`:** Truth is the substrate. Money, credit, voting, and commitments are just coordination data on that substrate.

✅ **Per `architecture@c0`:** Layer 2 outputs **data**, not **policy**. Economic primitives are neutral.

✅ **Per `architecture@c2`:** All economic actions are **commits** (immutable, auditable, causal).

---

**Status:** ✅ **ECONOMIC SUBSTRATE LOCKED**  
**Reference:** `architecture@c6`  
**Next:** Implement PR #2 (Monetary Filaments) or Resume Option C (Backend Render Endpoints)

---

**Bottom Line:** Relay doesn't "add payments." Relay **replaces money with truth**, and banks become transparent services.

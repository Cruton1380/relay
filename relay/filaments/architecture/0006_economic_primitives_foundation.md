# COMMIT 6: Economic Primitives Foundation

**Filament:** `architecture`  
**Commit Index:** 6  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** ECONOMIC_SUBSTRATE

---

## CONTEXT: WHY ECONOMICS MATTERS FOR RELAY

Relay is a **coordination substrate**. Money, credit, and banking are **coordination technologies**. Therefore, Relay must have native economic primitives, not bolted-on payments.

**The fundamental questions:**
1. How does Relay change banking and money?
2. Is banking even needed?
3. Should money's relationship with truth be mapped in Relay?
4. Should Relay have a "currency"?
5. Is voting a new form of currency?
6. What are the first-class economic primitives?

---

## PART 1: HOW RELAY CHANGES BANKING

### Traditional Banking Functions (What Banks Do)

| Function | Traditional Banking | Information Asymmetry |
|----------|-------------------|---------------------|
| **Store of Value** | Accounts, ledgers | Opaque (you trust the bank) |
| **Medium of Exchange** | Wire transfers, checks | Slow, intermediated |
| **Unit of Account** | Currency denomination | Centrally controlled |
| **Credit Creation** | Fractional reserve | Phantom money (9x leverage) |
| **Risk Pooling** | Deposits → loans | Hidden exposure |
| **Time-Shifting** | Loans (future→present) | Terms hidden, renegotiable |
| **Trust Intermediation** | Banks vouch for parties | Single point of failure |

**Core Issue:** All of these require **trusting an intermediary** because the ledger is **opaque**.

---

### Relay Banking Functions (How Relay Transforms Each)

| Function | Relay Implementation | Information Advantage |
|----------|---------------------|---------------------|
| **Store of Value** | Immutable filament commits | Transparent, auditable, unforgeable |
| **Medium of Exchange** | Authority transfer commits | Instant, causal, traceable |
| **Unit of Account** | Voting power, attention, authority | Plural (not just USD) |
| **Credit Creation** | Authority delegation | Transparent leverage, no phantom debt |
| **Risk Pooling** | Multi-sig filaments | Shared visibility, shared authority |
| **Time-Shifting** | Causal commitment chains | Binding, unbreakable, auditable |
| **Trust Intermediation** | Causal proof chains | Zero intermediaries |

**Core Transformation:** **Truth becomes the substrate, not trust.**

---

## PART 2: IS BANKING EVEN NEEDED?

### What Traditional Banks Do That Relay Replaces

✅ **Ledger-keeping** → Relay's append-only filaments  
✅ **Payment settlement** → Commit verification pipeline  
✅ **Credit scoring** → Authority chain inspection (visible history)  
✅ **Fraud prevention** → Causal refs + verification (impossible to forge)  
✅ **Dispute resolution** → Forensic commit inspection (immutable truth)

**Verdict:** Most traditional banking functions are **coordination** + **record-keeping**. Relay does this natively.

---

### What Banks Do That Relay Does NOT Replace (Yet)

🔄 **Physical infrastructure** (ATMs, branches) - Not relevant for digital coordination  
🔄 **Regulatory compliance** (KYC, AML) - Layer 1 concern (identity substrate)  
🔄 **Fiat conversion** (USD on-ramp) - Temporary bridge until Relay-native economy  
🔄 **Insurance** (FDIC) - Replaced by transparent risk (no hidden leverage)  
🔄 **Customer support** (disputes) - Replaced by causal proof + forensic tools

**Verdict:** Banks as **trusted intermediaries** are obsolete. Banks as **regulated gateways to fiat** are temporary.

---

### The Future: "Banks" Become Services, Not Gatekeepers

In a Relay world:
- **Anyone** can issue credit (by delegating authority)
- **Anyone** can pool risk (by creating multi-sig filaments)
- **Anyone** can verify credit-worthiness (by inspecting authority chains)

**Traditional banks** become:
1. **Fiat on/off-ramps** (temporary, until Relay-native economy)
2. **Specialized credit analyzers** (but anyone can compete)
3. **Insurance pools** (but transparent, not opaque)

**Bottom line:** Banking as a **gatekeeping function** ends. Banking as a **service** continues.

---

## PART 3: MONEY'S RELATIONSHIP WITH TRUTH

### Traditional Money: Trust-Based, Opaque

```
Traditional Money Flow:
Alice → Bank A → SWIFT → Bank B → Bob

Truth Visibility: ❌ (only banks see full path)
Settlement Time: Days
Reversibility: Yes (chargebacks, fraud)
Double-Spend: Possible (before settlement)
```

**Problem:** Money is **divorced from truth**. You must trust intermediaries to update ledgers correctly.

---

### Relay Money: Truth-Based, Transparent

```
Relay Money Flow:
Alice → (CommitEvent) → Bob

Truth Visibility: ✅ (entire causal chain visible)
Settlement Time: Instant (commit accepted)
Reversibility: No (append-only, immutable)
Double-Spend: Impossible (causal refs + verification)
```

**Solution:** Money **IS** truth. Every transaction is a commit in a monetary filament.

---

### Mapping Money to Truth (The Core Insight)

In Relay, **money is just a special kind of authority token**:

| Concept | Traditional Money | Relay Authority Token |
|---------|------------------|---------------------|
| **Creation** | Central bank prints | Authority delegation commit |
| **Transfer** | Bank updates ledgers | Authority transfer commit |
| **Verification** | Trust bank balance | Inspect filament (immutable) |
| **Supply** | Hidden (fractional reserve) | Visible (count delegation commits) |
| **Inflation** | Opaque (monetary policy) | Transparent (new authority issuance visible) |
| **Destruction** | Debt payoff (invisible) | Authority revocation commit (visible) |

**Key Insight:** **Money = transferable authority = delegation tokens**

---

## PART 4: SHOULD RELAY HAVE A "CURRENCY"?

### Option A: No Native Currency (Plural Economies)

**Philosophy:** Relay is a coordination substrate, not a monetary system. Any group can issue their own currency (authority tokens).

**Pros:**
- ✅ Maximum flexibility (communities self-organize)
- ✅ No central issuance (truly decentralized)
- ✅ Interoperability (exchange rates emerge naturally)

**Cons:**
- ❌ Fragmentation (many currencies, hard to compare value)
- ❌ Bootstrapping problem (why accept a new token?)

---

### Option B: Native Currency (Relay Tokens)

**Philosophy:** Relay has a single native currency (e.g., "REL" tokens) used for:
- Paying for compute (Layer 2 operations)
- Staking for authority (proof of commitment)
- Voting power (governance)

**Pros:**
- ✅ Unified economy (easy to compare value)
- ✅ Network effects (everyone uses REL)
- ✅ Clear incentive (run nodes, earn REL)

**Cons:**
- ❌ Central point of control (who issues initial REL?)
- ❌ Artificial scarcity (why should tokens be scarce?)
- ❌ Crypto baggage (speculation, volatility)

---

### **LOCKED DECISION: OPTION C - PLURAL AUTHORITIES, OPTIONAL NATIVE TOKEN**

**Hybrid approach:**
1. **Relay does NOT require a native currency** (any filament can be monetary)
2. **Relay MAY have an optional native token** (REL) for:
   - Paying for Layer 2 compute (gas fees)
   - Staking to become a verifier node
   - Boosting visibility (attention market)
3. **Communities issue their own currencies** (authority tokens on filaments)
4. **Exchange rates are transparent** (visible supply via commit history)

**Philosophy:** **Relay is substrate-neutral. Money is just data.**

---

## PART 5: IS VOTING A CURRENCY?

### Traditional Money: Negative Selection (Subtractive)

```
Money Flow:
Alice has $100 → Spends $50 on X → Alice now has $50

Key Property: SCARCITY (spending reduces balance)
```

**Money is subtractive:** You give up X to get Y.

---

### Voting: Positive Selection (Additive)

```
Voting Flow:
Alice votes for X → X gains +1 signal → Alice still has full voting power

Key Property: ABUNDANCE (voting does not reduce power)
```

**Voting is additive:** You signal X without giving up Y.

---

### The Core Difference: Scarcity vs Signaling

| Property | Money (Subtractive) | Voting (Additive) |
|----------|-------------------|------------------|
| **Scarcity** | Yes (finite balance) | No (can vote unlimited) |
| **Transferability** | Yes (can give to others) | Sometimes (delegation) |
| **Zero-Sum** | Yes (your loss = my gain) | No (both can win) |
| **Coordination** | Market-based | Consensus-based |

**Verdict:** **Voting and money are DIFFERENT coordination primitives.**

---

### BUT: They Can Coexist and Interact

In Relay:
1. **Money (Authority Tokens)** = Scarce, transferable, zero-sum
2. **Votes (Attention Signals)** = Abundant, non-transferable, positive-sum
3. **Reputation (Earned Authority)** = Scarce, non-transferable, merit-based

**Example Flow:**
```
Alice has 100 REL (money) + unlimited votes (attention)
Alice votes for Project X (costs 0 REL, signals support)
Project X reaches 1000 votes → Alice decides to fund with 50 REL
REL transfer is a commit (scarce, zero-sum)
Vote signal is a commit (abundant, positive-sum)
```

**Key Insight:** **Voting is a currency of ATTENTION, not MONEY.**

---

## PART 6: DESIGN A MONETARY FILAMENT TYPE

### Monetary Filament Structure

```rust
// Monetary Filament Type: "currency"
FilamentType::Currency {
    currency_id: "USD" | "REL" | "community.ACME",
    issuance_policy: IssuancePolicy,
    transfer_rules: TransferRules,
}

// Issuance Policy
IssuancePolicy {
    issuer: UnitId,           // Who can mint new tokens?
    max_supply: Option<u64>,  // Cap on total supply (None = unlimited)
    inflation_rate: Option<f32>, // Automated inflation (0.02 = 2% per year)
    burn_authority: BurnPolicy,  // Who can destroy tokens?
}

// Transfer Rules
TransferRules {
    transferability: Transferability,
    divisibility: u8,         // Decimal places (0 = whole units only)
    transfer_fee: Option<Fee>, // Optional network fee
}

enum Transferability {
    Freely,                   // Anyone can transfer to anyone
    WhitelistOnly(Vec<UnitId>), // Only approved recipients
    NonTransferable,          // Reputation/soulbound tokens
}
```

---

### Example 1: REL (Relay Native Token)

```rust
FilamentType::Currency {
    currency_id: "REL",
    issuance_policy: IssuancePolicy {
        issuer: UnitId::new("relay.foundation"),
        max_supply: Some(1_000_000_000), // 1 billion cap
        inflation_rate: None,             // No inflation
        burn_authority: BurnPolicy::Anyone, // Voluntary burn
    },
    transfer_rules: TransferRules {
        transferability: Transferability::Freely,
        divisibility: 6, // 0.000001 REL precision
        transfer_fee: Some(Fee::Fixed(1)), // 1 REL per transfer
    },
}
```

---

### Example 2: Community Token (ACME)

```rust
FilamentType::Currency {
    currency_id: "community.ACME",
    issuance_policy: IssuancePolicy {
        issuer: UnitId::new("unit.acme.admin"),
        max_supply: None, // Unlimited (community decides)
        inflation_rate: Some(0.02), // 2% per year
        burn_authority: BurnPolicy::IssuerOnly,
    },
    transfer_rules: TransferRules {
        transferability: Transferability::WhitelistOnly(vec![
            UnitId::new("unit.alice.001"),
            UnitId::new("unit.bob.002"),
        ]),
        divisibility: 2, // 0.01 ACME precision
        transfer_fee: None, // Free transfers
    },
}
```

---

### Example 3: Reputation Token (Non-Transferable)

```rust
FilamentType::Currency {
    currency_id: "reputation.karma",
    issuance_policy: IssuancePolicy {
        issuer: UnitId::new("system.reputation"),
        max_supply: None, // Unlimited
        inflation_rate: None, // Merit-based only
        burn_authority: BurnPolicy::Never, // Cannot be destroyed
    },
    transfer_rules: TransferRules {
        transferability: Transferability::NonTransferable, // Soulbound
        divisibility: 0, // Whole numbers only
        transfer_fee: None,
    },
}
```

---

### Monetary Commit Operations

```rust
// Issue new tokens
pub const CURRENCY_ISSUE: &str = "CURRENCY_ISSUE";

// Transfer tokens
pub const CURRENCY_TRANSFER: &str = "CURRENCY_TRANSFER";

// Burn tokens
pub const CURRENCY_BURN: &str = "CURRENCY_BURN";

// Delegate spending authority (credit)
pub const CURRENCY_DELEGATE: &str = "CURRENCY_DELEGATE";

// Revoke delegation
pub const CURRENCY_REVOKE: &str = "CURRENCY_REVOKE";
```

---

### Example Monetary Commit

```json
{
  "filament_id": "currency.REL",
  "commit_index": 42,
  "commit_ref": "currency.REL@c42",
  "timestamp": "2026-01-28T10:30:00Z",
  "op_type": "CURRENCY_TRANSFER",
  "author_unit_ref": "unit.alice.001",
  "payload": {
    "from": "unit.alice.001",
    "to": "unit.bob.002",
    "amount": "50.000000",
    "memo": "Payment for services"
  },
  "causal_refs": {
    "inputs": ["currency.REL@c41"],
    "authority_ref": "unit.alice.001@balance",
    "evidence": []
  }
}
```

---

## PART 7: HOW CREDIT CHANGES IN RELAY

### Traditional Credit: Opaque, Fractional, Risky

```
Traditional Credit:
Bank → Lends $1000 to Alice → Creates $1000 out of thin air
Bank → Lends $9000 more (fractional reserve)
Total: $10,000 in circulation, but only $1000 in reserves

Risk: Hidden leverage, bank run vulnerability
Trust: Must trust bank's solvency
```

**Problem:** **Phantom money** - Credit creates money that doesn't exist.

---

### Relay Credit: Transparent, Full-Reserve, Provable

```
Relay Credit:
Bob → Delegates 1000 REL authority to Alice
Alice → Can spend up to 1000 REL (Bob's balance)
Commit chain shows: Alice@spend → authority_ref: Bob@delegation

Transparency: Anyone can inspect Bob's balance
Risk: Visible leverage (Bob's balance is locked)
Trust: Zero trust needed (causal proof chain)
```

**Solution:** **Credit = visible authority delegation**

---

### Credit as Authority Delegation

In Relay, credit is just **delegating spending authority**:

```rust
// Bob delegates credit to Alice
CommitEvent {
    filament_id: "currency.REL",
    op_type: "CURRENCY_DELEGATE",
    author_unit_ref: "unit.bob.002",
    payload: {
        "delegatee": "unit.alice.001",
        "amount": "1000.000000",
        "expires_at": "2027-01-28T10:30:00Z",
        "conditions": ["max_per_tx: 100"]
    },
    causal_refs: {
        authority_ref: "unit.bob.002@balance", // Bob must have 1000 REL
        ...
    }
}

// Alice spends delegated credit
CommitEvent {
    filament_id: "currency.REL",
    op_type: "CURRENCY_TRANSFER",
    author_unit_ref: "unit.alice.001",
    payload: {
        "from": "unit.bob.002", // Spending Bob's money
        "to": "unit.charlie.003",
        "amount": "100.000000"
    },
    causal_refs: {
        authority_ref: "currency.REL@c42", // Points to Bob's delegation
        ...
    }
}
```

**Key Properties:**
1. ✅ **Transparent:** Anyone can see Alice is spending Bob's money
2. ✅ **Full-reserve:** Bob must have 1000 REL (no fractional reserve)
3. ✅ **Conditional:** Bob can set limits (max per tx, expiration)
4. ✅ **Revocable:** Bob can revoke delegation at any time

---

### Credit Types in Relay

| Credit Type | Traditional Banking | Relay Implementation |
|-------------|-------------------|---------------------|
| **Personal Loan** | Bank → Alice (hidden) | Bob → Alice (visible delegation) |
| **Credit Card** | Visa → Alice (monthly statement) | Bob → Alice (real-time filament) |
| **Mortgage** | Bank → Alice (30 years, renegotiable) | Bob → Alice (causal chain, immutable) |
| **Line of Credit** | Bank → Alice (revolving, opaque) | Bob → Alice (delegation with max_amount) |
| **Overdraft** | Bank → Alice (automatic, hidden fee) | Bob → Alice (pre-authorized delegation) |

**Transformation:** All credit becomes **explicit, visible, provable authority delegation**.

---

### Fractional Reserve is Impossible in Relay

**Traditional Banking:**
```
Bank has $1000 → Lends $9000 (fractional reserve) → Total supply: $10,000
Problem: Only $1000 actually exists (phantom money)
```

**Relay:**
```
Bob has 1000 REL → Delegates 1000 REL to Alice → Total supply: 1000 REL
Alice spends 500 REL → Bob's balance: 500 REL locked, 500 REL spent
Anyone can verify: Bob's balance commit + Alice's delegation commit
```

**Result:** **No phantom money. Every REL is accounted for.**

---

## PART 8: HOW CENTRAL BANKS PLUG INTO RELAY

### Traditional Central Bank Functions

| Function | How It Works Today | Relay Equivalent |
|----------|------------------|------------------|
| **Monetary Policy** | Set interest rates | Set delegation fees |
| **Currency Issuance** | Print money | Issue commits (CURRENCY_ISSUE) |
| **Inflation Control** | Adjust money supply | Adjust issuance_policy.inflation_rate |
| **Lender of Last Resort** | Bail out banks | Delegate emergency authority |
| **Financial Stability** | Regulate banks | Audit filaments (transparent) |

---

### Central Bank as a Relay Node

In Relay, a central bank becomes:

1. **Currency Issuer** (authority to issue national currency commits)
2. **Delegation Hub** (lends to commercial banks via authority delegation)
3. **Transparent Auditor** (inspects filaments, no hidden leverage)

**Example: Federal Reserve as a Relay Node**

```rust
// Fed issues USD on Relay
CommitEvent {
    filament_id: "currency.USD",
    op_type: "CURRENCY_ISSUE",
    author_unit_ref: "unit.federal_reserve",
    payload: {
        "recipient": "unit.treasury",
        "amount": "1000000000.00", // $1 billion
        "reason": "Quantitative easing 2026-Q1"
    },
    causal_refs: {
        authority_ref: "congress.authorization@c123",
        evidence: ["fomc.meeting.2026-01-15"]
    }
}

// Fed lends to commercial bank
CommitEvent {
    filament_id: "currency.USD",
    op_type: "CURRENCY_DELEGATE",
    author_unit_ref: "unit.federal_reserve",
    payload: {
        "delegatee": "unit.chase_bank",
        "amount": "100000000.00", // $100M credit line
        "interest_rate": 0.05, // 5% per year
        "expires_at": "2027-01-28T10:30:00Z"
    },
    causal_refs: {
        authority_ref: "unit.federal_reserve@balance",
        ...
    }
}
```

**Transparency Benefits:**
- ✅ Every dollar issuance is a visible commit
- ✅ Every bank loan is a visible delegation
- ✅ Total money supply = count(CURRENCY_ISSUE commits)
- ✅ Bank leverage = count(CURRENCY_DELEGATE commits) / bank.balance

---

### Central Banks Lose Opacity, Gain Legitimacy

**What Central Banks Lose:**
- ❌ Ability to hide money printing (all commits visible)
- ❌ Ability to bail out banks secretly (delegations are public)
- ❌ Control over fractional reserve ratios (Relay enforces full-reserve)

**What Central Banks Gain:**
- ✅ Transparent legitimacy (no "money printer go brrr" conspiracy)
- ✅ Real-time economic data (inspect filaments directly)
- ✅ Precise policy tools (conditional delegations, programmable rules)

**Bottom Line:** Central banks remain, but become **transparent utilities** instead of **opaque gatekeepers**.

---

## PART 9: "COMMITMENT WITHOUT MONEY" AS A FIRST-CLASS PRIMITIVE

### The Core Insight

**Most coordination doesn't need money, it needs commitment.**

Traditional systems conflate:
- **Commitment** (binding promise) → Requires money as collateral
- **Payment** (value transfer) → Requires money as medium

In Relay, these are separate:
- **Commitment** = Causal commit with authority_ref (no money needed)
- **Payment** = CURRENCY_TRANSFER commit (money moves)

---

### Commitment Primitive Design

```rust
// New filament type: Commitment
FilamentType::Commitment {
    commitment_type: CommitmentType,
    enforcement: EnforcementPolicy,
}

enum CommitmentType {
    Promise,          // "I will do X"
    Obligation,       // "I must do X or penalty Y"
    Delegation,       // "You may do X on my behalf"
    Guarantee,        // "I back X if it fails"
}

enum EnforcementPolicy {
    SelfReporting,    // Honor system (reputation risk only)
    ThirdPartyArbitration(Vec<UnitId>), // Designated arbiters
    AutomaticPenalty(Penalty), // Smart contract-like rules
}

struct Penalty {
    penalty_type: PenaltyType,
    amount: Option<u64>, // For financial penalties
    action: String,      // For non-financial penalties
}

enum PenaltyType {
    ReputationSlash(i32), // Lose reputation points
    FinancialBurn(u64),   // Burn tokens
    AuthorityRevoke,      // Lose delegated authority
    PublicShame,          // Visible scar on unit's history
}
```

---

### Example 1: Promise Without Money

```rust
// Alice promises to deliver a report by Friday
CommitEvent {
    filament_id: "commitment.promise.001",
    op_type: "COMMITMENT_PROMISE",
    author_unit_ref: "unit.alice.001",
    payload: {
        "promise": "Deliver Q1 report by 2026-01-31",
        "beneficiary": "unit.bob.002",
        "deadline": "2026-01-31T23:59:59Z",
        "penalty": {
            "penalty_type": "ReputationSlash",
            "amount": -50, // Lose 50 reputation points if broken
        }
    },
    causal_refs: {
        authority_ref: "unit.alice.001@reputation", // Alice stakes her reputation
        ...
    }
}
```

**No money involved.** Alice commits by staking **reputation**, not cash.

---

### Example 2: Obligation With Financial Penalty

```rust
// Bob guarantees server uptime, stakes 100 REL
CommitEvent {
    filament_id: "commitment.sla.001",
    op_type: "COMMITMENT_OBLIGATION",
    author_unit_ref: "unit.bob.002",
    payload: {
        "obligation": "99.9% server uptime for 2026-Q1",
        "beneficiary": "unit.acme.corp",
        "deadline": "2026-03-31T23:59:59Z",
        "penalty": {
            "penalty_type": "FinancialBurn",
            "amount": "100.000000" // Burn 100 REL if SLA broken
        }
    },
    causal_refs: {
        authority_ref: "currency.REL@c123", // Bob locks 100 REL as collateral
        ...
    }
}
```

**Money is collateral**, not payment. If Bob succeeds, REL is unlocked. If Bob fails, REL is burned (not transferred).

---

### Example 3: Delegation Without Money

```rust
// Alice delegates code review authority to Bob (no payment)
CommitEvent {
    filament_id: "commitment.delegation.001",
    op_type: "COMMITMENT_DELEGATE",
    author_unit_ref: "unit.alice.001",
    payload: {
        "delegated_authority": "code_review",
        "delegatee": "unit.bob.002",
        "scope": "project.relay-physics",
        "expires_at": "2026-12-31T23:59:59Z"
    },
    causal_refs: {
        authority_ref: "unit.alice.001@code_review_authority",
        ...
    }
}
```

**No money.** Just authority transfer (Bob can now approve code on Alice's behalf).

---

### Why This Matters: Most Coordination is Non-Monetary

**Examples of commitments that don't need money:**
- ✅ "I'll review your PR by Friday" (time commitment)
- ✅ "I vouch for Alice's work" (reputation commitment)
- ✅ "I'll maintain this server" (service commitment)
- ✅ "I approve this decision" (authority commitment)

**Traditional systems force these into money:**
- ❌ Pay reviewer (makes commitment transactional)
- ❌ Buy reputation (artificial scarcity)
- ❌ Charge for server (hides time commitment)
- ❌ Hire approver (centralizes authority)

**Relay allows pure commitment:** Stake reputation, authority, or time - not money.

---

## PART 10: LOCKED ECONOMIC PRIMITIVES (SUMMARY)

### 1. Authority Tokens (Money)
```rust
FilamentType::Currency { ... }
Operations: CURRENCY_ISSUE, CURRENCY_TRANSFER, CURRENCY_BURN
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
CommitType::CURRENCY_DELEGATE, COMMITMENT_DELEGATE
Properties: Temporary, revocable, conditional, transparent
```

---

## LOCKED INVARIANTS (ECONOMIC SUBSTRATE)

1. **Money = Transferable Authority Tokens** (currency filaments)
2. **Credit = Visible Authority Delegation** (no fractional reserve)
3. **Voting ≠ Money** (attention signals, not scarcity)
4. **Commitment ≠ Payment** (stake reputation/authority, not cash)
5. **All Economic Actions = Commits** (transparent, immutable, auditable)
6. **No Phantom Money** (full-reserve, all tokens accounted for)
7. **Central Banks = Transparent Issuers** (no hidden printing)
8. **Banking Functions = Coordination Services** (not gatekeepers)

---

## CAUSAL REFS

- **Inputs:** `architecture@c0` (Layer split), `architecture@c2` (SSE truth stream)
- **Authority:** system.architect
- **Evidence:**
  - `authority_store.rs` (authority primitives already exist)
  - `agent_ops.rs` (operation primitives already exist)
  - Traditional banking literature (fractional reserve, credit creation)
  - Cryptocurrency principles (transparent ledgers, full-reserve)

---

## NEXT STEPS

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

---

**Status:** LOCKED  
**Supersedes:** None (new economic substrate)  
**Next:** Implement PR #2 (Monetary Filaments)

---

**Philosophy:** **Truth is the substrate. Money, credit, voting, and commitments are just coordination data on that substrate.**

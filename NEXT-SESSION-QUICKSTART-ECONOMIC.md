# Next Session Quickstart - Economic Substrate

**Last Session:** 2026-01-28 (Economic & Coordination Substrate Lock)  
**Status:** Architecture complete, implementation ready  
**Quick Reference:** Start here for next session

---

## CURRENT STATE (WHAT'S DONE)

### ✅ Layer 2 Backend (Relay Physics)
- Base physics layer (commits, filaments, verification)
- Persistence + restart determinism
- SSE replay + Last-Event-ID
- Single commit fetch (forensic)
- `/render/*` endpoints (RenderSpec v1) ⚠️ **Needs testing**

**Files:** `apps/server/src/relay_physics/` (~4,000 lines)

---

### ✅ Architecture Filaments (All Locked)
- `c0` - Layer 2/3 rendering split
- `c1` - Render endpoints contract
- `c2` - SSE truth stream locks
- `c3` - RenderSpec v1 stub
- `c4` - Execution order decision
- `c5` - RenderSpec v1 locked
- **`c6` - Economic primitives foundation** ⬅️ NEW
- **`c7` - Coordination gauges (StarCraft HUD)** ⬅️ NEW
- **`c8` - Delegated decaying influence** ⬅️ NEW

**Files:** `relay/filaments/architecture/` (9 commits, ~25,000 lines)

---

### ✅ Gold Standard Documentation
- **Economic Substrate Locked** - Answers 9 key questions
- **Coordination Substrate Gold Standard** - Complete implementation guide (1,200 lines)
- **Session Summary** - What was accomplished

**Files:** `relay/*.md` (3 major docs)

---

## QUICK CONCEPTS REFERENCE

### The 4 Coordination Primitives

1. **Authority Tokens (Money)** - Scarce, transferable, zero-sum
2. **Attention Signals (Voting)** - Abundant, non-transferable, positive-sum
3. **Reputation (Merit)** - Scarce, non-transferable, merit-based
4. **Commitments (Promises)** - Binding, visible, enforceable

---

### The 5 Coordination Gauges (StarCraft HUD)

1. 🟦 **Legitimacy** - Votes / support (decaying)
2. 🟨 **Commitment Capacity** - Obligation bandwidth (capped)
3. 🟩 **Escrowed Resources** - Money / tokens (locked)
4. 🟪 **Time Pressure** - Countdowns / cooldowns (universal)
5. 🟥 **Risk / Scars** - Disputes / failures (permanent)

---

### Delegated Decaying Influence (DDI)

**What:** Voters lend legitimacy to an agent temporarily.  
**How:** Influence decays unless renewed. Agent spends influence to act.  
**Why:** Authority is borrowed continuously, not granted once.

---

## WHAT TO DO NEXT (3 OPTIONS)

### Option A: Test Render Endpoints ⚡ FASTEST
**What:** Validate `/render/world`, `/render/commit/:ref`, `/render/filament/:id`  
**Time:** 1-2 hours  
**Commands:**
```bash
cd apps/server
cargo run

# In another terminal:
curl http://localhost:3002/api/relay-physics/render/world | jq .
```

**Expected:** JSON scene graph (globe + units + filaments)

**Start phrase:**
```
"Resume Option C - backend testing"
```

---

### Option B: Implement Currency (PR #2) 🏆 MOST SIGNIFICANT
**What:** Build `FilamentType::Currency` + balance tracking  
**Time:** 4-6 hours  
**Deliverables:**
- `apps/server/src/relay_physics/currency_store.rs`
- `CURRENCY_ISSUE`, `CURRENCY_TRANSFER`, `CURRENCY_BURN`, `CURRENCY_DELEGATE`
- Balance tracking (derived from commits)
- Unit tests

**Start phrase:**
```
"Start PR #2 - Currency Filaments"
```

---

### Option C: Complete Full Stack 🚀 AMBITIOUS
**What:** Test render → Implement economic (PR #2-5) → Build HUD  
**Time:** 26-35 hours  
**Result:** Complete Layer 2 + Layer 3 (render + economics + UI)

**Start phrase:**
```
"Complete full stack - render, economic, HUD"
```

---

## KEY FILES TO REFERENCE

### Architecture Specs
```
relay/filaments/architecture/
├── 0006_economic_primitives_foundation.md     (Money, credit, banking)
├── 0007_coordination_gauges_hud.md            (5 gauges, StarCraft HUD)
└── 0008_delegated_decaying_influence.md       (DDI mechanics)
```

### Implementation Guides
```
relay/
├── ECONOMIC-SUBSTRATE-LOCKED.md               (Quick answers)
└── COORDINATION-SUBSTRATE-GOLD-STANDARD.md    (Complete spec)
```

### Backend Code
```
apps/server/src/relay_physics/
├── renderspec.rs                              (Scene graph types)
├── renderspec_generator.rs                    (Render logic)
├── main_integration.rs                        (Endpoints + handlers)
└── [TO BE BUILT]:
    ├── currency_store.rs                      (PR #2)
    ├── vote_store.rs                          (PR #3)
    ├── commitment_store.rs                    (PR #4)
    └── delegation_store.rs                    (PR #5)
```

---

## QUICK TESTING COMMANDS

### Test Render Endpoints
```bash
# Start server
cd apps/server && cargo run

# Test world scene
curl http://localhost:3002/api/relay-physics/render/world | jq .

# Create a unit
curl -X POST http://localhost:3002/api/relay-physics/units \
  -H "Content-Type: application/json" \
  -d '{"unit_id":"unit.test.001"}'

# Test world scene again (should show unit)
curl http://localhost:3002/api/relay-physics/render/world | jq '.nodes[] | select(.kind=="unit")'

# Test commit scene
curl http://localhost:3002/api/relay-physics/render/commit/work.W123@c1 | jq .
```

---

### Test SSE Stream
```bash
# Subscribe to events (leave running)
curl -N http://localhost:3002/api/relay-physics/events

# In another terminal, create a unit
curl -X POST http://localhost:3002/api/relay-physics/units \
  -H "Content-Type: application/json" \
  -d '{"unit_id":"unit.alice.001"}'

# Should see UnitStateChanged event in SSE stream
```

---

## PHILOSOPHICAL QUICK REFERENCE

### Core Transformations

**Money:**
- Traditional: Opaque ledgers, trust banks
- Relay: Transparent commits, zero trust

**Voting:**
- Traditional: Choose once, hope for best
- Relay: Lend authority proportionally, temporarily

**Authority:**
- Traditional: Granted once, permanent
- Relay: Borrowed continuously, decaying

**Banking:**
- Traditional: Gatekeepers, hidden leverage
- Relay: Services, transparent coordination

---

## DECISION TREE

```
Start here
    │
    ├─> Need visible demo fast?
    │   └─> Option A (test render) → 1-2 hours
    │
    ├─> Want real economic system?
    │   └─> Option B (currency) → 4-6 hours
    │
    └─> Want complete system?
        └─> Option C (full stack) → 26-35 hours
```

---

## SESSION START PHRASES

Just send ONE LINE to continue:

1. **`"Resume Option C - backend testing"`** → Test render endpoints (fastest demo)
2. **`"Start PR #2 - Currency Filaments"`** → Build economic layer (most significant)
3. **`"Complete full stack - render, economic, HUD"`** → Do everything (ambitious)

---

## LOCKED INVARIANTS (QUICK REFERENCE)

### Economic
- ✅ Money = Transferable authority tokens
- ✅ Credit = Visible authority delegation
- ✅ Voting ≠ Money (attention vs scarcity)
- ✅ Commitment ≠ Payment (reputation, not cash)

### Coordination Gauges
- ✅ 5 orthogonal gauges (legitimacy, commitments, escrow, time, disputes)
- ✅ No single power metric
- ✅ StarCraft HUD model

### Delegated Influence
- ✅ Authority is borrowed continuously
- ✅ Influence decays unless renewed
- ✅ Spending is visible and attributable

---

**Current Status:** ✅ Architecture complete, implementation ready  
**Next Step:** Choose path and start  
**Reference:** `architecture@c6-c8` + gold standard docs

---

**Quick Summary:**  
We built the economic and coordination substrate. Now we implement it.  
**Gauges, not wallets. Constraints, not wealth. Borrowed legitimacy, not granted power.**

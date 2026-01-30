# HOW TO READ RELAY

**Purpose:** Cognitive ergonomics for navigating Relay documentation  
**Audience:** Everyone (read this first)  
**Time:** 10 minutes

---

## 🎯 CRITICAL UNDERSTANDING

### **Text Is Never the Source of Truth**

**This changes everything:**
- Screen displays text
- But clicks navigate **filaments**
- Understanding comes from **physics**
- Text is for communication
- Filaments are for truth

**Corollary:** When you read Relay docs, you're reading **projections** of filament physics, not the physics itself.

---

## 📐 THE THREE LAYERS OF READING

### **Layer 0: Headers (Default)**
**What you see:** Bullets, summaries, "what changed"  
**Time:** Seconds  
**Purpose:** Operational awareness (HUD level)

**Example:**
```
## Task Completed
- Task: Budget approval
- Status: Delivered
- Next: Audit verification
```

---

### **Layer 1: Full Proof (Expand When Needed)**
**What you see:** Paragraphs, reasoning, evidence chains  
**Time:** Minutes  
**Purpose:** Understanding "why"

**Example:**
```
## Task Completed (Expanded)

Budget approval task (task.budget_q1_eng) has moved from InTransit to Delivered.

Evidence:
- Approval commit: c142 (2026-01-29T12:00:00Z)
- Delegated authority: verified via identity binding
- Escrow released: $10,000 USD
- MeaningFrame alignment: 0.98 (all translations verified)

Next action: Audit verification required (materiality > $10k threshold).
```

---

### **Layer 2: Evidence (Audit Trail)**
**What you see:** Commit refs, filament IDs, verification chains  
**Time:** Minutes to hours  
**Purpose:** Audit, dispute resolution, forensics

**Example:**
```
## Task Completed (Evidence Trail)

Commit refs:
- Task creation: commit.task_store@c045
- Approval: commit.identity.manager_alice@c142
- Escrow lock: commit.currency.acme@c089
- Escrow release: commit.currency.acme@c143

Filament refs:
- Task filament: task.budget_q1_eng.jsonl
- Identity filament: identity.manager_alice.jsonl
- Currency filament: currency.acme.usd.jsonl

Audit trail:
- MaterialityRule: mat.financial.transaction_10k
- AuditAssertion: assert.task_045.completion (Verified)
- AuditEvidence: ev.escrow_release.bank_conf
```

**You can click through to any commit or filament.**

---

## 🔗 FILAMENTS VS TEXT

### **What Is a Filament?**
A filament is:
- Append-only value identity
- Carried through time
- Conserved (not created/destroyed arbitrarily)
- The source of truth

**Examples:**
- `task.budget_q1_eng` (task lifecycle)
- `identity.alice` (person's history)
- `building.apple_store.nyc_001` (building state)

---

### **How Text Relates to Filaments:**

**Text displays:**
- Derived state (current)
- Summaries (compressed)
- Translations (multi-language)
- Explanations (for humans)

**Filaments contain:**
- Commits (atomic events)
- Causal refs (dependencies)
- Evidence (verification)
- Complete history

**Rule:** When in doubt, click through to the filament.

---

## 🎮 THE SELECT / HOLD / BIND MODEL

### **NO COPY-PASTE IN RELAY**

**Why:** Copy-paste is a lie in a world of conserved history.

**Instead:** Select → Hold → Bind

---

### **1. SELECT**
**What it does:** Choose a filament reference

**Example:**
- Click "Budget approval task"
- Selection highlights object
- System knows you're referencing `task.budget_q1_eng`

---

### **2. HOLD**
**What it does:** Tether object to your focus

**Visual:**
- Object stays ghosted/highlighted
- Connected to cursor with visual line
- Indicates "active reference"

**Semantic:**
- System tracks held object
- Shows allowed operations
- Prevents losing context

---

### **3. BIND**
**What it does:** Create relationship to held object

**Examples:**
| Drop Target | Result |
|-------------|--------|
| SCV task list | Bind object → task context (SCV gets full context) |
| Governance proposal | Reference object as evidence |
| Session | Attach object to session (participants can inspect) |
| Audit finding | Link as remediation evidence |
| Simulation | Project object into scenario |

**Each bind:**
- Creates explicit relationship
- Logged as commit
- Auditable
- Reversible

**No duplication. Only references.**

---

## 🔍 FOLLOWING DEPENDENCY CHAINS

### **Every Object Has:**
- `commit_ref` (when it was created/updated)
- `filament_ref` (which filament it belongs to)
- `causal_refs` (what caused this)

### **To Understand "Why Something Exists":**

**Step 1:** Find the object  
**Step 2:** Check `commit_ref`  
**Step 3:** Load commit, check `causal_refs`  
**Step 4:** Follow causal chain backward  
**Step 5:** Reach foundational decision or evidence

**Example:**
```
Why does this task exist?

task.budget_q1_eng
  ↓ commit_ref
commit.task_store@c045
  ↓ causal_refs
[commit.session.finance_review@c038, commit.identity.alice@c031]
  ↓ trace further
session.finance_review
  ↓ commit_ref
commit.session_store@c038
  ↓ causal_refs
[commit.building.acme_hq@c015, commit.identity.alice@c020]

Conclusion: Task created during finance review session at ACME HQ, 
initiated by Alice's commitment.
```

**Every "why" has a traceable answer.**

---

## 🌲 UNDERSTANDING DISAGREEMENTS

### **Disagreements Are Explicit Objects**

**In Relay, disagreement = fork:**
- Two MeaningFrame interpretations
- Two bounce chain branches
- Two translation alternatives
- Two audit opinions

**Forks require SelectionCommit:**
- User (or authorized role) chooses winner
- Rejected branch becomes scar
- Both branches preserved in history

**Example:**
```
Prompt: "Implement audit system"
  ↓
SCV-A suggests: "Build all 6 objects first"
SCV-B suggests: "Thin vertical slice first"
  ↓
Fork created (BounceFork)
  ↓
User SelectionCommit: Choose SCV-B
  ↓
SCV-A branch = scar (preserved but not active)
```

**No hidden consensus. No silent merge. Explicit selection always.**

---

## 📊 LOD (LEVEL OF DETAIL) RELEVANCE

### **One Global Function**

**All objects use same LOD calculation:**

**Factors:**
1. Distance (meters from viewer)
2. Time recency (seconds since update)
3. Materiality (c16 MaterialityLevel)
4. Zone visibility (c13 rules)
5. Viewer permissions (c14 presence tier)
6. Active lens (which question/filter)

**Output:** LOD0, LOD1, LOD2, or LOD3

**Examples:**
- **LOD0:** Exists, coarse region
- **LOD1:** Class + region
- **LOD2:** Movement vectors + status
- **LOD3:** Full detail

**Applies to:**
- Buildings
- ForceUnits
- Sessions
- Audit findings
- MeaningFrames
- Everything renderable

---

## 🔗 CROSS-LINK FORMAT

### **All Documents Use Standard Footer:**

```markdown
---
**Refs:** [c0.Filaments], [c7.Tasks], [c16.Audit]
**Objects:** [Task], [Commit], [AuditAssertion]
**Audit:** [Completeness], [Traceability]
```

**This enables:**
- Claim tracing (every claim → physics)
- Dependency validation (automated checks)
- Audit verification (provenance)

**Rule:** Every non-trivial claim MUST have refs.

---

## 🎯 COMMON READING PATTERNS

### **Pattern 1: "What is X?"**
1. Check `RELAY-OBJECTS-REFERENCE.md`
2. Find object definition
3. Read "What it is" / "What it is NOT"
4. Check governing architecture commits

---

### **Pattern 2: "How do I do X?"**
1. Check `RELAY-CONTEXT-MAP.md` (this page's parent)
2. Find your role's guide
3. Follow reader path
4. Reference objects as needed

---

### **Pattern 3: "Why does X work this way?"**
1. Find object in `RELAY-OBJECTS-REFERENCE.md`
2. Check "Governed by" architecture commits
3. Read those architecture specs
4. Trace invariants

---

### **Pattern 4: "Is X allowed?"**
1. Check relevant architecture (c13 for zones, c16 for audit, etc.)
2. Look for explicit forbids
3. Check zone rules if spatial
4. If not mentioned, check meta-locks

---

## 🚫 READING ANTI-PATTERNS

**Don't:**
- ❌ Read linearly (start to end)
- ❌ Read everything (pick your path)
- ❌ Skip this guide (critical foundation)
- ❌ Trust narrative without refs
- ❌ Assume text = truth

**Do:**
- ✅ Start with your role/intent
- ✅ Follow reader paths
- ✅ Check cross-link refs
- ✅ Zoom into physics when needed
- ✅ Trace dependencies

---

## ✅ READING CHECKLIST

**Before proceeding, verify you understand:**
- [ ] Text is projection, filaments are truth
- [ ] Three layers exist (headers, proof, evidence)
- [ ] No copy-paste (select, hold, bind)
- [ ] Dependencies are traceable (commit_ref, causal_refs)
- [ ] LOD is universal (one function)
- [ ] Cross-links are mandatory (every claim → ref)

**If yes to all:** You're ready to navigate Relay.

---

**Refs:** [c0.Filaments], [c12.Headers/Proof]  
**Objects:** [Filament], [Commit]  
**Next:** Choose your reader path in RELAY-CONTEXT-MAP.md

**END OF READING GUIDE**

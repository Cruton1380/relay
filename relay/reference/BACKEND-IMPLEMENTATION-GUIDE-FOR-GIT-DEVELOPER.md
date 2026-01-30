# RELAY BACKEND IMPLEMENTATION GUIDE

**For: Ari (Git/Replication Expert)**  
**From: System Architect**  
**Type:** Technical Implementation Specification  
**Status:** 🔒 Canonical  
**Date:** 2026-01-29

---

## 🎯 EXECUTIVE SUMMARY FOR BACKEND DEVELOPERS

**What you're building:**  
A Git-native coordination OS where the commit DAG is the substrate for all truth, and reconciliation is enforced by physics, not process.

**What makes this different from "another Git wrapper":**  
1. Git is not history storage—it's the **operating system of the present**
2. Branches are not dev artifacts—they're **visible states of reality**
3. Reconciliation is not optional—it's **enforced at commit-time**
4. The 3D globe renders **live commit graph state**, not analytics

**Your job:**  
Build the backend that makes this real. Start with the smallest runnable slice, prove the model works, then scale.

---

## 📊 THE 2D → 3D JUMP (PRECISE DEFINITION)

### **2D Thinking (Current Systems)**

```
Git stores history
  ↓
Apps read "latest" from DB
  ↓
UI shows state
  ↓
Truth = whatever UI displays
  ↓
Reconciliation = human work (tickets, emails, meetings)
  ↓
Failure = state diverges silently
```

**Example:**
- Code in Git repo
- Config in SharePoint
- Permissions in SAP
- Decisions in email
- Status in Jira

**Nobody guarantees consistency.** Humans are the glue.

---

### **3D Thinking (Relay)**

```
Git is the operating system
  ↓
Commits are discrete events (everything)
  ↓
Branches are visible operating states
  ↓
Canon = active branch (voted, time-stamped, reversible)
  ↓
Reconciliation = enforced (no silent success)
  ↓
Truth = visible commit DAG + canon pointers
```

**Example:**
- Everything is a commit (votes, decisions, permissions, evidence)
- State = branch tips + forks preserved
- Changes = new commits (append-only)
- Reconciliation = required acknowledgment

**Git guarantees consistency.** No human glue needed.

---

## 🔒 CORE ARCHITECTURAL DECISIONS (LOCKED)

### **Decision 1: Single Canonical State = Git Commit Log**

**Old way:**
```
State Location 1: Laptop
State Location 2: Production DB
State Location 3: SharePoint
State Location 4: Cache
State Location 5: Client's system

Reconciliation paths: 5² = 25 loops
```

**Relay way:**
```
State Location: Git commit log (append-only)
Projections: Derived views (read-only)

Reconciliation paths: 0
```

**Implementation:**
- All truth lives in `.relay/` structured commits
- Everything else is a query/projection
- No external databases for truth
- Caches allowed, but always derived

---

### **Decision 2: Commits Are Discrete Events**

**Not just code changes. Everything:**
- Vote cast = commit
- Permission granted = commit
- Evidence attached = commit
- Canon selected = commit
- Authority delegated = commit
- User action = commit

**Schema:**
```json
{
  "commit_id": "abc123...",
  "commit_type": "VOTE_CAST" | "DELEGATION_CREATED" | "CANON_SELECTED" | ...,
  "timestamp": "2026-01-29T14:32:18Z",
  "commit_index": 12345,
  "author_unit_ref": "unit.user.james@c100",
  "filament_id": "history.topic.french_revolution",
  "payload": { /* operation-specific */ },
  "causal_refs": {
    "inputs": ["commit.prev@c12344"],
    "authority_ref": "delegation.alice@c100"
  }
}
```

**Key properties:**
- Append-only (no edits)
- Causally linked (refs to inputs)
- Authority explicit (no ambient)
- Deterministic (replay gives same result)

---

### **Decision 3: Evidence ≠ Belief (Critical)**

**This prevents "votes rewrite reality":**

**Evidence filaments:**
```
Append-only record
Immutable
Never voted away
Example: Raw telescope data, financial records, logs
```

**Belief/canon filaments:**
```
Vote-selected "working model"
Points to a branch tip
Reversible (can flip to different branch)
Example: "Economic crisis caused French Revolution" (current canon)
```

**Git analogy:**
- Evidence = commits (never deleted)
- Belief = which branch is "main" right now
- Forks = alternative interpretations (preserved)

**Implementation:**
```rust
struct EvidenceFilament {
    filament_id: String,
    commits: Vec<Commit>,  // Append-only
    immutable: true,
}

struct BeliefFilament {
    filament_id: String,
    branches: Vec<Branch>,
    active_canon: BranchRef,  // Vote-selected
    canon_history: Vec<CanonChange>,  // Time-stamped transitions
}
```

---

### **Decision 4: Mandatory Reconciliation (THE KEYSTONE)**

**This is replication semantics applied to meaning.**

**Rule:**
> No write becomes canonical until all required mirrors acknowledge it.

**Git parallel:**
```
Git:
  Push commit
  Wait for remote to acknowledge
  If no ack → push failed
  If ack → commit replicated

Relay:
  Append commit
  Wait for required mirrors to acknowledge
  If no ack → HOLD state
  If ack → RECONCILED state
```

**Implementation:**
```rust
struct Commit {
    commit_id: String,
    required_mirrors: Vec<MirrorLocation>,
    acknowledgments: Vec<Acknowledgment>,
    status: CommitStatus,
}

enum CommitStatus {
    Pending,      // Not all mirrors ack'd
    Hold,         // Timeout or conflict
    Reconciled,   // All mirrors ack'd
    Failed,       // Explicit failure
}

fn is_canonical(commit: &Commit) -> bool {
    commit.acknowledgments.len() == commit.required_mirrors.len()
        && commit.status == CommitStatus::Reconciled
}
```

**Concrete example (Firefighter Access):**
```
1. Admin grants permission (commit A)
2. System writes to auth store (commit B)
3. User acknowledges receipt (commit C)
4. Audit log confirms (commit D)

State: HOLD
  ↓
All 4 acknowledged?
  ↓
State: RECONCILED (permission active)

Missing ack?
  ↓
State: HOLD (permission NOT active)
  ↓
Timeout?
  ↓
State: FAILED (rollback)
```

**This is why SAP permissions don't "stay open" in Relay.**

---

## 🎮 STOP / HOLD / FORK = COMMIT CONTROL (NOT UX)

**These are first-class operations on the commit graph:**

### **HOLD**
```rust
fn hold_execution_path(commit_ref: CommitRef) -> HoldState {
    // Pause execution, preserve state
    // No silent continuation
    // Resume requires explicit commit
    HoldState {
        paused_at: commit_ref,
        resume_conditions: Vec<Condition>,
        timeout: Option<Duration>,
    }
}
```

**Use case:** AI agent needs user confirmation before proceeding

---

### **STOP**
```rust
fn stop_execution_path(commit_ref: CommitRef, reason: String) -> StopCommit {
    // Cancel and mark incomplete
    // No pretending it finished
    StopCommit {
        stopped_at: commit_ref,
        reason,
        incomplete: true,
        refs: vec![commit_ref],
    }
}
```

**Use case:** User cancels in-progress operation

---

### **FORK**
```rust
fn fork_execution_path(commit_ref: CommitRef, new_branch: String) -> ForkCommit {
    // Branch alternative path
    // Explicit lineage preserved
    ForkCommit {
        forked_from: commit_ref,
        branch_name: new_branch,
        divergence_reason: String,
        refs: vec![commit_ref],
    }
}
```

**Use case:** User wants to try alternative decision without losing current path

---

**Why this matters:**  
Agent drift happens today because there's no forced reconciliation checkpoint.  
HOLD/STOP/FORK make reconciliation explicit and unavoidable.

---

## 🔥 VOTE TURBULENCE = BRANCH VELOCITY RENDERED

**This should map cleanly to your replication brain:**

### **The Model**

```
Votes/actions = commits/events (discrete)
  ↓
Compute signals:
  - vote_velocity = Δvotes / Δtime
  - vote_acceleration = Δvelocity / Δtime
  - authority_transfer_rate = Σ(influence_spent) / Δtime
  - event_rate = count(forks + scars) / Δtime
  - causal_footprint = count(refs_to_this_branch)
  ↓
Render as thermal/force field around branches
  ↓
LOD controls frame rate:
  - Near: 30-60fps
  - Mid: 10-30fps
  - Far: 1-10fps (batch updates)
```

**Key separation:**
- **Ledger = exact, continuous** (every commit recorded)
- **Projection = bounded, sampled** (rendering at LOD)

**Git parallel:**
```
Git:
  Commits happen continuously
  UI updates on refresh
  History is exact, view is sampled

Relay:
  Commits happen continuously
  Thermal field updates at LOD
  Ledger is exact, rendering is bounded
```

**Implementation:**
```rust
struct ThermalSignals {
    vote_velocity: f32,      // commits/second
    vote_acceleration: f32,  // Δvelocity/second
    authority_rate: f32,     // influence/second
    event_rate: f32,         // events/second
    footprint: u32,          // ref count
}

fn compute_thermal_field(
    branch_ref: BranchRef,
    time_window: Duration,
) -> ThermalSignals {
    let commits = get_commits_in_window(branch_ref, time_window);
    let velocity = commits.len() as f32 / time_window.as_secs_f32();
    // ... compute other signals
}

fn render_at_lod(
    camera_distance: f32,
    signals: ThermalSignals,
) -> RenderUpdate {
    let update_rate = if camera_distance < 100.0 {
        60.0  // fps
    } else if camera_distance < 1000.0 {
        30.0
    } else {
        10.0
    };
    
    RenderUpdate {
        signals,
        update_rate,
        // ... shader params
    }
}
```

---

## 🔐 CONCRETE PROFESSIONAL EXAMPLE: FIREFIGHTER ACCESS

### **Old World (Broken)**

```
1. Ticket: "Grant firefighter access to SAP"
2. Admin: Grants access in SAP
3. SAP logs: Show activity
4. Ticket: "Closed" (manual step)
5. Reality: Access still active (forgotten)

Evidence in different places:
  - Ticket system (Jira)
  - SAP logs
  - Approver email
  - Closure "expected"

Reconciliation: Human memory

Result: Exceptions age silently, audit finds violation months later
```

---

### **Relay (Fixed)**

```rust
struct FirefighterAccessGrant {
    commit_id: String,
    commit_type: "FIREFIGHTER_GRANT",
    
    payload: {
        user_id: "user.james",
        resource: "sap.system.prod",
        reason: "P1 incident: Database corruption",
        ttl_commit_index: 12500,  // Expires at commitIndex 12500
    },
    
    authority_ref: {
        delegation_ref: "delegation.manager.sarah@c12345",
        scope: "emergency.access",
    },
    
    required_closures: vec![
        "system.sap.ack",    // SAP confirms grant
        "user.james.ack",    // User confirms receipt
        "audit.log.ack",     // Audit records event
        "manager.sarah.ack", // Manager confirms closure
    ],
    
    acknowledgments: Vec<Ack>,  // Populated as acks arrive
    
    status: CommitStatus::Pending,
}
```

**Lifecycle:**
```
1. Commit created (Pending)
2. SAP acks (1/4)
3. User acks (2/4)
4. Audit acks (3/4)
5. State: HOLD (waiting for manager)
   ↓
   Timeout (24h)
   ↓
   Alert generated (visible turbulence)
   ↓
6. Manager acks (4/4)
7. State: RECONCILED
8. At commitIndex 12500: Expires automatically
9. Expired commit → No longer valid (deterministic)
```

**Key properties:**
- ✅ Access expires by commitIndex (not forgotten)
- ✅ Closure is a state transition (not a step)
- ✅ HOLD state is visible (generates alerts)
- ✅ All evidence in one commit (no fragmentation)

**This is closure as physics, not process.**

---

## 🧪 SMALLEST RUNNABLE SLICE (IMPLEMENTATION PLAN)

### **Phase 1: Prove Commit Model (1 week)**

**Goal:** Show that Git-native commits work for coordination, not just code.

**Deliverables:**
1. Commit schema (JSON)
2. Append-only commit writer
3. Query engine (read commits)
4. Step counter (monotonic commitIndex)

**Test case:**
```
Action: User casts vote
  ↓
Commit: VOTE_CAST with payload
  ↓
Query: Aggregate votes from commit history
  ↓
Result: Vote count matches commit sum
```

**Success criteria:**
- ✅ Commits are append-only
- ✅ Queries derive state from history
- ✅ No external DB for truth

---

### **Phase 2: Prove Reconciliation (1 week)**

**Goal:** Show that mandatory acknowledgment prevents drift.

**Deliverables:**
1. `required_mirrors` field in commits
2. `acknowledgments` tracking
3. HOLD state when incomplete
4. Timeout handler

**Test case:**
```
Action: Grant permission
  ↓
Commit: PERMISSION_GRANT (Pending)
  ↓
Wait for acks: [system, user, audit]
  ↓
2/3 ack'd after 30s
  ↓
State: HOLD (visible, generates alert)
  ↓
3/3 ack'd
  ↓
State: RECONCILED (permission active)
```

**Success criteria:**
- ✅ Commits don't become canonical without acks
- ✅ HOLD state is explicit
- ✅ Timeouts are handled

---

### **Phase 3: Prove Expiry (3 days)**

**Goal:** Show that authority expires deterministically.

**Deliverables:**
1. `expiry_commit_index` field
2. Validity check against current commitIndex
3. Auto-expiry (no manual cleanup)

**Test case:**
```
Action: Grant temp access (expires at commitIndex 1000)
  ↓
commitIndex advances to 1001
  ↓
Access check: INVALID (deterministic)
  ↓
No manual revocation needed
```

**Success criteria:**
- ✅ Expiry is deterministic (commitIndex, not time)
- ✅ No "stays open" bug possible
- ✅ No cleanup process needed

---

### **Phase 4: Prove Evidence/Belief Split (3 days)**

**Goal:** Show that votes select canon without erasing forks.

**Deliverables:**
1. Evidence filament (immutable commits)
2. Belief filament (branch pointers)
3. Canon selection commit
4. Fork preservation

**Test case:**
```
Evidence: "French Revolution data" (commits 1-100)
  ↓
Branch A: "Social inequality model" (234 votes)
Branch B: "Economic crisis model" (645 votes)
  ↓
Canon vote: Select Branch B
  ↓
Result:
  - Evidence unchanged ✅
  - Branch A preserved ✅
  - Branch B = canon ✅
  - Vote recorded with timestamp ✅
```

**Success criteria:**
- ✅ Evidence never changes
- ✅ Forks preserved
- ✅ Canon selection is reversible

---

### **Phase 5: Prove Thermal Field (1 week)**

**Goal:** Show that vote turbulence is just branch velocity.

**Deliverables:**
1. Signal calculations (velocity, acceleration, etc.)
2. Baseline computation (contextual normalization)
3. Heat intensity formula
4. LOD update scheduler

**Test case:**
```
Branch: "Climate policy"
  ↓
Votes: 100 → 500 in 1 hour (velocity spike)
  ↓
Baseline: 50 votes/hour (normal)
  ↓
Heat: (500-50)/50 = 900% deviation
  ↓
Alert tier: BATTLE (red force trails)
```

**Success criteria:**
- ✅ Signals computed from commit history
- ✅ Normalization is contextual
- ✅ LOD controls render rate
- ✅ Field never mutates ledger

---

## 📋 ATTACK VECTORS (FOR YOU TO CHECK)

**As a Git/replication expert, attack this model:**

### **Question 1: Consistency Constraints**

**Does this create impossible consistency guarantees?**

```
Example: If commit A requires ack from mirrors 1-5,
but mirror 3 is offline, does the system deadlock?

Answer: No. Timeout moves to HOLD state.
User sees explicit HOLD, can retry or cancel.
No silent failure.
```

**Your check:** Can you find a scenario where consistency is violated?

---

### **Question 2: Replication Reality**

**Does this violate how distributed systems actually work?**

```
Example: Network partition between mirrors.
How does reconciliation handle split-brain?

Answer: Commits don't become canonical until majority ack.
Minority partition stays in HOLD.
When partition heals, re-reconciliation required.
```

**Your check:** Does this break CAP theorem or other laws?

---

### **Question 3: Performance at Scale**

**Does waiting for acknowledgments kill performance?**

```
Example: 10,000 users voting simultaneously.
Does ack wait create bottleneck?

Answer: Acks are batched per time window.
LOD controls granularity (near=realtime, far=periodic).
Projections can lag, ledger remains exact.
```

**Your check:** Where would this choke at 10K, 100K, 1M users?

---

### **Question 4: Git Semantics**

**Does this abuse Git or stay true to its model?**

```
Example: Using commits for votes, not code changes.
Is this a Git anti-pattern?

Answer: Git is a content-addressable filesystem.
Content = any data (not just code).
Commits = discrete events (not just diffs).
This is Git-native, not Git-abusive.
```

**Your check:** Are we violating Git's design philosophy?

---

## 🎯 WHAT I WANT FROM YOU

### **1. Attack the model like a backend person**

- Where does it violate Git/replication reality?
- What parts create impossible consistency constraints?
- Where would performance choke?

---

### **2. Implement the smallest runnable slice**

**Start here:**
```
Phase 1: Prove commit model (1 week)
  - Commit schema
  - Append-only writer
  - Query engine
  - Vote aggregation test
```

**If that works, we're real.**

---

### **3. Tell me where this is still "2D"**

If you say "this part is hand-wavy," that's gold.  
I'll treat it as a bug and fix the model.

---

## 📚 REFERENCE DOCUMENTS (ALL LOCKED)

**Foundation:**
- `THE-RELAY-THESIS.md` - Why old world fails, why this works
- `RECONCILIATION-PROTOCOL.md` - The keystone primitive

**Architecture:**
- `VOTING-SYSTEM-ALIGNMENT.md` - Voting using DDI
- `AUTHORITY-DELEGATION-COMPLETE.md` - No ambient authority
- `GLOBE-TIME-SPACE-MODEL.md` - History as depth, space as distance

**Visualization:**
- `VOTE-TURBULENCE-VISUALIZATION-SPEC.md` - Thermal field = branch velocity

**Backend:**
- `BACKEND-LEGACY-AUDIT.md` - What we built before (preserve these)

---

## 🔒 FINAL STATEMENT

**This is not "social media with voting."**

**This is:**
- Git as the operating system
- Commits as discrete events
- Reconciliation as physics
- State as visible commit DAG
- Canon as vote-selected branch tip
- Everything legible in 3D

**It's what you already know about correct distributed work—applied to everything humans keep breaking.**

---

## ✅ SUCCESS CRITERIA

**You'll know we succeeded when:**
1. ✅ Commits work for coordination (not just code)
2. ✅ Reconciliation is enforced (not optional)
3. ✅ Authority expires deterministically (no drift)
4. ✅ Evidence ≠ belief (votes don't erase forks)
5. ✅ The model survives your attack

**Then we're real.**

---

**Status:** 🔒 **CANONICAL IMPLEMENTATION GUIDE**  
**Target:** Backend/Git expert  
**Purpose:** Rigorous explanation + first tasks  
**Next:** Prove Phase 1 (commit model)

**You showed me branching. Now let's make it real for everything.** 🎯

**END OF IMPLEMENTATION GUIDE**

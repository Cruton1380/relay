# architecture@c12: Root AI Cognitive Substrate

**Date:** 2026-01-29  
**Status:** 🔒 Locked  
**Depends on:** architecture@c11 (Root AI Coherence Layer), architecture@c10 (Identity filaments), architecture@c9 (StarCraft HUD), architecture@c2 (SSE Truth Stream)

---

## 🎯 THE PROBLEM

**Three interconnected failure modes in current AI agent systems:**

### **1. Context Loss + Copy/Paste Hell**
- Conversations get heavy and lag
- Hitting token limits forces manual copy/paste to new windows
- "Onboarding" new agents is manual document-passing
- No button, no commit-based training
- Context is lost or degraded in transfer

### **2. Hidden Steering + Keyword Stuffing**
- Users must "over-write" prompts with extra keywords to hit correct model pathways
- "Within biology and not philosophy, be rigorous and cite sources..."
- Steering is hidden, brittle, non-auditable
- Can't see what influenced an output
- Different words = different internal branches, but invisibly

### **3. Opaque Learning + Mysterious Changes**
- Models "learn" by changing internal weights silently
- No audit trail of what changed or why
- Can't replay "why did the model say X on date Y?"
- Reliability drift is invisible
- No way to know if an agent is getting better or worse at specific tasks

**These aren't UX problems. They're physics problems.**

Relay needs to apply its own coordination substrate to the AI conversation layer itself.

---

## 🧬 THE SOLUTION: ROOT AI COGNITIVE SUBSTRATE

### **Core insight:**
> **Conversations ARE filaments. AI training IS deterministic compilation. Steering IS explicit branch selection. Learning IS append-only artifact creation.**

**Three integrated architectural components:**

1. **Conversation as Filament + Agent Training**
   - Conversations = append-only commit nodes
   - Agent onboarding = spawn from commit via Training Pack
   - Header/proof zoom layers (humans operate at HUD level)

2. **Logic Branches + Visible Steering**
   - Root Logic has named branches (domains/lenses/constraints)
   - Users select branches explicitly (no keyword stuffing)
   - Influence Traces show what affected every output

3. **Root HUD Mechanics**
   - Root HUD = Compiler + Router + Auditor
   - Learning via append-only artifacts (not hidden training)
   - Profiles + Gates + Traces (not just raw weights)

---

## 🔑 PART A: CONVERSATION AS FILAMENT

### **The fundamental shift:**

**Before:** Conversations are stateless text exchanges, disposable after token limit  
**After:** Conversations are append-only filament history, replayable and branchable

---

### **A1: Commit Node Structure**

Every meaningful exchange becomes a commit node.

**CommitNode schema:**
```rust
struct CommitNode {
    commit_ref: String,           // "conv.user123.c47"
    timestamp: u64,
    author: String,               // "user.alice" or "scv.forecast_03"
    parent_refs: Vec<String>,     // causal chain
    content_type: ContentType,    // UserPrompt | AgentResponse | TrainingPack | ProfileChange
    content_hash: String,         // deterministic hash
    training_pack_ref: Option<String>,  // if this is an onboarding point
    weight_profile_ref: Option<String>, // active profile for this turn
    influence_trace_ref: Option<String>, // proof of what influenced this
    outcome_status: Option<OutcomeStatus>, // Accepted | Patched | Rejected
}

enum ContentType {
    UserPrompt,
    AgentResponse,
    TrainingPack,
    ProfileChange,
    GateViolation,
    CorrectionCommit,
}

enum OutcomeStatus {
    Accepted,      // user confirmed ✅
    Patched,       // required correction
    Rejected,      // blocked by gates
    Pending,       // not yet reviewed
}
```

**Invariant laws:**
- Commits are never mutated
- Commits may spawn descendant branches
- Parent commit remains intact
- Conservation holds (no history deletion)

---

### **A2: Agent Onboarding = Spawn from Commit**

**The "Onboard" button:**

```
ONBOARD_AGENT_FROM_COMMIT(
    commit_ref: String,        // "conv.user123.c47"
    role_type: AgentRole,      // Root | BranchRoot | Specialist
    scope: String,             // "scientific_domain" | "forecast_ops" | etc
    weight_profile: String     // "scientific_rigor" | "legal_conservative" | etc
) -> Result<AgentInstance, Error>
```

**What happens:**
1. System selects commit cutoff point
2. Compiles Training Pack (all history c0 → cutoff)
3. Binds Weight Profile (domain + constraints)
4. Spawns new SCV with sealed artifacts
5. SCV is "trained" (not just "prompted once")

**Critical invariant:**
> **The new agent does not rewrite history. The parent branch remains intact. This is descendant spawning, not ambiguous forking.**

---

### **A3: Training Packs = Deterministic Compilation**

**Training is not "give the agent a document."**

Training is:
> **Summarize → Validate → Compress → Seal**

**TrainingPack v1 schema:**
```rust
struct TrainingPack {
    pack_id: String,
    scope_id: String,
    cutoff_commit: String,
    created_at: u64,
    version: String,
    
    // Core content
    locked_invariants: Vec<Invariant>,      // physics laws (must never violate)
    active_decisions: Vec<Decision>,        // choices made (context)
    open_questions: Vec<Question>,          // explicit unknowns
    glossary: Vec<GlossaryEntry>,          // terms + meanings
    do_not_do: Vec<ForbiddenPattern>,      // drift prevention
    current_tasks: Vec<TaskSummary>,       // next steps
    
    // Proof + traceability
    citations: Vec<CommitRef>,             // anchors to exact commits
    reliability_score: f32,                // quality of source material
    
    // Compression metadata
    original_token_count: u64,
    compressed_token_count: u64,
    compression_method: String,
}

struct Invariant {
    invariant_id: String,
    statement: String,
    commit_ref: String,       // where it was locked
    violation_cost: String,   // "physics breaks" | "drift" | "invalid"
}

struct Decision {
    decision_id: String,
    question: String,
    chosen: String,
    rejected_alternatives: Vec<String>,
    rationale: String,
    commit_ref: String,
}

struct ForbiddenPattern {
    pattern_id: String,
    description: String,
    why_forbidden: String,
    example: String,
    commit_ref: String,
}
```

**Compilation process:**
1. **Extract:** Pull all commits from c0 to cutoff
2. **Cluster:** Group by topic (architecture, implementation, decisions)
3. **Validate:** Check for contradictions, missing refs
4. **Compress:** Headers + proof links (discard verbose filler)
5. **Seal:** Hash + sign, make immutable

**What gets compressed away:**
- Verbose conversation filler
- Repeated paraphrases
- Unverified speculation
- Style-only chatter
- Transient context that doesn't affect decisions

**What must remain:**
- Locked invariants (physics)
- Active decisions (choices made)
- Open questions (explicit unknowns)
- Commit refs (proof anchors)
- Forbidden patterns (drift prevention)

**This is mechanical, not creative.**

---

### **A4: Header/Proof Zoom Layers**

**The problem:**
> Humans don't need to read every paragraph, but the system must remain auditable.

**The solution:**
> **Dual-layer output structure**

**Every SCV response has:**

**Layer 0: Headers (default view)**
- One screen maximum
- Bullets only
- "What will be done" and "what changed"
- Action items
- Commit refs

**Layer 1: Proof (expandable on zoom)**
- Full paragraphs
- Reasoning chains
- Evidence links
- Commit refs + invariant checks
- Influence Trace details

**UI behavior:**
```
┌─────────────────────────────────────┐
│ ▼ SCV-Forecast-03 Response         │
│                                     │
│ ✅ Updated 3-month projection       │
│ ✅ Applied seasonality adjustment   │
│ ⚠️  Confidence: 78% (low data)     │
│                                     │
│ Citations: c41, c42, c44            │
│ Influence: Scientific 0.85          │
│                                     │
│ [Expand Proof ▼]                   │
└─────────────────────────────────────┘
```

**When expanded:**
```
┌─────────────────────────────────────┐
│ ▲ SCV-Forecast-03 Response (Full)  │
│                                     │
│ ## Updated 3-Month Projection       │
│ Applied seasonality model from c42  │
│ Input: raw quarterly data from...   │
│ Formula: trend * seasonal_factor... │
│ Validation: confidence bands...     │
│                                     │
│ ## Seasonality Adjustment           │
│ Based on historical pattern c41...  │
│ [full reasoning paragraphs...]      │
│                                     │
│ ## Confidence Analysis              │
│ 78% confidence due to sparse...     │
│ [detailed analysis...]              │
│                                     │
│ [Collapse ▲]                        │
└─────────────────────────────────────┘
```

**Invariant:**
> **Humans should be able to run Relay at "RTS HUD level" most of the time. The system remains auditable by zooming in.**

---

## 🔑 PART B: LOGIC BRANCHES + VISIBLE STEERING

### **The fundamental shift:**

**Before:** Steering is hidden keyword stuffing  
**After:** Steering is explicit branch selection + influence tracing

---

### **B1: Logic Branches as First-Class Objects**

**A Logic Branch is:**
> A stable, versioned unit of root cognition: a prior, heuristic, domain lens, safety rail, style constraint, or verification mode.

**LogicBranch schema:**
```rust
struct LogicBranch {
    branch_id: String,           // "scientific_rigor" | "no_metaphor" | "citations_required"
    name: String,
    description: String,
    version: String,             // commit-addressable
    created_at: u64,
    
    // Behavior modifiers
    constraints: Vec<Constraint>,      // hard rules
    biases: Vec<Bias>,                 // soft preferences
    validation_mode: ValidationMode,   // how to check outputs
    
    // Metadata
    domain: String,              // "science" | "legal" | "engineering" | "UI"
    strength: BranchStrength,    // Hard | Soft | Advisory
}

struct Constraint {
    constraint_id: String,
    rule: String,                // "Citations required for claims"
    violation_action: Action,    // Block | Warn | Log
    examples: Vec<String>,
}

struct Bias {
    bias_id: String,
    preference: String,          // "Prefer deterministic algorithms"
    strength: f32,               // 0.0-1.0
}

enum ValidationMode {
    CitationsRequired,
    MathCheck,
    NoSpeculation,
    NoMetaphor,
    NoSemanticZoom,
    DeterministicOnly,
}

enum BranchStrength {
    Hard,        // Must be followed (gate)
    Soft,        // Prefer but not required
    Advisory,    // Informational only
}
```

**Example branches:**
- **Scientific Rigor:** Citations required, no speculation, math check
- **No Metaphor:** Literal language only, no analogies unless explicitly marked
- **Systems Engineering:** Deterministic, testable, no magic
- **Legal Conservative:** Cite statutes, no interpretation, flag ambiguity
- **UI Minimalism:** Fewer elements, more whitespace, no decoration
- **No Semantic Zoom:** Zoom = lens change only, never reinterpretation

**These are versioned and commit-addressable:**
```
branch.scientific_rigor@v1.2
branch.no_metaphor@v2.0
```

---

### **B2: Weight Profiles = Explicit Steering**

**A Weight Profile is:**
> A normalized set of weights applied to Logic Branches, making steering explicit and auditable.

**WeightProfile schema:**
```rust
struct WeightProfile {
    profile_id: String,
    name: String,
    description: String,
    version: String,
    created_at: u64,
    created_from_commit: String,
    
    // The weights
    weights: HashMap<String, f32>,  // branch_id -> weight (0.0-1.0)
    normalization_rule: NormRule,   // Sum1 | Max1 | None
    
    // Metadata
    domain_scope: String,
    recommended_for: Vec<String>,   // task types
    do_not_use_for: Vec<String>,    // anti-patterns
    
    // Audit
    usage_count: u64,
    reliability_score: f32,         // based on outcomes
}

enum NormRule {
    Sum1,      // weights sum to 1.0
    Max1,      // largest weight = 1.0
    None,      // no normalization
}
```

**Example profiles:**

**Profile: "Scientific Domain"**
```rust
WeightProfile {
    profile_id: "profile.scientific_domain",
    name: "Scientific Domain",
    weights: {
        "scientific_rigor": 0.95,
        "citations_required": 1.0,
        "no_speculation": 0.90,
        "math_check": 0.85,
        "no_metaphor": 0.70,
    },
    domain_scope: "science,research,biology,physics",
    ...
}
```

**Profile: "Legal Conservative"**
```rust
WeightProfile {
    profile_id: "profile.legal_conservative",
    name: "Legal Conservative",
    weights: {
        "citations_required": 1.0,
        "no_speculation": 1.0,
        "no_metaphor": 0.95,
        "flag_ambiguity": 0.90,
    },
    domain_scope: "legal,compliance,contracts",
    ...
}
```

**Profile: "Fast Exploration"**
```rust
WeightProfile {
    profile_id: "profile.fast_exploration",
    name: "Fast Exploration",
    weights: {
        "speed_over_rigor": 0.90,
        "speculation_ok": 0.80,
        "draft_quality": 0.70,
    },
    domain_scope: "brainstorm,prototype,ideation",
    ...
}
```

---

### **B3: Applying Profiles**

**Users can apply profiles at two levels:**

**1) Persistent (SCV-level):**
```rust
APPLY_PROFILE_TO_SCV(
    scv_id: String,
    profile_id: String,
) -> Result<(), Error>
```

All future messages from this SCV use this profile by default.

**2) Ephemeral (turn-level override):**
```rust
APPLY_PROFILE_TO_TURN(
    conversation_id: String,
    turn_id: String,
    profile_id: String,
) -> Result<(), Error>
```

Just this one message uses this profile.

**Turn overrides are recorded as commits** so historical reconstruction is possible.

---

### **B4: Influence Trace = Auditability**

**Every SCV response emits an Influence Trace.**

**InfluenceTrace schema:**
```rust
struct InfluenceTrace {
    trace_id: String,
    response_commit_ref: String,
    timestamp: u64,
    
    // Active configuration
    active_profile_id: String,
    active_profile_version: String,
    
    // What influenced this response
    top_influences: Vec<Influence>,
    
    // Validation results
    gates_checked: Vec<GateCheck>,
    violations: Vec<Violation>,
    
    // Overrides
    turn_overrides: Vec<Override>,
    
    // Evidence
    citations: Vec<CommitRef>,
}

struct Influence {
    branch_id: String,
    weight: f32,
    why_triggered: String,      // short explanation
    impact_score: f32,          // how much it affected output
}

struct GateCheck {
    gate_id: String,
    gate_name: String,
    passed: bool,
    details: String,
}

struct Violation {
    gate_id: String,
    severity: Severity,
    description: String,
    suggestion: String,
}

enum Severity {
    Blocking,    // output invalid until fixed
    Warning,     // flag but allow
    Info,        // log only
}

struct Override {
    override_type: String,
    original_value: String,
    overridden_value: String,
    reason: String,
}
```

**UI display (at top of every response):**
```
┌─────────────────────────────────────────┐
│ 🔍 Influence Trace                      │
│                                         │
│ Profile: Scientific Domain v1.2         │
│                                         │
│ Top Influences:                         │
│   Scientific Rigor        0.95 ████████ │
│   Citations Required      1.00 █████████│
│   No Speculation          0.90 ████████ │
│   Math Check              0.85 ███████  │
│                                         │
│ Gates: ✅ All passed                    │
│                                         │
│ [View Full Trace ▼]                    │
└─────────────────────────────────────────┘
```

**Invariant:**
> **"What influenced this output?" is always visible and historically reconstructable.**

---

### **B5: Profile Changes Create Commits**

**Invariant C12.8:**
> **If a weight profile changes, it must create a commit.**

**Why:**
- Historical reconstruction of influence
- Audit trail of steering changes
- Prevents invisible drift
- Makes A/B testing possible (different profiles on same input)

**Example commit:**
```rust
CommitNode {
    commit_ref: "conv.user123.c52",
    content_type: ContentType::ProfileChange,
    parent_refs: ["conv.user123.c51"],
    content: ProfileChangeEvent {
        from_profile: "profile.scientific_domain",
        to_profile: "profile.legal_conservative",
        reason: "Switching to contract review task",
        applied_to: "scv.forecast_03",
        scope: ScopeType::Persistent,
    },
    ...
}
```

---

## 🔑 PART C: ROOT HUD MECHANICS

### **The fundamental shift:**

**Before:** Root AI is "the smartest agent"  
**After:** Root AI is Compiler + Router + Auditor (structural coordinator)

---

### **C1: Root HUD = Three Functions**

**Root HUD is NOT:**
- The smartest agent
- An oracle
- A decision-maker
- A supervisor

**Root HUD IS:**

**1) COMPILER**
- Turns human intent into structured Task Contracts
- Compiles conversation history into Training Packs
- Transforms profiles + gates into runtime validators

**2) ROUTER**
- Assigns tasks to SCVs based on scope + reliability
- Routes cross-branch communication
- Selects tools based on task type

**3) AUDITOR**
- Enforces gates (hard validators)
- Emits Influence Traces
- Preserves commit history
- Tracks reliability scores

**Mental model:**
> **Root HUD = Air traffic control + Audit log + Lens manager**

Not: "A chatbot that knows everything."

---

### **C2: How Root HUD Cooperates with Users**

**Three operational modes:**

**Mode 1: STEERING MODE**

User sets:
- Scope (domain, objective, forbidden actions)
- Lens profile (cognitive branches / constraints / rigor)
- Success criteria (what counts as done)

Root HUD:
- Compiles into Task Contract
- Assigns appropriate SCV(s)
- Sets up gates and tracing

**Mode 2: EXECUTION MODE**

SCVs do work. Root HUD:
- Routes subtasks to correct SCVs
- Watches for drift and violations
- Collects results
- Returns header-level summary
- Preserves drill-down proof links

**Mode 3: VERIFICATION MODE**

User zooms in when needed:
- See evidence, commit refs, invariants
- Request correction commit if needed
- Lock decisions for future work
- Update reliability scores

**Core loop:**
```
User → Set steering → Root compiles → SCV executes 
→ Root audits → User sees headers → Zoom if needed 
→ Accept/Patch/Reject → Reliability updated → Loop
```

---

### **C3: How Root HUD Learns**

**Critical distinction:**
> **Root HUD does NOT "learn" by silently changing internal weights. It learns by writing append-only artifacts that improve future behavior.**

**Learning = Versioned Memory + Scored Reliability**

Not: Hidden gradient descent  
But: Explicit outcome tracking

---

### **C4: What Root HUD Stores**

Root HUD stores **only** things that are:
- Stable (won't change hourly)
- Reusable
- Auditable

**Storage hierarchy:**

**Tier 1: Training Packs**
- Per commit cutoff
- Locked invariants
- Active decisions
- Open questions
- Glossary
- Do-not-do rules
- Citations

**Purpose:** Enable deterministic agent onboarding

**Tier 2: Weight Profiles / Logic Branches**
- Named profiles (versioned)
- Branch definitions (versioned)
- Per-SCV defaults
- Per-project defaults
- Turn overrides (as commits)

**Purpose:** Enable explicit steering

**Tier 3: Outcome Records**
- Task success/failure
- Patch rate (corrections needed)
- User acceptance (✅/⚠️/❌)
- Time to completion
- Violation frequency

**Purpose:** Reliability scoring

**Tier 4: Artifact Index**
- Not whole transcripts
- Just pointers:
  - "Decision X is in commit c47"
  - "Proof Y is in artifact A"
  - "Spec Z is version v2.3"

**Purpose:** Fast lookup, no duplication

---

### **C5: What Root HUD Discards**

**Root HUD aggressively discards/compresses:**

❌ **Verbose conversation filler**
- "Let me think about that..."
- "That's a great question..."
- "To answer your question..."

❌ **Repeated paraphrases**
- Saying the same thing 3 different ways
- Redundant clarifications

❌ **Unverified speculation**
- "Maybe X could work?" (unless explicitly tagged as exploration)
- Hypotheticals without commit anchor

❌ **Style-only chatter**
- Emoji storms
- Excessive formatting
- Politeness padding that doesn't change decisions

❌ **Transient context**
- Temporary variables
- Debugging traces
- "Just for this turn" notes that don't affect future

**What remains:**
- Header-level summary
- Proof links (commit refs)
- Actual decisions
- Locked invariants

**Compression ratio target:** 10:1 to 20:1

---

### **C6: What Root HUD Ranks (and How)**

**Root HUD weights influences based on:**
> **Reliability + Scope Fit (NOT popularity)**

**High weight signals:**

🟢 **Locked invariants** (physics laws) → Weight: 1.0  
🟢 **User-accepted decisions** (explicit ✅ locked) → Weight: 0.95  
🟢 **Successful task patterns** (low patch rate) → Weight: 0.85  
🟢 **Verified evidence artifacts** (cited, traced) → Weight: 0.80  
🟢 **SCVs with strong reliability history** (in same scope) → Weight: 0.75

**Low weight signals:**

🔴 **Outputs that required many patches** → Weight: 0.30  
🔴 **Scope violations** → Weight: 0.20  
🔴 **"Pretty but unauditable" responses** → Weight: 0.15  
🔴 **Ungrounded inferences** → Weight: 0.10  
🔴 **Anything not anchored to commits** → Weight: 0.05

**Reliability scoring algorithm:**
```rust
fn calculate_reliability(scv_id: &str, scope: &str) -> f32 {
    let outcomes = get_outcomes(scv_id, scope);
    
    let acceptance_rate = outcomes.accepted / outcomes.total;
    let patch_rate = outcomes.patched / outcomes.total;
    let violation_rate = outcomes.violations / outcomes.total;
    
    let base_score = acceptance_rate - (patch_rate * 0.5) - (violation_rate * 0.8);
    
    // Time decay (recent outcomes weighted higher)
    let weighted_score = apply_time_decay(base_score, outcomes);
    
    // Scope bonus (specialist in correct domain)
    let scope_bonus = if in_scope(scv_id, scope) { 0.1 } else { -0.2 };
    
    (weighted_score + scope_bonus).clamp(0.0, 1.0)
}
```

**This is how the system gets better without becoming opaque.**

---

### **C7: Profiles + Gates + Traces > Raw Weights**

**The evolution:**

**V1 (Too Complex):** Raw weight sliders everywhere  
- 50 branches, 50 sliders
- Cognitive overload
- Unclear what combination means

**V2 (Better):** Named profiles + Hard gates + Influence traces  
- **Profiles** = pre-built bundles (domain + constraints + preferences)
- **Gates** = hard validators (binary pass/fail)
- **Traces** = visibility (what profile, what gates, what evidence)
- **Weights** = hidden behind the scenes for routing/optimization

**Why this is better:**

**Profiles are easier to reason about:**
```
"Use Scientific Domain profile"
  vs
"Set 12 different sliders to specific values"
```

**Gates are clearer than soft weights:**
```
"Citations Required: PASS/FAIL"
  vs
"Citations weight: 0.73"
```

**Traces provide visibility:**
```
"Scientific Rigor influenced this (0.95)"
  vs
"Model output" (black box)
```

**Weights still exist** behind the scenes for:
- Routing (which SCV gets task)
- Prioritization (which tool first)
- Compression (what to surface in HUD)
- Optimization (speed vs rigor tradeoffs)

But users interact with **profiles + gates + traces**, not raw weights.

---

### **C8: Gate System (Hard Validators)**

**A Gate is:**
> A hard validator that enforces physics laws and prevents drift.

**Gate schema:**
```rust
struct Gate {
    gate_id: String,
    name: String,
    description: String,
    version: String,
    
    // Validation
    check_fn: ValidationFn,
    violation_action: ViolationAction,
    
    // Context
    applies_to: Vec<String>,     // "all" | "scientific" | "legal" | etc
    required_for: Vec<String>,   // profile IDs that must use this
    
    // Audit
    violation_count: u64,
    last_violation: Option<u64>,
}

enum ViolationAction {
    Block,            // Output invalid, cannot proceed
    BlockWithPatch,   // Output blocked, suggest correction
    Warn,             // Flag but allow
    Log,              // Record only
}

type ValidationFn = fn(&str) -> Result<(), ViolationDetail>;
```

**Example gates:**

**Gate: "No New Primitives"**
```rust
Gate {
    gate_id: "gate.no_new_primitives",
    name: "No New Primitives",
    description: "Output must not invent new physics concepts without explicit lock",
    check_fn: |output| {
        let primitives = extract_primitives(output);
        let known = load_locked_primitives();
        let new = primitives.difference(&known);
        if !new.is_empty() {
            Err(ViolationDetail {
                found: new.to_vec(),
                suggestion: "Use existing primitives or propose architecture commit",
            })
        } else {
            Ok(())
        }
    },
    violation_action: ViolationAction::Block,
    applies_to: vec!["all"],
    ...
}
```

**Gate: "Citations Required"**
```rust
Gate {
    gate_id: "gate.citations_required",
    name: "Citations Required",
    check_fn: |output| {
        let claims = extract_claims(output);
        for claim in claims {
            if !has_citation(claim) {
                return Err(ViolationDetail {
                    found: vec![claim],
                    suggestion: "Add commit ref or evidence link",
                });
            }
        }
        Ok(())
    },
    violation_action: ViolationAction::BlockWithPatch,
    applies_to: vec!["scientific", "legal"],
    ...
}
```

**Gate: "No Semantic Zoom"**
```rust
Gate {
    gate_id: "gate.no_semantic_zoom",
    name: "No Semantic Zoom",
    description: "Zoom must be lens change, not reinterpretation",
    check_fn: |output| {
        if detects_semantic_change(output) {
            Err(ViolationDetail {
                found: vec!["Semantic change detected in zoom operation"],
                suggestion: "Zoom reveals detail, never changes meaning",
            })
        } else {
            Ok(())
        }
    },
    violation_action: ViolationAction::Block,
    applies_to: vec!["all"],
    ...
}
```

**Gates run automatically on every SCV output.**

If violated:
1. Output flagged **INVALID**
2. Violation logged to Influence Trace
3. Correction commit required
4. Reliability score decremented

---

### **C9: Violation Detection + Correction Flow**

**Automatic violation detection:**

```rust
fn validate_output(
    output: &str,
    active_profile: &WeightProfile,
    gates: &[Gate],
) -> ValidationResult {
    let mut violations = Vec::new();
    
    for gate in gates {
        if gate.applies_to_profile(active_profile) {
            match (gate.check_fn)(output) {
                Ok(()) => continue,
                Err(detail) => violations.push(Violation {
                    gate_id: gate.gate_id.clone(),
                    severity: gate.violation_action.to_severity(),
                    description: detail.found,
                    suggestion: detail.suggestion,
                }),
            }
        }
    }
    
    if violations.is_empty() {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid { violations }
    }
}
```

**Correction flow:**

```
1. SCV produces output
2. Gates run automatically
3. Violation detected
4. Output marked INVALID
5. User sees violation details
6. User requests correction
7. SCV produces patched version
8. Gates run again
9. If pass → Accept
10. Outcome recorded (reliability -0.1)
```

**Example UI:**
```
┌─────────────────────────────────────────┐
│ ⚠️  SCV-Research-07 Response INVALID    │
│                                         │
│ Gate Violation:                         │
│   Citations Required (BLOCKING)         │
│                                         │
│ Details:                                │
│   Claim: "Fusion reactors are safe"    │
│   Missing: Evidence link or commit ref  │
│                                         │
│ Suggestion:                             │
│   Add citation to source material       │
│                                         │
│ [Request Correction] [Override (Admin)]│
└─────────────────────────────────────────┘
```

---

## 🔒 LOCKED INVARIANTS (architecture@c12)

### **PART A: CONVERSATION AS FILAMENT (4 invariants)**

**Invariant C12.1: Conversations are append-only commit history**
> Every AI conversation is a filament. Commits never mutated. Branches spawn descendants. Parent remains intact.

**Invariant C12.2: Agent onboarding spawns from commit**
> "Onboard" action creates SCV trained up to chosen commit via Training Pack. No history rewriting.

**Invariant C12.3: Training Packs are deterministic artifacts**
> Training = compile (summarize → validate → compress → seal). Output: sealed Training Pack (invariants, decisions, questions, glossary, forbidden drifts, tasks, citations). Mechanical, not creative.

**Invariant C12.4: Humans operate at header level**
> Every SCV output has Layer 0 (headers, one screen) + Layer 1 (proof, expandable). Humans zoom only when needed. System remains auditable.

---

### **PART B: LOGIC BRANCHES + VISIBLE STEERING (4 invariants)**

**Invariant C12.5: Root Logic has named branches**
> Logic Branches are first-class objects (versioned, commit-addressable). Examples: "Scientific Rigor", "No Metaphor", "Citations Required". Not hidden model internals.

**Invariant C12.6: Weight Profiles make steering explicit**
> Users select branches explicitly via Profiles. No keyword stuffing. Profile = bundle of branch weights + constraints + validation modes.

**Invariant C12.7: Influence Traces provide auditability**
> Every response emits Influence Trace showing: active profile, top branches, weights, gate results, violations. "What influenced this?" is always visible.

**Invariant C12.8: Profile changes create commits**
> Weight Profile changes are commits. Historical reconstruction of "what influenced output" is always possible. No invisible steering.

---

### **PART C: ROOT HUD MECHANICS (5 invariants)**

**Invariant C12.9: Root HUD is Compiler + Router + Auditor**
> Not "smartest agent." Structural coordinator that: compiles intent → routes tasks → enforces gates → preserves audit trail. Not oracle, not supervisor.

**Invariant C12.10: Learning is append-only artifacts**
> Root learns by writing versioned artifacts (Training Packs, Profiles, Outcome Records), NOT hidden training. Everything auditable. No silent weight changes.

**Invariant C12.11: Storage hierarchy (store/discard/rank)**
> **Store:** Training Packs, Profiles, Outcome Records (reliability scores), Artifact Index (commit pointers).  
> **Discard:** Verbose filler, repeated paraphrases, unverified speculation, style-only chatter.  
> **Rank by:** Reliability + scope fit (locked invariants, user-accepted decisions, low patch rate), NOT popularity.

**Invariant C12.12: Profiles + Gates + Traces > raw weights**
> **Profiles** = named bundles (domain + constraints + preferences + validation mode).  
> **Gates** = hard validators (block/flag violations).  
> **Traces** = visibility (what profile, what gates, what evidence).  
> Weights used behind scenes for routing/optimization only. Users interact with profiles, not raw sliders.

**Invariant C12.13: Violation detection enforces physics**
> Gates run automatically on every output. If violated → flagged INVALID → requires correction commit. Examples: "No new primitives", "Citations required", "No semantic zoom". Reliability score decremented on violations.

---

## 🎯 WHAT THIS UNLOCKS

### **Immediate (with c12):**
- ✅ Conversation = filament (spawn agents from commit, no copy/paste hell)
- ✅ Explicit steering (select branches, see influences, no keyword stuffing)
- ✅ Header/proof zoom (HUD-level operation, audit when needed)
- ✅ Training Packs (deterministic compilation, reproducible agents)
- ✅ Gates enforce physics (automatic violation detection)
- ✅ Learning via artifacts (auditable, not hidden)

### **With PR #9 (Identity Filaments):**
- Agents have identity filaments (history of reliability)
- Root checks agent identity before assigning tasks
- Access to tasks derived from agent's outcome history

### **With PR #12 (Proximity Channels):**
- Agents "visit" Root HUD building (conceptual space)
- Proximity channel activates during consultation
- Steering scoped to location + profile

### **With Frontend:**
- Visual Branch Palette (sliders + presets)
- Onboard button on commit timeline
- Header/proof toggle
- Influence Trace overlay
- Gate violation warnings

---

## 🚀 IMPLEMENTATION CHECKLIST (Future PRs)

### **PR #17: Conversation Filament + Commit Nodes**
- [ ] CommitNode schema + storage
- [ ] Commit timeline UI
- [ ] Branch/fork mechanics (descendant spawning)
- [ ] Parent-ref linkage

### **PR #18: Training Pack Compilation**
- [ ] TrainingPack schema
- [ ] Compilation pipeline (extract → cluster → validate → compress → seal)
- [ ] Storage + versioning
- [ ] Onboard-from-commit action

### **PR #19: Logic Branches + Weight Profiles**
- [ ] LogicBranch registry (versioned)
- [ ] WeightProfile objects
- [ ] Branch Palette UI (sliders + presets)
- [ ] Apply-to-SCV / apply-to-turn controls
- [ ] Profile save/load

### **PR #20: Influence Tracing**
- [ ] InfluenceTrace schema
- [ ] Trace emission on every response
- [ ] UI overlay (top influences)
- [ ] Zoom to full trace
- [ ] Historical reconstruction

### **PR #21: Gate System + Violation Detection**
- [ ] Gate registry
- [ ] ValidationFn framework
- [ ] Automatic checking on output
- [ ] Violation flagging UI
- [ ] Correction commit flow

### **PR #22: Outcome Tracking + Reliability Scoring**
- [ ] OutcomeRecord schema
- [ ] Acceptance/patch/reject tracking
- [ ] Reliability score calculation
- [ ] SCV ranking by reliability
- [ ] Profile effectiveness tracking

### **PR #23: Root HUD UI (Compiler/Router/Auditor)**
- [ ] Steering Mode panel
- [ ] Execution Mode summary
- [ ] Verification Mode zoom
- [ ] Task routing visualization
- [ ] Audit trail viewer

### **PR #24: Header/Proof Zoom System**
- [ ] Layer 0 (headers) generator
- [ ] Layer 1 (proof) expansion
- [ ] Collapse/expand controls
- [ ] Commit-ref linking
- [ ] Invariant highlighting

**Estimated total time:** ~100 hours (8 PRs × ~12-15 hours each)  
**Priority:** High (this is the cognitive substrate for all AI work)

---

## 📊 EXAMPLE WORKFLOWS

### **Workflow 1: Onboard Specialist SCV from Conversation**

**Context:** User has 47 commits of conversation about biology research.

**Steps:**
1. User scrolls to commit c47 (stable point)
2. Clicks "Onboard Specialist from c47"
3. Selects:
   - Role: Specialist
   - Scope: "biology_research"
   - Profile: "Scientific Domain"
4. System compiles Training Pack:
   - Extracts commits c0 → c47
   - Identifies locked invariants (12 found)
   - Identifies active decisions (8 found)
   - Identifies open questions (3 found)
   - Compresses 50K tokens → 5K tokens
   - Seals Training Pack (hash + sign)
5. System spawns SCV-Biology-01:
   - Bound to Training Pack
   - Bound to Scientific Domain profile
   - Reliability score: 0.0 (new)
6. User sends first task: "Summarize recent mRNA research"
7. SCV produces response (header-level)
8. Influence Trace shows:
   - Scientific Rigor: 0.95
   - Citations Required: 1.0
   - No Speculation: 0.90
9. User accepts (✅)
10. Outcome recorded (reliability → 0.1)

**Result:** New specialist agent trained on conversation history, with explicit steering profile, auditable outputs.

---

### **Workflow 2: Adjust Steering Mid-Conversation**

**Context:** User working with SCV-Forecast-03 on financial projections. Next turn needs legal review lens.

**Steps:**
1. User composes message: "Check if this forecast violates SEC rules"
2. User opens Branch Palette
3. User selects profile: "Legal Conservative"
4. User clicks "Apply to this message only"
5. System creates turn override commit:
   - From: "Financial Forecast" profile
   - To: "Legal Conservative" profile
   - Scope: Ephemeral (one turn)
6. User sends message
7. SCV-Forecast-03 processes with new profile
8. Response shows header:
   - "✅ No obvious SEC violations"
   - "⚠️ Ambiguous: forward-looking statement disclosure"
9. Influence Trace shows:
   - Legal Conservative: 0.90
   - Citations Required: 1.0
   - Flag Ambiguity: 0.95
10. User zooms into proof layer
11. See exact statute citations + ambiguity details
12. User accepts (✅)
13. Next turn returns to "Financial Forecast" profile

**Result:** Steering changed for one turn, explicitly recorded, influence visible, no keyword stuffing needed.

---

### **Workflow 3: Detect and Correct Gate Violation**

**Context:** SCV-Research-07 produces response that violates "Citations Required" gate.

**Steps:**
1. SCV produces output: "Fusion reactors are safe and scalable"
2. Gate system runs automatically
3. Gate "Citations Required" checks output
4. Violation detected:
   - Claim: "Fusion reactors are safe"
   - Missing: Citation or evidence link
5. Output flagged INVALID
6. User sees violation warning:
   ```
   ⚠️ Gate Violation: Citations Required (BLOCKING)
   Claim without evidence: "Fusion reactors are safe"
   Suggestion: Add commit ref or source link
   ```
7. User clicks "Request Correction"
8. SCV produces patched version:
   - "Fusion reactors show promise for safety (c23, ITER Report 2024)"
9. Gate runs again
10. Pass ✅
11. User accepts
12. Outcome recorded:
    - SCV-Research-07 reliability: -0.1 (violation penalty)
    - Gate "Citations Required" triggered count: +1

**Result:** Physics violation caught automatically, correction required, reliability tracked.

---

### **Workflow 4: Audit Historical Decision**

**Context:** 6 months later, user needs to understand why a decision was made.

**Steps:**
1. User opens conversation timeline
2. Navigates to commit c73 (decision point)
3. Clicks "View Influence Trace"
4. System reconstructs:
   - Active profile: "Systems Engineering v1.3"
   - Top influences:
     - Deterministic Only: 0.95
     - No Magic: 0.90
     - Testable: 0.85
   - Gates checked:
     - No New Primitives: PASS
     - Citations Required: PASS
   - Citations: c64, c67, c70
5. User zooms into proof layer
6. See exact reasoning chains + commit refs
7. User clicks citation c64
8. Opens full context of that earlier decision
9. Understands causal chain: c64 → c67 → c70 → c73

**Result:** Historical decision fully auditable, influence chain traceable, no black box.

---

## 🔗 INTEGRATION WITH PRIOR ARCHITECTURE

### **Extends architecture@c11 (Root AI Coherence Layer):**
- c11 defined Root AI **role** (coherence layer, not supervisor)
- c12 defines Root AI **mechanics** (how it operates, stores, learns)
- c11: "What" (Root + branches)
- c12: "How" (compilation, routing, auditing)

### **Extends architecture@c10 (Users as Filament Trees):**
- c10: Users ARE filament trees
- c12: **Conversations ARE filaments** (AI interactions join the tree)
- User's identity = intersection of all filaments (including AI conversations)

### **Implements architecture@c9 (StarCraft HUD):**
- c9: HUD is personal, globe is shared
- c12: Root HUD is cognitive control plane (RTS-style operation)
- Header/proof zoom = HUD-level vs tactical zoom

### **Extends architecture@c7 (Coordination Gauges):**
- c7: Multiple orthogonal gauges (not single currency)
- c12: Multiple logic branches (not single "intelligence" score)
- Reliability score is per-scope, not global

### **Enforces architecture@c2 (SSE Truth Stream):**
- c2: Deterministic replay, no phantom events
- c12: Training Packs + Influence Traces = deterministic AI behavior
- Same commit cutoff + profile → same Training Pack → same agent behavior

### **Enforces architecture@c0 (Layer Separation):**
- c0: Layer 2 outputs data, Layer 3 renders
- c12: Root HUD outputs Task Contracts, SCVs execute, Layer 3 renders UI
- No rendering logic in Root HUD

---

## 🚫 ANTI-PATTERNS (WHAT NOT TO DO)

### **❌ Don't: Treat conversations as disposable**
- Conversations are filaments, not chat logs
- History must be preserved, not discarded
- Copy/paste hell is a physics violation

### **❌ Don't: Let steering be invisible**
- No hidden keyword magic
- Branches must be explicit and selectable
- Influence must be traceable

### **❌ Don't: Let AI "learn" silently**
- No hidden gradient descent on production agents
- Learning = write artifacts, not mutate weights
- Everything must be auditable

### **❌ Don't: Treat Root HUD as "smartest agent"**
- Root is structural, not intellectual
- Root compiles, routes, audits
- Root doesn't replace specialist knowledge

### **❌ Don't: Make humans read everything**
- Header-level operation is the default
- Zoom into proof only when needed
- RTS HUD level, not paragraph-by-paragraph

### **❌ Don't: Use raw weight sliders everywhere**
- Profiles + Gates + Traces > raw weights
- Users shouldn't tune 50 sliders
- Presets + named bundles + hard validators

### **❌ Don't: Let violations pass silently**
- Gates must run automatically
- Violations must block or flag
- Reliability must be tracked

---

## 📝 TESTING STRATEGY

### **Test 1: Deterministic Training Pack Compilation**
**Given:** Conversation commits c0 → c47  
**When:** Compile Training Pack twice  
**Then:** Identical output (same hash, same content)

### **Test 2: Agent Onboarding Reproducibility**
**Given:** Training Pack + Weight Profile  
**When:** Spawn agent twice from same artifacts  
**Then:** Identical initial behavior on same input

### **Test 3: Profile Change Creates Commit**
**Given:** SCV with Profile A  
**When:** Change to Profile B  
**Then:** Commit created with from/to/reason

### **Test 4: Influence Trace Completeness**
**Given:** SCV response with Profile + Gates  
**When:** Emit Influence Trace  
**Then:** Contains profile, branches, weights, gate results, citations

### **Test 5: Gate Violation Blocks Output**
**Given:** SCV output without required citations  
**When:** Gate "Citations Required" runs  
**Then:** Output flagged INVALID, correction required

### **Test 6: Reliability Score Updates**
**Given:** SCV produces output  
**When:** User accepts/patches/rejects  
**Then:** Reliability score updated accordingly

### **Test 7: Header/Proof Zoom Preserves Meaning**
**Given:** Full response (Layer 1)  
**When:** Collapse to headers (Layer 0)  
**Then:** No semantic information lost (just compressed)

### **Test 8: Historical Influence Reconstruction**
**Given:** Commit from 6 months ago  
**When:** Query "What influenced this?"  
**Then:** Reconstruct active profile, branches, weights, gates from commits

---

## 🎯 SUCCESS CRITERIA

**This architecture is successful if:**

1. ✅ Conversations never hit "copy/paste hell" (spawn new agents from commits instead)
2. ✅ Users stop keyword-stuffing prompts (select profiles explicitly)
3. ✅ "What influenced this?" is always answerable (Influence Traces)
4. ✅ Humans operate at header-level 90% of the time (zoom only when needed)
5. ✅ AI behavior is reproducible (same Training Pack + Profile → same output)
6. ✅ Gates catch physics violations automatically (no manual checking)
7. ✅ Reliability improves over time (outcome tracking works)
8. ✅ Historical decisions are auditable (commit-based reconstruction)

---

## 🔮 FUTURE EXTENSIONS (Post-c12)

### **Possible architecture@c13: Cross-Conversation Filaments**
- Conversations can reference other conversations
- Shared Training Packs across projects
- "Import learnings from conversation X"

### **Possible architecture@c14: Federated Root HUDs**
- Multiple Root HUDs for different orgs
- Cross-organization profile sharing
- Privacy-preserving collaboration

### **Possible architecture@c15: Real-Time Drift Detection**
- Continuous monitoring during long conversations
- Automatic re-onboarding when drift detected
- "Your SCV needs retraining" notifications

### **Possible architecture@c16: Outcome-Based Profile Optimization**
- Automatic profile tuning based on outcome history
- "This profile works better for task type X"
- A/B testing framework for profiles

---

## 📋 SUMMARY

**The breakthrough:**
> Conversations ARE filaments. AI training IS deterministic compilation. Steering IS explicit branch selection. Learning IS append-only artifact creation.

**The implementation:**
> CommitNodes + Training Packs + Logic Branches + Weight Profiles + Influence Traces + Gates + Root HUD (Compiler/Router/Auditor).

**The result:**
> No context loss. No keyword stuffing. No hidden influences. No opaque learning. No copy/paste hell. Coherent AI coordination by construction.

---

**Status:** 🔒 LOCKED  
**Invariants added:** 13 (total: 83)  
**Dependencies:** c11 (Root AI Coherence), c10 (Identity), c9 (StarCraft HUD), c2 (SSE Truth Stream), c0 (Layer Separation)  
**Next:** Implement PR #17 (Conversation Filament) or build detailed module specs

---

**THIS IS HOW RELAY'S FILAMENT PHYSICS APPLIES TO THE AI CONVERSATION SUBSTRATE ITSELF.**

**END OF architecture@c12**

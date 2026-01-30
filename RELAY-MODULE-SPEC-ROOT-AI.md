# RELAY MODULE SPEC: ROOT AI COGNITIVE SUBSTRATE

**Version:** 1.0  
**Date:** 2026-01-29  
**Status:** READY FOR IMPLEMENTATION  
**Architecture:** Based on architecture@c12

---

## 🎯 PURPOSE

**This spec provides concrete, actionable implementation details for:**
- Conversation as Filament (commit nodes, onboarding, Training Packs)
- Logic Branches + Weight Profiles (visible steering, influence tracing)
- Root HUD (compiler/router/auditor mechanics)
- Gates (violation detection, correction flow)

**Target audience:** Claude (or dev team) implementing Root AI system

**Non-negotiable constraints:**
- Everything append-only (no history mutation)
- Everything auditable (commit refs everywhere)
- Everything deterministic (same inputs → same outputs)
- Everything explicit (no hidden steering)

---

## 📦 MODULE 1: CONVERSATION AS FILAMENT

### **Deliverable 1.1: CommitNode Data Model**

**Schema (TypeScript/Rust-like):**
```typescript
interface CommitNode {
  commit_ref: string;              // "conv.user123.c47"
  timestamp: number;               // Unix ms
  author: string;                  // "user.alice" | "scv.forecast_03"
  parent_refs: string[];           // ["conv.user123.c46"]
  content_type: ContentType;
  content_hash: string;            // SHA-256 of content
  content: string | object;        // The actual content (text or structured)
  
  // Metadata
  training_pack_ref?: string;      // If this is onboarding point
  weight_profile_ref?: string;     // Active profile for this turn
  influence_trace_ref?: string;    // Proof artifact
  outcome_status?: OutcomeStatus;  // User feedback
  
  // Audit
  created_at: number;
  version: string;                 // Schema version
}

enum ContentType {
  UserPrompt = "user_prompt",
  AgentResponse = "agent_response",
  TrainingPack = "training_pack",
  ProfileChange = "profile_change",
  GateViolation = "gate_violation",
  CorrectionCommit = "correction_commit",
}

enum OutcomeStatus {
  Accepted = "accepted",      // User confirmed ✅
  Patched = "patched",        // Required correction
  Rejected = "rejected",      // Blocked by gates
  Pending = "pending",        // Not yet reviewed
}
```

**Storage:** JSONL file per conversation
- Location: `var/relay_ai/conversations/<conversation_id>.jsonl`
- Each line = one CommitNode (serialized JSON)
- Append-only (no edits)

**API endpoints:**
```
GET  /api/conversations/:id/commits
GET  /api/conversations/:id/commits/:commit_ref
POST /api/conversations/:id/commits
```

---

### **Deliverable 1.2: Training Pack Compilation**

**Schema:**
```typescript
interface TrainingPack {
  pack_id: string;
  scope_id: string;
  cutoff_commit: string;          // "conv.user123.c47"
  created_at: number;
  version: string;
  
  // Core content
  locked_invariants: Invariant[];
  active_decisions: Decision[];
  open_questions: Question[];
  glossary: GlossaryEntry[];
  do_not_do: ForbiddenPattern[];
  current_tasks: TaskSummary[];
  
  // Proof
  citations: CommitRef[];
  reliability_score: number;      // 0.0-1.0
  
  // Compression metadata
  original_token_count: number;
  compressed_token_count: number;
  compression_ratio: number;
  compression_method: string;     // "semantic_clustering_v1"
}

interface Invariant {
  invariant_id: string;
  statement: string;
  commit_ref: string;             // Where it was locked
  violation_cost: string;         // "physics_breaks" | "drift" | "invalid"
}

interface Decision {
  decision_id: string;
  question: string;
  chosen: string;
  rejected_alternatives: string[];
  rationale: string;
  commit_ref: string;
}

interface ForbiddenPattern {
  pattern_id: string;
  description: string;
  why_forbidden: string;
  example: string;
  commit_ref: string;
}

interface GlossaryEntry {
  term: string;
  definition: string;
  aliases: string[];
  commit_ref: string;
}

interface Question {
  question_id: string;
  question: string;
  context: string;
  blocking: boolean;              // Must be answered before proceeding?
  commit_ref: string;
}

interface TaskSummary {
  task_id: string;
  description: string;
  status: "pending" | "in_progress" | "done";
  commit_ref: string;
}
```

**Compilation pipeline:**
```typescript
async function compileTrainingPack(
  conversation_id: string,
  cutoff_commit: string,
  scope_id: string
): Promise<TrainingPack> {
  // 1. EXTRACT: Load all commits c0 → cutoff
  const commits = await loadCommits(conversation_id, cutoff_commit);
  
  // 2. CLUSTER: Group by topic
  const clusters = await clusterByTopic(commits);
  
  // 3. EXTRACT INVARIANTS: Find locked physics laws
  const invariants = await extractInvariants(clusters);
  
  // 4. EXTRACT DECISIONS: Find choices made
  const decisions = await extractDecisions(clusters);
  
  // 5. EXTRACT QUESTIONS: Find explicit unknowns
  const questions = await extractOpenQuestions(clusters);
  
  // 6. BUILD GLOSSARY: Terms + definitions
  const glossary = await buildGlossary(clusters);
  
  // 7. IDENTIFY FORBIDDEN PATTERNS: Drift prevention
  const forbidden = await identifyForbiddenPatterns(clusters);
  
  // 8. SUMMARIZE TASKS: Next steps
  const tasks = await summarizeTasks(clusters);
  
  // 9. COMPRESS: Remove filler
  const compressed = await compress(clusters);
  
  // 10. VALIDATE: Check for contradictions
  await validate(invariants, decisions);
  
  // 11. SEAL: Hash + sign
  const pack = createPack({
    locked_invariants: invariants,
    active_decisions: decisions,
    open_questions: questions,
    glossary,
    do_not_do: forbidden,
    current_tasks: tasks,
    citations: extractCitations(commits),
    original_token_count: countTokens(commits),
    compressed_token_count: countTokens(compressed),
  });
  
  return seal(pack);
}
```

**API endpoints:**
```
POST /api/training-packs/compile
  Body: { conversation_id, cutoff_commit, scope_id }
  Returns: TrainingPack

GET  /api/training-packs/:pack_id
```

---

### **Deliverable 1.3: Agent Onboarding Action**

**Function:**
```typescript
async function onboardAgentFromCommit(
  commit_ref: string,
  role_type: AgentRole,
  scope: string,
  weight_profile_id: string
): Promise<AgentInstance> {
  // 1. Compile Training Pack
  const pack = await compileTrainingPack(
    extractConversationId(commit_ref),
    commit_ref,
    scope
  );
  
  // 2. Load Weight Profile
  const profile = await loadWeightProfile(weight_profile_id);
  
  // 3. Create Agent Instance
  const agent = {
    agent_id: generateAgentId(),
    role_type,
    scope,
    training_pack_ref: pack.pack_id,
    weight_profile_ref: profile.profile_id,
    created_at: Date.now(),
    reliability_score: 0.0,      // New agent starts at 0
    outcome_history: [],
  };
  
  // 4. Store
  await storeAgentInstance(agent);
  
  // 5. Emit event
  await emitEvent({
    type: "agent_onboarded",
    agent_id: agent.agent_id,
    from_commit: commit_ref,
    training_pack: pack.pack_id,
    profile: profile.profile_id,
  });
  
  return agent;
}

enum AgentRole {
  Root = "root",
  BranchRoot = "branch_root",
  Specialist = "specialist",
}

interface AgentInstance {
  agent_id: string;
  role_type: AgentRole;
  scope: string;
  training_pack_ref: string;
  weight_profile_ref: string;
  created_at: number;
  reliability_score: number;
  outcome_history: OutcomeRecord[];
}
```

**UI trigger:**
```
Button: "Onboard Agent from c47"
  ↓
Modal:
  - Role: [Root | BranchRoot | Specialist]
  - Scope: [text input]
  - Profile: [dropdown of profiles]
  ↓
POST /api/agents/onboard
  Body: { commit_ref, role_type, scope, profile_id }
  ↓
Returns: agent_id
```

---

### **Deliverable 1.4: Header/Proof Zoom System**

**Output structure:**
```typescript
interface LayeredOutput {
  response_id: string;
  agent_id: string;
  commit_ref: string;
  
  // Layer 0: Headers (default view)
  headers: {
    summary: string[];           // Bullet points
    actions: string[];           // What will be done
    citations: string[];         // Commit refs
    influence_summary: string;   // "Scientific Rigor 0.95"
  };
  
  // Layer 1: Proof (expandable)
  proof: {
    full_text: string;           // Complete response
    reasoning_chains: ReasoningChain[];
    evidence_links: EvidenceLink[];
    influence_trace: InfluenceTrace;
  };
  
  // Metadata
  created_at: number;
  collapsed: boolean;            // UI state
}

interface ReasoningChain {
  step: number;
  description: string;
  inputs: string[];
  outputs: string[];
  commit_ref: string;
}

interface EvidenceLink {
  claim: string;
  evidence_type: "commit" | "artifact" | "external";
  evidence_ref: string;
}
```

**UI rendering:**
```html
<!-- Layer 0: Headers (default) -->
<div class="layered-output collapsed">
  <div class="header-section">
    <h4>✅ Updated 3-month projection</h4>
    <ul>
      <li>Applied seasonality adjustment</li>
      <li>Confidence: 78% (low data)</li>
    </ul>
    <div class="citations">
      Citations: <a href="#c41">c41</a>, <a href="#c42">c42</a>
    </div>
    <div class="influence">
      Influence: Scientific 0.85, Citations 1.0
    </div>
    <button onclick="expandProof()">Expand Proof ▼</button>
  </div>
</div>

<!-- Layer 1: Proof (expandable) -->
<div class="proof-section" style="display:none;">
  <h3>Full Response</h3>
  <div class="full-text">
    <!-- Complete paragraphs -->
  </div>
  <h4>Reasoning Chains</h4>
  <div class="reasoning-chains">
    <!-- Detailed reasoning -->
  </div>
  <h4>Influence Trace</h4>
  <div class="influence-trace-full">
    <!-- Complete trace details -->
  </div>
  <button onclick="collapseProof()">Collapse ▲</button>
</div>
```

**Invariant enforcement:**
> **Zoom = lens change, NOT semantic change. Collapsing to headers never loses information, just compresses it.**

---

## 📦 MODULE 2: LOGIC BRANCHES + WEIGHT PROFILES

### **Deliverable 2.1: LogicBranch Data Model**

**Schema:**
```typescript
interface LogicBranch {
  branch_id: string;              // "scientific_rigor"
  name: string;                   // "Scientific Rigor"
  description: string;
  version: string;                // "v1.2"
  created_at: number;
  
  // Behavior modifiers
  constraints: Constraint[];
  biases: Bias[];
  validation_mode: ValidationMode[];
  
  // Metadata
  domain: string;                 // "science" | "legal" | "engineering"
  strength: BranchStrength;
  
  // Audit
  usage_count: number;
  last_used: number;
}

interface Constraint {
  constraint_id: string;
  rule: string;                   // "Citations required for claims"
  violation_action: ViolationAction;
  examples: string[];
}

interface Bias {
  bias_id: string;
  preference: string;             // "Prefer deterministic algorithms"
  strength: number;               // 0.0-1.0
}

enum ValidationMode {
  CitationsRequired = "citations_required",
  MathCheck = "math_check",
  NoSpeculation = "no_speculation",
  NoMetaphor = "no_metaphor",
  NoSemanticZoom = "no_semantic_zoom",
  DeterministicOnly = "deterministic_only",
}

enum BranchStrength {
  Hard = "hard",        // Must be followed (gate)
  Soft = "soft",        // Prefer but not required
  Advisory = "advisory", // Informational only
}

enum ViolationAction {
  Block = "block",
  BlockWithPatch = "block_with_patch",
  Warn = "warn",
  Log = "log",
}
```

**Example branches:**
```typescript
const branches: LogicBranch[] = [
  {
    branch_id: "scientific_rigor",
    name: "Scientific Rigor",
    description: "Evidence-based reasoning with citations",
    version: "v1.2",
    constraints: [
      {
        constraint_id: "sc_citations",
        rule: "All claims must have citations",
        violation_action: ViolationAction.Block,
        examples: ["Claim: X → Citation: [source]"],
      },
      {
        constraint_id: "sc_no_speculation",
        rule: "No speculation without explicit marking",
        violation_action: ViolationAction.Warn,
        examples: [],
      },
    ],
    biases: [
      {
        bias_id: "sc_math_check",
        preference: "Verify mathematical claims",
        strength: 0.9,
      },
    ],
    validation_mode: [
      ValidationMode.CitationsRequired,
      ValidationMode.MathCheck,
      ValidationMode.NoSpeculation,
    ],
    domain: "science",
    strength: BranchStrength.Hard,
    usage_count: 0,
    last_used: 0,
  },
  {
    branch_id: "no_metaphor",
    name: "No Metaphor",
    description: "Literal language only, no analogies",
    version: "v2.0",
    constraints: [
      {
        constraint_id: "nm_literal",
        rule: "Use literal language, not metaphorical",
        violation_action: ViolationAction.Warn,
        examples: [
          "Bad: 'filaments are like pipes'",
          "Good: 'filaments are append-only logs'",
        ],
      },
    ],
    biases: [],
    validation_mode: [ValidationMode.NoMetaphor],
    domain: "all",
    strength: BranchStrength.Soft,
    usage_count: 0,
    last_used: 0,
  },
  // ... more branches
];
```

**Storage:** `var/relay_ai/logic_branches/<branch_id>@<version>.json`

**API endpoints:**
```
GET  /api/logic-branches
GET  /api/logic-branches/:branch_id
POST /api/logic-branches       (admin only)
```

---

### **Deliverable 2.2: WeightProfile Data Model**

**Schema:**
```typescript
interface WeightProfile {
  profile_id: string;
  name: string;
  description: string;
  version: string;
  created_at: number;
  created_from_commit?: string;
  
  // The weights
  weights: Record<string, number>;  // branch_id -> weight (0.0-1.0)
  normalization_rule: NormalizationRule;
  
  // Metadata
  domain_scope: string;
  recommended_for: string[];
  do_not_use_for: string[];
  
  // Audit
  usage_count: number;
  reliability_score: number;        // Based on outcomes
  last_used: number;
}

enum NormalizationRule {
  Sum1 = "sum1",      // Weights sum to 1.0
  Max1 = "max1",      // Largest weight = 1.0
  None = "none",      // No normalization
}
```

**Example profiles:**
```typescript
const profiles: WeightProfile[] = [
  {
    profile_id: "profile.scientific_domain",
    name: "Scientific Domain",
    description: "Evidence-based, citations required, no speculation",
    version: "v1.0",
    created_at: Date.now(),
    weights: {
      "scientific_rigor": 0.95,
      "citations_required": 1.0,
      "no_speculation": 0.90,
      "math_check": 0.85,
      "no_metaphor": 0.70,
    },
    normalization_rule: NormalizationRule.Max1,
    domain_scope: "science,research,biology,physics",
    recommended_for: ["research_tasks", "evidence_synthesis"],
    do_not_use_for: ["creative_writing", "brainstorming"],
    usage_count: 0,
    reliability_score: 0.0,
    last_used: 0,
  },
  {
    profile_id: "profile.legal_conservative",
    name: "Legal Conservative",
    description: "Cite statutes, no interpretation, flag ambiguity",
    version: "v1.0",
    created_at: Date.now(),
    weights: {
      "citations_required": 1.0,
      "no_speculation": 1.0,
      "no_metaphor": 0.95,
      "flag_ambiguity": 0.90,
    },
    normalization_rule: NormalizationRule.Max1,
    domain_scope: "legal,compliance,contracts",
    recommended_for: ["contract_review", "compliance_check"],
    do_not_use_for: ["creative_tasks", "exploration"],
    usage_count: 0,
    reliability_score: 0.0,
    last_used: 0,
  },
  // ... more profiles
];
```

**Storage:** `var/relay_ai/weight_profiles/<profile_id>@<version>.json`

**API endpoints:**
```
GET  /api/weight-profiles
GET  /api/weight-profiles/:profile_id
POST /api/weight-profiles
PUT  /api/weight-profiles/:profile_id  (creates new version)
```

---

### **Deliverable 2.3: Profile Application**

**Functions:**
```typescript
async function applyProfileToSCV(
  scv_id: string,
  profile_id: string
): Promise<void> {
  const agent = await loadAgent(scv_id);
  const oldProfile = agent.weight_profile_ref;
  
  // Update agent
  agent.weight_profile_ref = profile_id;
  await storeAgent(agent);
  
  // Create commit
  await createCommit({
    content_type: ContentType.ProfileChange,
    author: "system",
    content: {
      type: "profile_change_persistent",
      agent_id: scv_id,
      from_profile: oldProfile,
      to_profile: profile_id,
      reason: "User changed agent profile",
    },
  });
  
  // Emit event
  await emitEvent({
    type: "profile_changed",
    agent_id: scv_id,
    profile_id,
    scope: "persistent",
  });
}

async function applyProfileToTurn(
  conversation_id: string,
  turn_id: string,
  profile_id: string
): Promise<void> {
  // Create ephemeral override commit
  await createCommit({
    content_type: ContentType.ProfileChange,
    author: "system",
    content: {
      type: "profile_change_ephemeral",
      conversation_id,
      turn_id,
      profile_id,
      reason: "User override for this turn only",
    },
  });
  
  // Mark turn with override
  await markTurnOverride(turn_id, profile_id);
}
```

**UI:**
```html
<!-- Branch Palette Panel -->
<div class="branch-palette">
  <h3>Branch Palette</h3>
  
  <div class="profile-selector">
    <label>Active Profile:</label>
    <select id="profile-dropdown">
      <option value="scientific_domain">Scientific Domain</option>
      <option value="legal_conservative">Legal Conservative</option>
      <option value="fast_exploration">Fast Exploration</option>
      <option value="custom">Custom...</option>
    </select>
  </div>
  
  <div class="branch-weights">
    <h4>Branch Weights</h4>
    <div class="branch-slider">
      <label>Scientific Rigor</label>
      <input type="range" min="0" max="100" value="95" />
      <span>0.95</span>
    </div>
    <div class="branch-slider">
      <label>Citations Required</label>
      <input type="range" min="0" max="100" value="100" />
      <span>1.0</span>
    </div>
    <!-- More sliders -->
  </div>
  
  <div class="apply-controls">
    <button onclick="applyToSCV()">Apply to SCV</button>
    <button onclick="applyToTurn()">Apply to This Message Only</button>
    <button onclick="saveProfile()">Save as New Profile</button>
  </div>
</div>
```

---

### **Deliverable 2.4: Influence Trace**

**Schema:**
```typescript
interface InfluenceTrace {
  trace_id: string;
  response_commit_ref: string;
  timestamp: number;
  
  // Active configuration
  active_profile_id: string;
  active_profile_version: string;
  
  // What influenced this response
  top_influences: Influence[];
  
  // Validation results
  gates_checked: GateCheck[];
  violations: Violation[];
  
  // Overrides
  turn_overrides: Override[];
  
  // Evidence
  citations: CommitRef[];
}

interface Influence {
  branch_id: string;
  branch_name: string;
  weight: number;
  why_triggered: string;      // Short explanation
  impact_score: number;       // 0.0-1.0
}

interface GateCheck {
  gate_id: string;
  gate_name: string;
  passed: boolean;
  details: string;
}

interface Violation {
  gate_id: string;
  severity: Severity;
  description: string;
  suggestion: string;
}

enum Severity {
  Blocking = "blocking",
  Warning = "warning",
  Info = "info",
}

interface Override {
  override_type: string;
  original_value: string;
  overridden_value: string;
  reason: string;
}

type CommitRef = string;  // "conv.user123.c47"
```

**Generation function:**
```typescript
async function generateInfluenceTrace(
  response: string,
  agent_id: string,
  active_profile_id: string,
  gates: Gate[]
): Promise<InfluenceTrace> {
  const profile = await loadWeightProfile(active_profile_id);
  const branches = await loadLogicBranches(Object.keys(profile.weights));
  
  // Calculate influences
  const influences = branches.map(branch => ({
    branch_id: branch.branch_id,
    branch_name: branch.name,
    weight: profile.weights[branch.branch_id],
    why_triggered: detectTrigger(response, branch),
    impact_score: calculateImpact(response, branch),
  })).sort((a, b) => b.impact_score - a.impact_score);
  
  // Run gates
  const gateChecks = await Promise.all(
    gates.map(gate => checkGate(gate, response, profile))
  );
  
  const violations = gateChecks
    .filter(check => !check.passed)
    .map(check => ({
      gate_id: check.gate_id,
      severity: check.severity,
      description: check.description,
      suggestion: check.suggestion,
    }));
  
  // Collect citations
  const citations = extractCitations(response);
  
  return {
    trace_id: generateTraceId(),
    response_commit_ref: getCurrentCommitRef(),
    timestamp: Date.now(),
    active_profile_id,
    active_profile_version: profile.version,
    top_influences: influences.slice(0, 5),  // Top 5
    gates_checked: gateChecks,
    violations,
    turn_overrides: getTurnOverrides(),
    citations,
  };
}
```

**UI display:**
```html
<!-- Influence Trace Overlay (at top of response) -->
<div class="influence-trace-summary">
  <h4>🔍 Influence Trace</h4>
  <div class="profile-info">
    Profile: <strong>Scientific Domain v1.2</strong>
  </div>
  <div class="top-influences">
    <div class="influence-bar">
      <span>Scientific Rigor</span>
      <div class="bar" style="width: 95%"></div>
      <span>0.95</span>
    </div>
    <div class="influence-bar">
      <span>Citations Required</span>
      <div class="bar" style="width: 100%"></div>
      <span>1.00</span>
    </div>
    <div class="influence-bar">
      <span>No Speculation</span>
      <div class="bar" style="width: 90%"></div>
      <span>0.90</span>
    </div>
  </div>
  <div class="gates-status">
    Gates: ✅ All passed
  </div>
  <button onclick="expandTrace()">View Full Trace ▼</button>
</div>
```

---

## 📦 MODULE 3: ROOT HUD (COMPILER/ROUTER/AUDITOR)

### **Deliverable 3.1: Compiler Function**

**Task: Intent → Task Contract**

```typescript
interface TaskContract {
  task_id: string;
  created_at: number;
  created_by: string;          // user_id
  
  // Intent
  objective: string;
  scope: string;
  forbidden_actions: string[];
  success_criteria: string[];
  
  // Configuration
  assigned_scv: string;
  weight_profile_id: string;
  gates: string[];             // gate_ids
  
  // Constraints
  deadline?: number;
  max_tokens?: number;
  
  // Audit
  parent_commit_ref: string;
  status: "pending" | "executing" | "completed" | "failed";
}

async function compileTaskContract(
  intent: string,
  scope: string,
  profile_id: string,
  user_id: string
): Promise<TaskContract> {
  // Parse intent
  const parsed = await parseIntent(intent);
  
  // Determine gates from profile
  const profile = await loadWeightProfile(profile_id);
  const gates = determineGates(profile);
  
  // Create contract
  const contract: TaskContract = {
    task_id: generateTaskId(),
    created_at: Date.now(),
    created_by: user_id,
    objective: parsed.objective,
    scope,
    forbidden_actions: parsed.forbidden || [],
    success_criteria: parsed.success_criteria || [],
    assigned_scv: "",  // To be filled by router
    weight_profile_id: profile_id,
    gates,
    parent_commit_ref: getCurrentCommitRef(),
    status: "pending",
  };
  
  return contract;
}
```

---

### **Deliverable 3.2: Router Function**

**Task: Assign SCVs based on scope + reliability**

```typescript
async function routeTask(
  contract: TaskContract
): Promise<string> {
  // Get candidate SCVs
  const candidates = await findCandidateSCVs(contract.scope);
  
  // Score each candidate
  const scored = await Promise.all(
    candidates.map(async scv => ({
      scv_id: scv.agent_id,
      reliability: await calculateReliability(scv.agent_id, contract.scope),
      scope_match: calculateScopeMatch(scv.scope, contract.scope),
      availability: await checkAvailability(scv.agent_id),
    }))
  );
  
  // Filter available
  const available = scored.filter(s => s.availability);
  
  if (available.length === 0) {
    throw new Error("No available SCVs for this scope");
  }
  
  // Sort by (reliability * scope_match)
  available.sort((a, b) => {
    const scoreA = a.reliability * a.scope_match;
    const scoreB = b.reliability * b.scope_match;
    return scoreB - scoreA;
  });
  
  // Assign to top candidate
  const assigned = available[0];
  contract.assigned_scv = assigned.scv_id;
  
  await storeTaskContract(contract);
  
  return assigned.scv_id;
}

async function calculateReliability(
  scv_id: string,
  scope: string
): Promise<number> {
  const outcomes = await getOutcomes(scv_id, scope);
  
  if (outcomes.total === 0) return 0.5;  // Neutral for new SCVs
  
  const acceptance_rate = outcomes.accepted / outcomes.total;
  const patch_rate = outcomes.patched / outcomes.total;
  const violation_rate = outcomes.violations / outcomes.total;
  
  let score = acceptance_rate - (patch_rate * 0.5) - (violation_rate * 0.8);
  
  // Time decay (recent outcomes weighted higher)
  score = applyTimeDecay(score, outcomes);
  
  // Scope bonus
  const inScope = await isInScope(scv_id, scope);
  const scopeBonus = inScope ? 0.1 : -0.2;
  
  return Math.max(0.0, Math.min(1.0, score + scopeBonus));
}
```

---

### **Deliverable 3.3: Auditor Function**

**Task: Enforce gates, emit traces, preserve commits**

```typescript
async function auditOutput(
  response: string,
  agent_id: string,
  profile_id: string,
  gates: Gate[]
): Promise<AuditResult> {
  // 1. Run gates
  const gateResults = await Promise.all(
    gates.map(gate => runGate(gate, response))
  );
  
  // 2. Collect violations
  const violations = gateResults.filter(r => !r.passed);
  
  // 3. Emit Influence Trace
  const trace = await generateInfluenceTrace(
    response,
    agent_id,
    profile_id,
    gates
  );
  await storeInfluenceTrace(trace);
  
  // 4. Create commit
  const commit = await createCommit({
    content_type: violations.length > 0
      ? ContentType.GateViolation
      : ContentType.AgentResponse,
    author: agent_id,
    content: response,
    influence_trace_ref: trace.trace_id,
    outcome_status: violations.length > 0
      ? OutcomeStatus.Rejected
      : OutcomeStatus.Pending,
  });
  
  // 5. Update reliability (if violations)
  if (violations.length > 0) {
    await decrementReliability(agent_id, 0.1);
  }
  
  return {
    passed: violations.length === 0,
    violations,
    trace_id: trace.trace_id,
    commit_ref: commit.commit_ref,
  };
}

interface AuditResult {
  passed: boolean;
  violations: Violation[];
  trace_id: string;
  commit_ref: string;
}
```

---

## 📦 MODULE 4: GATE SYSTEM

### **Deliverable 4.1: Gate Data Model**

**Schema:**
```typescript
interface Gate {
  gate_id: string;
  name: string;
  description: string;
  version: string;
  
  // Validation
  check_fn: (output: string) => Promise<GateResult>;
  violation_action: ViolationAction;
  
  // Context
  applies_to: string[];        // ["all"] | ["scientific"] | etc
  required_for: string[];      // profile_ids
  
  // Audit
  violation_count: number;
  last_violation?: number;
}

interface GateResult {
  passed: boolean;
  details?: string;
  suggestion?: string;
  severity?: Severity;
}
```

**Example gates:**

```typescript
const gates: Gate[] = [
  {
    gate_id: "gate.no_new_primitives",
    name: "No New Primitives",
    description: "Output must not invent new physics concepts",
    version: "v1.0",
    check_fn: async (output: string) => {
      const primitives = await extractPrimitives(output);
      const known = await loadLockedPrimitives();
      const newPrimitives = primitives.filter(p => !known.includes(p));
      
      if (newPrimitives.length > 0) {
        return {
          passed: false,
          details: `New primitives found: ${newPrimitives.join(", ")}`,
          suggestion: "Use existing primitives or propose architecture commit",
          severity: Severity.Blocking,
        };
      }
      
      return { passed: true };
    },
    violation_action: ViolationAction.Block,
    applies_to: ["all"],
    required_for: [],
    violation_count: 0,
  },
  
  {
    gate_id: "gate.citations_required",
    name: "Citations Required",
    description: "All claims must have citations",
    version: "v1.0",
    check_fn: async (output: string) => {
      const claims = await extractClaims(output);
      const uncited = claims.filter(c => !hasCitation(c, output));
      
      if (uncited.length > 0) {
        return {
          passed: false,
          details: `Uncited claims: ${uncited.join("; ")}`,
          suggestion: "Add commit ref or evidence link for each claim",
          severity: Severity.Blocking,
        };
      }
      
      return { passed: true };
    },
    violation_action: ViolationAction.BlockWithPatch,
    applies_to: ["scientific", "legal"],
    required_for: ["profile.scientific_domain", "profile.legal_conservative"],
    violation_count: 0,
  },
  
  {
    gate_id: "gate.no_semantic_zoom",
    name: "No Semantic Zoom",
    description: "Zoom must be lens change, not reinterpretation",
    version: "v1.0",
    check_fn: async (output: string) => {
      const hasSemanticChange = await detectSemanticChange(output);
      
      if (hasSemanticChange) {
        return {
          passed: false,
          details: "Semantic change detected in zoom operation",
          suggestion: "Zoom reveals detail, never changes meaning",
          severity: Severity.Blocking,
        };
      }
      
      return { passed: true };
    },
    violation_action: ViolationAction.Block,
    applies_to: ["all"],
    required_for: [],
    violation_count: 0,
  },
];
```

---

### **Deliverable 4.2: Gate Execution**

```typescript
async function runGate(
  gate: Gate,
  output: string
): Promise<GateCheck> {
  try {
    const result = await gate.check_fn(output);
    
    if (!result.passed) {
      // Increment violation count
      gate.violation_count++;
      gate.last_violation = Date.now();
      await storeGate(gate);
    }
    
    return {
      gate_id: gate.gate_id,
      gate_name: gate.name,
      passed: result.passed,
      details: result.details || "",
      severity: result.severity,
      suggestion: result.suggestion,
    };
  } catch (error) {
    // Gate execution failed (this is a system error, not a violation)
    return {
      gate_id: gate.gate_id,
      gate_name: gate.name,
      passed: false,
      details: `Gate execution error: ${error.message}`,
      severity: Severity.Blocking,
      suggestion: "Fix gate implementation",
    };
  }
}
```

---

### **Deliverable 4.3: Correction Flow**

**UI flow:**

```
1. SCV produces output
2. Gates run automatically
3. If violation:
   ┌─────────────────────────────────────────┐
   │ ⚠️  Gate Violation: Citations Required │
   │                                         │
   │ Details:                                │
   │   Uncited claim: "Fusion reactors..."  │
   │                                         │
   │ Suggestion:                             │
   │   Add commit ref or source link        │
   │                                         │
   │ [Request Correction] [Override (Admin)]│
   └─────────────────────────────────────────┘
4. User clicks "Request Correction"
5. System calls SCV with correction request
6. SCV produces patched version
7. Gates run again
8. If pass → Accept
9. If fail → Repeat correction
```

**API:**
```typescript
async function requestCorrection(
  response_commit_ref: string,
  violation: Violation
): Promise<string> {
  // Load original response
  const originalCommit = await loadCommit(response_commit_ref);
  const agent_id = originalCommit.author;
  
  // Generate correction prompt
  const correctionPrompt = `
    Your previous response violated a gate:
    
    Gate: ${violation.gate_id}
    Issue: ${violation.description}
    Suggestion: ${violation.suggestion}
    
    Original response:
    ${originalCommit.content}
    
    Please provide a corrected version that passes the gate.
  `;
  
  // Call SCV
  const corrected = await callSCV(agent_id, correctionPrompt);
  
  // Create correction commit
  const correctionCommit = await createCommit({
    content_type: ContentType.CorrectionCommit,
    author: agent_id,
    parent_refs: [response_commit_ref],
    content: corrected,
  });
  
  // Re-run gates
  const audit = await auditOutput(
    corrected,
    agent_id,
    originalCommit.weight_profile_ref,
    await loadGates(originalCommit.weight_profile_ref)
  );
  
  if (audit.passed) {
    // Success
    await updateOutcome(agent_id, {
      type: "patched",
      original_commit: response_commit_ref,
      corrected_commit: correctionCommit.commit_ref,
    });
  }
  
  return correctionCommit.commit_ref;
}
```

---

## 📦 MODULE 5: OUTCOME TRACKING + RELIABILITY

### **Deliverable 5.1: OutcomeRecord Schema**

```typescript
interface OutcomeRecord {
  outcome_id: string;
  agent_id: string;
  task_id: string;
  scope: string;
  timestamp: number;
  
  // Outcome
  type: OutcomeType;
  response_commit_ref: string;
  correction_commit_ref?: string;
  
  // Metrics
  patch_count: number;          // How many corrections needed
  violation_count: number;      // How many gates violated
  time_to_completion: number;   // ms
  
  // User feedback
  user_rating?: number;         // 1-5 stars
  user_notes?: string;
}

enum OutcomeType {
  Accepted = "accepted",        // ✅ User accepted first try
  Patched = "patched",          // ⚠️ Required corrections
  Rejected = "rejected",        // ❌ Blocked by gates, never fixed
  Timeout = "timeout",          // ⏱️ Took too long
}
```

---

### **Deliverable 5.2: Reliability Calculation**

```typescript
async function calculateReliability(
  agent_id: string,
  scope: string,
  window_days: number = 30
): Promise<ReliabilityReport> {
  const outcomes = await getOutcomes(agent_id, scope, window_days);
  
  if (outcomes.length === 0) {
    return {
      reliability_score: 0.5,  // Neutral for new agents
      total_outcomes: 0,
      acceptance_rate: 0,
      patch_rate: 0,
      violation_rate: 0,
      avg_time_to_completion: 0,
    };
  }
  
  const total = outcomes.length;
  const accepted = outcomes.filter(o => o.type === OutcomeType.Accepted).length;
  const patched = outcomes.filter(o => o.type === OutcomeType.Patched).length;
  const rejected = outcomes.filter(o => o.type === OutcomeType.Rejected).length;
  
  const acceptance_rate = accepted / total;
  const patch_rate = patched / total;
  const violation_rate = outcomes.reduce((sum, o) => sum + o.violation_count, 0) / total;
  
  // Base score
  let score = acceptance_rate - (patch_rate * 0.5) - (violation_rate * 0.8);
  
  // Time decay (recent outcomes weighted higher)
  score = applyTimeDecay(score, outcomes);
  
  // Scope bonus
  const inScope = await isSpecialistInScope(agent_id, scope);
  const scopeBonus = inScope ? 0.1 : -0.2;
  
  const reliability_score = Math.max(0.0, Math.min(1.0, score + scopeBonus));
  
  return {
    reliability_score,
    total_outcomes: total,
    acceptance_rate,
    patch_rate,
    violation_rate,
    avg_time_to_completion: outcomes.reduce((sum, o) => sum + o.time_to_completion, 0) / total,
  };
}

interface ReliabilityReport {
  reliability_score: number;
  total_outcomes: number;
  acceptance_rate: number;
  patch_rate: number;
  violation_rate: number;
  avg_time_to_completion: number;
}
```

---

## 🎯 IMPLEMENTATION PRIORITY

**Phase 1: Foundation (Week 1-2)**
- ✅ CommitNode data model + storage
- ✅ Conversation filament basics
- ✅ LogicBranch data model
- ✅ WeightProfile data model

**Phase 2: Core Mechanics (Week 3-4)**
- ✅ Training Pack compilation
- ✅ Agent onboarding action
- ✅ Profile application
- ✅ Influence Trace generation

**Phase 3: Enforcement (Week 5-6)**
- ✅ Gate system
- ✅ Violation detection
- ✅ Correction flow
- ✅ Outcome tracking

**Phase 4: UI (Week 7-8)**
- ✅ Header/proof zoom
- ✅ Branch Palette panel
- ✅ Influence Trace overlay
- ✅ Gate violation warnings
- ✅ Onboard button

**Phase 5: Root HUD (Week 9-10)**
- ✅ Compiler function
- ✅ Router function
- ✅ Auditor function
- ✅ Reliability dashboard

---

## ✅ ACCEPTANCE CRITERIA

**Module 1: Conversation as Filament**
- [ ] Conversations stored as append-only JSONL
- [ ] Training Packs compile deterministically (same input → same pack)
- [ ] Onboard button spawns agents with sealed packs
- [ ] Header/proof zoom preserves semantics

**Module 2: Logic Branches + Profiles**
- [ ] Logic Branches versioned and loadable
- [ ] Weight Profiles apply to SCVs (persistent) and turns (ephemeral)
- [ ] Influence Traces emit on every response
- [ ] Profile changes create commits

**Module 3: Root HUD**
- [ ] Compiler turns intent → Task Contract
- [ ] Router assigns SCVs by reliability + scope
- [ ] Auditor enforces gates, emits traces, preserves commits

**Module 4: Gates**
- [ ] Gates run automatically on every output
- [ ] Violations block/warn as configured
- [ ] Correction flow works (request → patch → re-check)
- [ ] Reliability decrements on violations

**Module 5: Outcomes**
- [ ] Outcome Records track accepted/patched/rejected
- [ ] Reliability score updates based on outcomes
- [ ] Historical reliability reconstructable from commits

---

## 📊 SUCCESS METRICS

**User-facing:**
1. **No copy/paste:** Users spawn new agents from commits (not manual copy/paste)
2. **No keyword stuffing:** Users select profiles (not stuff prompts with keywords)
3. **Header-level operation:** Users read headers 90% of time (zoom 10%)
4. **Influence visibility:** Users can always answer "What influenced this?"

**System-level:**
1. **Determinism:** Same Training Pack + Profile → Same behavior
2. **Auditability:** Historical influence always reconstructable
3. **Physics enforcement:** Gates catch violations automatically
4. **Reliability improvement:** Agents improve over time (outcome tracking works)

---

## 🚫 NON-NEGOTIABLES

1. **No history mutation:** Everything append-only
2. **No hidden steering:** Profiles + Traces always visible
3. **No silent learning:** Artifacts only, no hidden training
4. **No semantic zoom:** Zoom = lens, never reinterpretation
5. **No new primitives without lock:** Gates enforce physics
6. **No black boxes:** Everything commit-addressable

---

## 📋 CHECKLIST FOR CLAUDE

**Before starting:**
- [ ] Read architecture@c12 fully
- [ ] Understand "conversation as filament" metaphysics
- [ ] Understand "Profiles + Gates + Traces > weights"
- [ ] Understand "Root HUD = Compiler/Router/Auditor"

**While implementing:**
- [ ] All schemas match spec exactly
- [ ] All storage is append-only (JSONL)
- [ ] All changes create commits
- [ ] All outputs emit Influence Traces
- [ ] All gates run automatically
- [ ] No hidden state

**Testing:**
- [ ] Deterministic Training Pack compilation
- [ ] Agent onboarding reproducibility
- [ ] Profile changes create commits
- [ ] Gates detect violations
- [ ] Corrections work
- [ ] Reliability updates
- [ ] Historical reconstruction works

---

## 🎯 FINAL NOTES

**This is not "AI UX."**

This is:
> **Relay's filament physics applied to the AI conversation substrate.**

**Key principles:**
1. Conversations = filaments (append-only, replayable, branchable)
2. Training = compilation (deterministic, mechanical)
3. Steering = explicit (profiles, not keywords)
4. Learning = artifacts (auditable, not hidden)
5. Root HUD = structural (compiler/router/auditor, not oracle)

**If you violate these principles, you're not "being creative" —**  
**You're breaking physics.**

---

**END OF RELAY MODULE SPEC: ROOT AI COGNITIVE SUBSTRATE**

**Status:** READY FOR IMPLEMENTATION  
**Estimated time:** 10 weeks (8 phases)  
**Architecture basis:** architecture@c12  
**Next step:** Begin Phase 1 (Foundation)

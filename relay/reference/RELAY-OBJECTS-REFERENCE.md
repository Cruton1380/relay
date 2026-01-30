# RELAY OBJECTS REFERENCE

**Purpose:** Canonical definitions of all Relay objects  
**Last Updated:** 2026-01-29  
**Status:** 🔒 Locked

---

## 🎯 HOW TO USE THIS REFERENCE

**This is an API-like reference for Relay's reality model.**

**Each object includes:**
- What it is (definition)
- What it is NOT (anti-patterns)
- Fields (canonical schema)
- Allowed mutations
- Invariants (physics constraints)
- Governed by (architecture commits)
- Related objects

**Format:** Alphabetical by object name

---

## 📋 OBJECT CATEGORIES

### **Identity & Access**
- AccessDecision
- AccessPathStep
- Attestation
- AuthorityGrant
- Binding
- IdentityCommit
- IdentityFilament
- IdentityState
- PresenceTier
- Scar

### **Coordination & Work**
- Commitment
- Task
- TaskPriority
- TaskProgress
- TaskState
- Vote

### **Physical World**
- Building
- BuildingBeacon
- BuildingStatus
- BuildingType
- CatalogItem
- ForceAnchor
- ForceUnit
- GeoAnchor
- GeoPosition
- Shipment
- ShipmentState
- Zone

### **Cognitive & AI**
- DialogContextBundle
- LogicBranch
- MeaningFrame
- PromptBounceChain
- SCV
- TrainingPack
- TranslationBranch
- WeightProfile

### **Verification & Audit**
- AuditAssertion
- AuditEvidence
- AuditFinding
- AuditOpinion
- MaterialityRule
- RealityCoverageMap

### **Conflict & Damage**
- DamageScar
- Lens

### **System Objects**
- Commit
- CommitRef
- Filament
- RenderSpec
- DirectRelayArtifact

### **Genesis Objects**
- MasterPromptFilament
- MasterConversationFilament

---

## 📖 OBJECT DEFINITIONS

---

### **AccessDecision**

**What it is:**
A derived decision on whether a viewer can access a target, with explanation path.

**What it is NOT:**
- ❌ A stored permission
- ❌ A role assignment
- ❌ A static ACL entry

**Fields:**
```rust
struct AccessDecision {
    allow: bool,
    reason: String,
    path: Vec<AccessPathStep>,
    blocked_by: Option<CommitRef>
}
```

**Invariants:**
- Must include explanation path (even if deny)
- If blocked, must reference blocking commit
- Decision must be deterministic (same inputs → same output)

**Governed by:** [c10.Ontology], PR#9

**Related:** AccessPathStep, IdentityState, Zone

---

### **AccessPathStep**

**What it is:**
One step in the explanation of why access was granted or denied.

**Fields:**
```rust
struct AccessPathStep {
    commit_ref: CommitRef,
    effect: String  // e.g. "ATTESTATION:DEPT_MEMBER->dept.finance"
}
```

**Governed by:** PR#9

**Related:** AccessDecision

---

### **Attestation**

**What it is:**
A claim made by one identity about another identity.

**Fields:**
```rust
struct Attestation {
    attester_id: IdentityId,
    attestation_type: AttestationType,
    target_ref: String,
    confidence: f64,
    evidence_refs: Vec<CommitRef>
}
```

**Invariants:**
- Must have evidence references
- Confidence must be 0.0-1.0
- Cannot self-attest for privileged claims

**Governed by:** PR#9

**Related:** IdentityCommit, AuthorityGrant

---

### **AuditAssertion**

**What it is:**
A claim about reality that must be verified through audit procedures.

**What it is NOT:**
- ❌ A fact
- ❌ Self-certified truth
- ❌ An opinion

**Fields:**
```rust
struct AuditAssertion {
    assertion_id: String,
    assertion_type: AssertionType,  // Existence, Completeness, Accuracy, etc.
    target_ref: String,
    claim: String,
    evidence_required: Vec<EvidenceType>,
    materiality: MaterialityLevel,
    verification_status: VerificationStatus
}
```

**Invariants:**
- Must specify evidence requirements
- Cannot be self-verified
- Must link to MaterialityRule
- Status changes create commits

**Governed by:** [c16.UniversalAudit]

**Related:** AuditEvidence, AuditFinding, MaterialityRule

---

### **AuditEvidence**

**What it is:**
Verifiable proof linked to an audit assertion.

**Fields:**
```rust
struct AuditEvidence {
    evidence_id: String,
    assertion_id: String,
    evidence_type: EvidenceType,
    filament_refs: Vec<String>,
    commit_refs: Vec<CommitRef>,
    external_refs: Vec<String>,
    collected_at: Timestamp,
    collector_id: String
}
```

**Invariants:**
- Must link to assertion
- Must have filament or external refs
- Cannot be retroactively edited
- Collector cannot be same as asserter (independence)

**Governed by:** [c16.UniversalAudit]

**Related:** AuditAssertion, AuditFinding

---

### **AuditFinding**

**What it is:**
A documented mismatch, omission, or violation discovered during audit.

**What it is NOT:**
- ❌ Deletable
- ❌ Hideable
- ❌ Retroactively editable

**Fields:**
```rust
struct AuditFinding {
    finding_id: String,
    assertion_id: String,
    severity: Severity,
    description: String,
    evidence_refs: Vec<String>,
    remediation_required: bool,
    remediation_refs: Vec<CommitRef>,
    status: FindingStatus  // Open, Remediated, Accepted Risk
}
```

**Invariants:**
- Findings are scars (permanent)
- Status changes are commits (traceable)
- Cannot be deleted (only closed with remediation)
- Must reference evidence

**Governed by:** [c16.UniversalAudit]

**Related:** AuditAssertion, AuditEvidence, AuditOpinion

---

### **AuditOpinion**

**What it is:**
A scoped conclusion issued by an independent auditor about reality coverage.

**Fields:**
```rust
struct AuditOpinion {
    opinion_id: String,
    auditor_id: String,
    scope: String,
    opinion_type: OpinionType,  // Verified, Qualified, Adverse, Disclaimer
    findings_refs: Vec<String>,
    conclusion: String,
    issued_at: Timestamp,
    valid_until: Option<Timestamp>
}
```

**Invariants:**
- Must be scoped (not global)
- Must reference findings
- Auditor must be independent
- Opinion is immutable once issued

**Governed by:** [c16.UniversalAudit]

**Related:** AuditFinding, MaterialityRule

---

### **AuthorityGrant**

**What it is:**
Temporary, scoped authority delegated to an identity.

**Fields:**
```rust
struct AuthorityGrant {
    granted_to: IdentityId,
    granted_by: IdentityId,
    scope: String,
    expires_at: Timestamp,
    authority_type: AuthorityType,
    revoked: bool
}
```

**Invariants:**
- Must have expiration
- Must be scoped
- Revocation creates commit
- Cannot be retroactively granted

**Governed by:** [c8.DelegatedInfluence], PR#9

**Related:** IdentityCommit, Vote

---

### **Binding**

**What it is:**
A structural relationship between an identity and a scope (organization, department, building).

**Fields:**
```rust
struct Binding {
    identity_id: IdentityId,
    binding_type: BindingType,
    target_ref: String,
    established_at: Timestamp,
    evidence_ref: CommitRef
}
```

**Invariants:**
- Bindings are additive (not replacing)
- Must have evidence
- Cannot be silently removed (requires SCAR or new commit)

**Governed by:** PR#9

**Related:** IdentityCommit, AccessDecision

---

### **Building**

**What it is:**
A spatial anchor with capabilities, not just a location.

**What it is NOT:**
- ❌ Just a map pin
- ❌ A static asset
- ❌ Purely decorative

**Fields:**
```rust
struct Building {
    building_id: String,
    building_type: BuildingType,
    geo_anchor: GeoAnchor,
    owner_ref: Option<String>,
    catalog_ref: Option<String>,
    status: BuildingStatus,
    capacity: Option<u32>,
    zone_id: Option<String>
}
```

**Invariants:**
- Buildings are units (tradable, governable)
- Buildings are tiles (accumulate history)
- Position derived from geo_anchor (deterministic)
- Status changes create commits

**Governed by:** [c9.StarCraft], [c10.Ontology], PR#6

**Related:** CatalogItem, Zone, BuildingBeacon, Task, Shipment

---

### **BuildingBeacon**

**What it is:**
A spatial anchor for sessions, tracking participants and applying zone rules.

**Fields:**
```rust
struct BuildingBeacon {
    beacon_id: String,
    building_id: String,
    active_sessions: Vec<String>,
    participants: Vec<String>,
    capacity: u32,
    zone_rules_applied: Vec<String>
}
```

**Governed by:** [c14.Presence]

**Related:** Building, Session, Zone

---

### **CatalogItem**

**What it is:**
A producible unit available at a building (not a world object).

**What it is NOT:**
- ❌ Placed on globe
- ❌ Has transform
- ❌ A world geometry

**Fields:**
```rust
struct CatalogItem {
    item_id: String,
    name: String,
    production_time: u32,  // seconds
    escrow_required: f64,
    availability: Availability
}
```

**Invariants:**
- Lives in building.props.catalog[]
- NOT a RenderSpec node
- Belongs inside SelectedBuildingPanel (UI)

**Governed by:** [c9.StarCraft], PR#6

**Related:** Building, Task

---

### **Commit**

**What it is:**
An atomic execution event in a filament.

**What it is NOT:**
- ❌ A change proposal
- ❌ A draft
- ❌ Undoable

**Fields:**
```rust
struct Commit {
    commit_ref: CommitRef,
    filament_id: String,
    commit_index: u32,
    payload: serde_json::Value,
    causal_refs: Vec<CommitRef>,
    timestamp: Timestamp,
    author: String
}
```

**Invariants:**
- Commits are immutable
- Commits require causal_refs
- commit_index is monotonic per filament
- Commits are execution (not tooling)

**Governed by:** [c0.Filaments], [c1.Commits]

**Related:** Filament, CommitRef

---

### **CommitRef**

**What it is:**
A stable reference to a specific commit.

**Format:**
```
{filament_id}@c{commit_index}
```

**Examples:**
- `work.W123@c45`
- `identity.alice@c142`
- `architecture@c10`

**Governed by:** [c0.Filaments]

**Related:** Commit

---

### **DamageScar**

**What it is:**
An irreversible damage event to a building or force unit.

**Fields:**
```rust
struct DamageScar {
    scar_id: String,
    target_ref: String,  // building or force unit
    damage_type: DamageType,
    severity: Severity,
    occurred_at: Timestamp,
    evidence_refs: Vec<CommitRef>,
    verification_status: VerificationStatus,
    disputed: bool
}
```

**Invariants:**
- Scars are permanent (never deleted)
- Must have evidence
- Disputes create finding objects
- Unverified damage is marked as such

**Governed by:** [c15.Conflict]

**Related:** ForceUnit, Building, AuditFinding

---

### **DialogContextBundle**

**What it is:**
A paired dialog (user prompt + Root HUD response) sent to an SCV for task execution.

**What it is NOT:**
- ❌ A naked prompt
- ❌ A single message
- ❌ Uncontextualized instruction

**Fields:**
```rust
struct DialogContextBundle {
    bundle_id: String,
    user_prompt: String,
    root_response: String,
    task_contract: Option<String>,
    active_profile: Option<String>,
    active_gates: Vec<String>,
    trace_ref: Option<String>,
    created_at: Timestamp
}
```

**Invariants:**
- SCVs receive bundles by default (not naked prompts)
- Bundle preserves negotiated meaning
- Includes active cognitive constraints

**Governed by:** [c12.Cognitive], Module 2

**Related:** PromptBounceChain, TrainingPack, LogicBranch

---

### **DirectRelayArtifact**

**What it is:**
A verbatim transfer between agents with zero semantic mutation.

**Fields:**
```rust
struct DirectRelayArtifact {
    artifact_id: String,
    source_agent: String,
    target_agent: String,
    content_ref: String,
    transfer_mode: "verbatim",
    checksum: String,  // SHA-256
    created_at: Timestamp,
    context_note: Option<String>
}
```

**Invariants:**
- Checksum must match (fails if mutated)
- Bypasses MeaningFrame compilation
- No bounce tracking
- Transfer integrity auditable

**Governed by:** [c12.Cognitive], Direct Relay Mode

**Related:** DialogContextBundle

---

### **Filament**

**What it is:**
An append-only value identity carried through time.

**What it is NOT:**
- ❌ A database row
- ❌ A mutable object
- ❌ A transaction log

**Fields:**
```rust
struct Filament {
    filament_id: String,
    commits: Vec<Commit>,
    current_state: serde_json::Value  // derived, not stored
}
```

**Invariants:**
- Filaments never mutate, only extend
- Filaments conserve value and history
- Filaments never fork into conflicting histories
- Zoom reveals temporal spacing, not new semantics

**Governed by:** [c0.Filaments]

**Related:** Commit, CommitRef

---

### **ForceAnchor**

**What it is:**
A military base or formation treated as a special building.

**Fields:**
```rust
struct ForceAnchor {
    anchor_id: String,
    building_id: String,  // extends Building
    force_type: ForceType,
    capacity: u32,
    lod_level: u8,
    zone_constraints: Vec<String>
}
```

**Governed by:** [c15.Conflict]

**Related:** Building, ForceUnit, Zone

---

### **ForceUnit**

**What it is:**
A military or power asset rendered as a StarCraft-style unit.

**What it is NOT:**
- ❌ Hidden by default
- ❌ Deniable
- ❌ Retroactively movable

**Fields:**
```rust
struct ForceUnit {
    unit_id: String,
    unit_type: ForceUnitType,  // Battleship, Fighter, Missile System, etc.
    position: GeoPosition,
    lod_level: u8,  // 0-3
    movement_vector: Option<MovementVector>,
    anchor_ref: Option<String>,
    status: ForceUnitStatus,
    damage_scars: Vec<String>
}
```

**Invariants:**
- All critical force objects MUST exist
- LOD governs detail, not existence
- Movement creates commits
- No hidden force
- No invisible damage

**Governed by:** [c15.Conflict]

**Related:** ForceAnchor, DamageScar, Lens

---

### **GeoAnchor**

**What it is:**
A latitude/longitude + optional altitude anchor for buildings.

**Fields:**
```rust
struct GeoAnchor {
    lat: f64,
    lon: f64,
    altitude_meters: Option<f64>
}
```

**Invariants:**
- lat: -90.0 to 90.0
- lon: -180.0 to 180.0
- Once set, position is deterministic

**Governed by:** PR#6

**Related:** Building

---

### **IdentityCommit**

**What it is:**
A commit to an identity filament that changes identity state over time.

**Fields:**
```rust
struct IdentityCommit {
    commit_ref: CommitRef,
    identity_id: IdentityId,
    commit_index: u32,
    payload: IdentityPayload,  // SelfClaim, Attestation, Binding, Scar, AuthorityGranted
    causal_refs: Vec<CommitRef>,
    timestamp: Timestamp
}
```

**Governed by:** PR#9

**Related:** IdentityFilament, IdentityState

---

### **IdentityFilament**

**What it is:**
Identity as a filament (not a profile).

**What it is NOT:**
- ❌ A fixed account
- ❌ A credential store
- ❌ A static role

**Fields:**
```rust
struct IdentityFilament {
    identity_id: String,
    commits: Vec<IdentityCommit>,
    current_state: IdentityState  // derived
}
```

**Invariants:**
- Identity IS a filament (not just "has" a filament)
- Identity evolves via commits
- Identity can be attested by others
- Identity can accumulate scars

**Governed by:** [c10.Ontology], PR#9

**Related:** IdentityCommit, IdentityState, AccessDecision

---

### **IdentityState**

**What it is:**
The derived current state of an identity by replaying its commits.

**Fields:**
```rust
struct IdentityState {
    identity_id: String,
    self_claims: Vec<String>,
    attestations: Vec<Attestation>,
    bindings: Vec<Binding>,
    scars: Vec<Scar>,
    authority_grants: Vec<AuthorityGrant>,
    as_of_commit: CommitRef
}
```

**Invariants:**
- State is derived, not stored
- Replay must be deterministic
- State includes scars (not hidden)

**Governed by:** PR#9

**Related:** IdentityFilament, AccessDecision

---

### **Lens**

**What it is:**
A visibility filter that adjusts LOD and context based on a question or mode.

**Types:**
- Global Conflict State
- Force Mobilization
- Escalation Risk
- Civilian Impact

**Fields:**
```rust
struct Lens {
    lens_id: String,
    lens_type: LensType,
    filters: Vec<Filter>,
    aggregation_rules: Vec<AggregationRule>,
    alert_thresholds: Vec<AlertThreshold>
}
```

**Governed by:** [c15.Conflict]

**Related:** ForceUnit, LOD

---

### **LogicBranch**

**What it is:**
A named, versioned unit of Root AI cognition (prior/heuristic/lens/constraint).

**Fields:**
```rust
struct LogicBranch {
    branch_id: String,
    name: String,
    description: String,
    constraints: Vec<String>,
    biases: Vec<String>,
    validation_mode: String,
    version: String
}
```

**Invariants:**
- Branches are versioned (commit-addressable)
- Weight changes create commits
- No hidden steering

**Governed by:** [c12.Cognitive]

**Related:** WeightProfile, InfluenceTrace

---

### **MasterConversationFilament**

**What it is:**
The preserved reasoning dialogue between humans and AIs that led to Relay.

**What it is NOT:**
- ❌ Documentation
- ❌ Authoritative specs
- ❌ Marketing narrative

**Purpose:**
Pedagogical understanding - enables "Build Relay Alongside the Author" learning.

**Fields:**
```rust
struct MasterConversationFilament {
    filament_id: "relay/master_conversation",
    participants: Vec<Participant>,
    conversations: Vec<Conversation>,
    reading_order: Vec<ReadingChapter>,
    replay_guidance: String,
    audit_assertions: Vec<AuditAssertionType>
}
```

**Invariants:**
- Verbatim transcripts preserved
- Time-ordered
- Links to MasterPromptFilament stages
- Non-authoritative (context only)
- No retroactive editing

**Governed by:** Genesis filaments

**Related:** MasterPromptFilament

---

### **MasterPromptFilament**

**What it is:**
The canonical construction path of Relay - how invariants were discovered and locked.

**What it is NOT:**
- ❌ User documentation
- ❌ Implementation guide
- ❌ Prose explanation

**Purpose:**
Faithful reconstruction protocol - enables rebuilding Relay without semantic drift.

**Fields:**
```rust
struct MasterPromptFilament {
    filament_id: "relay/master_prompt",
    language_level: LanguageLevel,
    stages: Vec<ConstructionStage>,
    toc_ref: TocReference,
    replay_instructions: String,
    audit_assertions: Vec<AuditAssertionType>
}
```

**Invariants:**
- Stages ordered by discovery
- Verbatim prompts/responses preserved
- Invariants explicitly locked per stage
- Discarded ideas documented
- Replayable without interpretation

**Governed by:** Genesis filaments

**Related:** MasterConversationFilament, Architecture commits

---

### **MaterialityRule**

**What it is:**
A rule defining when reality becomes material enough to require audit verification.

**Fields:**
```rust
struct MaterialityRule {
    rule_id: String,
    domain: String,
    threshold: MaterialityThreshold,
    verification_required: Vec<AssertionType>,
    applies_to: Vec<String>
}
```

**Invariants:**
- Rules must be explicit (no implied thresholds)
- Materiality drives audit intensity
- No global "everything is material" rule

**Governed by:** [c16.UniversalAudit]

**Related:** AuditAssertion, AuditOpinion

---

### **MeaningFrame**

**What it is:**
A structured, language-neutral representation of what a message claims/requests/commits to.

**What it is NOT:**
- ❌ The text itself
- ❌ A translation
- ❌ An interpretation

**Fields:**
```rust
struct MeaningFrame {
    meaning_id: String,
    message_id: String,
    speech_act: SpeechAct,
    entities: Vec<String>,
    actions: Vec<String>,
    constraints: Vec<String>,
    risk_level: RiskLevel,
    glossary_refs: Vec<String>,
    confidence: f64
}
```

**Invariants:**
- Translations bind to MeaningFrame (not raw text)
- MeaningFrame is versioned (descendants allowed)
- Clarifications create new versions (not edits)
- Alignment checks validate against MeaningFrame

**Governed by:** [c17.Communication]

**Related:** TranslationBranch, AlignmentCheck

---

### **PromptBounceChain**

**What it is:**
A filament tracking how many agents a prompt bounced through before acceptance.

**Fields:**
```rust
struct PromptBounceChain {
    chain_id: String,
    origin_prompt_ref: String,
    bounce_hops: Vec<BounceHop>,
    finalized_output_ref: Option<String>,
    accepted_by_user: bool,
    patch_count: u32,
    time_to_accept: Option<u64>
}
```

**Invariants:**
- Bounce chains are filaments (allow forks)
- Forks require SelectionCommit
- High bounce count triggers glossary requirement
- Maturity is local/scoped (not global score)

**Governed by:** [c12.Cognitive], Module 2

**Related:** DialogContextBundle, TrainingPack

---

### **RealityCoverageMap**

**What it is:**
A systematic map of reality domains to detect gaps in representation.

**Fields:**
```rust
struct RealityCoverageMap {
    map_id: String,
    domains: Vec<Domain>,
    coverage_assertions: Vec<CoverageAssertion>,
    blind_spots: Vec<BlindSpot>,
    as_of_commit: CommitRef
}
```

**Invariants:**
- Systematic (not cherry-picked)
- Detects omissions
- Updated continuously

**Governed by:** [c16.UniversalAudit]

**Related:** MaterialityRule, AuditFinding

---

### **RenderSpec**

**What it is:**
A deterministic JSON scene graph for Layer 3 rendering.

**What it is NOT:**
- ❌ UI state
- ❌ Animation instructions
- ❌ Runtime logic

**Fields:**
```rust
struct RenderSpec {
    schema_version: "relay-render-v1",
    generated_from: GeneratedFrom,
    nodes: Vec<RenderNode>
}
```

**Invariants:**
- No randomness
- Semantic materials (not RGB)
- Stable IDs (deterministic)
- No time-dependent geometry
- Filament timeboxes have commit_refs

**Governed by:** [c5.RenderSpec]

**Related:** Building, Task, Shipment, ForceUnit

---

### **Scar**

**What it is:**
A permanent record of failure, rejection, or violation on an identity.

**Fields:**
```rust
struct Scar {
    scar_id: String,
    scar_type: ScarType,
    target_scope: String,
    applied_at: Timestamp,
    evidence_ref: CommitRef,
    severity: Severity
}
```

**Invariants:**
- Scars are permanent (never deleted)
- Scars can block access
- Scars are visible in identity state
- Rehabilitation possible via verified actions

**Governed by:** [c10.Ontology], PR#9

**Related:** IdentityState, AccessDecision

---

### **SCV**

**What it is:**
A specialized AI agent (worker unit) that performs scoped tasks.

**What it is NOT:**
- ❌ Autonomous authority
- ❌ Truth creator
- ❌ Decision maker

**Fields:**
```rust
struct SCV {
    scv_id: String,
    role: SCVRole,
    training_pack_ref: String,
    weight_profile_ref: String,
    active_tasks: Vec<String>,
    reliability_score: Option<f64>,
    consults_root: bool
}
```

**Invariants:**
- SCVs propose, never assert
- SCVs consult Root for alignment
- SCVs operate within scope
- Limited to 3-5 specialized agents

**Governed by:** [c11.RootAI], [c12.Cognitive]

**Related:** TrainingPack, DialogContextBundle, Task

---

### **Session**

**What it is:**
A first-class spatial object requiring consent, anchored in space/time.

**Types:**
- Duel
- Co-op RTS
- Shared Build/Edit
- Spectate

**Fields:**
```rust
struct Session {
    session_id: String,
    session_type: SessionType,
    participants: Vec<String>,
    beacon_ref: Option<String>,
    zone_id: String,
    consent_required: bool,
    created_at: Timestamp,
    expires_at: Option<Timestamp>
}
```

**Invariants:**
- Sessions require consent
- Sessions logged as commits
- Sessions apply zone rules
- Sessions anchored to BuildingBeacon

**Governed by:** [c14.Presence]

**Related:** BuildingBeacon, Zone

---

### **Shipment**

**What it is:**
A physical logistics unit (drone) that visualizes logistics on the 3D globe.

**What it is NOT:**
- ❌ A tracking number
- ❌ Abstract logistics
- ❌ Hidden delivery

**Fields:**
```rust
struct Shipment {
    shipment_id: String,
    task_ref: String,
    carrier_type: CarrierType,
    route: Vec<GeoPosition>,
    current_position: GeoPosition,
    state: ShipmentState,
    created_at: Timestamp,
    estimated_arrival: Timestamp
}
```

**Invariants:**
- Position calculated deterministically (created_at + estimated_arrival + route)
- Shipments are units (visible on globe)
- Lifecycle: Created → InTransit → Arrived / Failed

**Governed by:** [c9.StarCraft], PR#8

**Related:** Task, GeoPosition, Building

---

### **Task**

**What it is:**
A unit production order or work in progress.

**What it is NOT:**
- ❌ A todo item
- ❌ A calendar event
- ❌ Abstract work

**Fields:**
```rust
struct Task {
    task_id: String,
    task_type: TaskType,
    state: TaskState,
    progress: TaskProgress,
    priority: TaskPriority,
    escrow_locked: Option<f64>,
    building_ref: Option<String>,
    assignee_ref: Option<String>,
    created_at: Timestamp,
    failure_count: u32
}
```

**Invariants:**
- Tasks are units (in build queue)
- Lifecycle: Queued → Packing → Dispatched → InTransit → Delivered / Failed
- Task completion releases escrow
- Failures are scars (not hidden)

**Governed by:** [c7.Gauges], [c9.StarCraft], PR#7

**Related:** Building, Shipment, SCV

---

### **TrainingPack**

**What it is:**
A deterministic compilation of knowledge used to onboard an SCV from a specific commit.

**Fields:**
```rust
struct TrainingPack {
    pack_id: String,
    scope_id: String,
    cutoff_commit: CommitRef,
    locked_invariants: Vec<String>,
    active_decisions: Vec<String>,
    open_questions: Vec<String>,
    glossary: Vec<GlossaryEntry>,
    do_not_do: Vec<String>,
    current_tasks: Vec<String>,
    citations: Vec<CommitRef>
}
```

**Invariants:**
- Training is deterministic compilation
- Pack is sealed (immutable)
- Pack is versioned by cutoff commit
- No hidden training

**Governed by:** [c12.Cognitive]

**Related:** SCV, PromptBounceChain, LogicBranch

---

### **TranslationBranch**

**What it is:**
A translation artifact tied to a MeaningFrame.

**Fields:**
```rust
struct TranslationBranch {
    translation_id: String,
    meaning_id: String,
    target_language: String,
    target_modality: String,
    rendered_content_ref: String,
    translation_mode: TranslationMode,  // Strict, Conversational, Scientific, Legal
    evidence: AlignmentEvidence,
    status: TranslationStatus  // Provisional, Verified, Disputed
}
```

**Invariants:**
- Translations are descendants (not replacements)
- Strict mode forbids paraphrase
- Alignment failures mark Provisional
- Disputes create fork objects

**Governed by:** [c17.Communication]

**Related:** MeaningFrame, AlignmentCheck

---

### **Vote**

**What it is:**
Delegated authority (force lending), not truth production.

**What it is NOT:**
- ❌ A belief statement
- ❌ Permanent power
- ❌ Global authority

**Fields:**
```rust
struct Vote {
    vote_id: String,
    voter_id: String,
    target_ref: String,
    influence_amount: f64,
    expires_at: Timestamp,
    revoked: bool,
    scope: String
}
```

**Invariants:**
- Votes decay over time
- Votes are revocable
- Votes are scope-bound
- Votes reward duration, not spikes

**Governed by:** [c8.DelegatedInfluence]

**Related:** AuthorityGrant, Zone

---

### **WeightProfile**

**What it is:**
A normalized set of weights applied to LogicBranches for an SCV or turn.

**Fields:**
```rust
struct WeightProfile {
    profile_id: String,
    weights: HashMap<String, f64>,  // branch_id -> weight (0.0-1.0)
    normalization_rule: String,
    created_from_commitRef: CommitRef,
    notes: Option<String>
}
```

**Invariants:**
- Weights are normalized
- Profile changes create commits
- Applied per-SCV or per-turn

**Governed by:** [c12.Cognitive]

**Related:** LogicBranch, InfluenceTrace, SCV

---

### **Zone**

**What it is:**
A rule-based spatial/contextual constraint region.

**What it is NOT:**
- ❌ A global rule
- ❌ Implicit norms
- ❌ Discretionary enforcement

**Fields:**
```rust
struct Zone {
    zone_id: String,
    location: LocationSpec,
    context: String,
    constraint_set: ConstraintSet,
    enforcement_mode: EnforcementMode,  // Hard, Soft, Deferred, Informational
    transition_rules: Vec<TransitionRule>,
    governed_by: Vec<String>
}
```

**Invariants:**
- Rules spatially/contextually bound
- No global rules by default
- Hierarchical voting (one level up)
- Transitions have warnings + grace periods
- No retroactive enforcement

**Governed by:** [c13.RuleBasedZones]

**Related:** Building, ForceUnit, Session, Vote

---

## 📊 OBJECT COUNT

**Total objects defined:** 59

**By category:**
- Identity & Access: 10
- Coordination & Work: 6
- Physical World: 12
- Cognitive & AI: 8
- Verification & Audit: 6
- Conflict & Damage: 2
- System Objects: 5
- Genesis Objects: 2

---

## ✅ USAGE NOTES

**When implementing:**
1. Check "Governed by" for architecture dependencies
2. Verify all invariants enforced
3. Check "Related" for integration points
4. Reference "What it is NOT" to avoid anti-patterns

**When auditing:**
1. Check all invariants hold
2. Verify no forbidden mutations
3. Trace governing architecture
4. Check evidence links

**When documenting:**
1. Reference objects by name (not description)
2. Link to this reference
3. Quote invariants when relevant

---

**Refs:** ALL architecture commits (c0-c16)  
**Objects:** ALL  
**Audit:** [Completeness], [Consistency], [Traceability]

**END OF OBJECTS REFERENCE**

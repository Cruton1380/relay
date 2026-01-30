# MASTER PROMPT FILAMENT SPECIFICATION

**Purpose:** Canonical construction protocol for Relay  
**Last Updated:** 2026-01-29  
**Status:** 🔒 Locked (Genesis Object)

---

## 🎯 WHAT THIS IS

**MasterPromptFilament is Relay's Genesis Block.**

**It preserves:**
- How Relay was constructed (step by step)
- Which invariants were discovered (and when)
- What order constraints were locked
- How to faithfully reconstruct Relay

**This is not documentation. This is causal history.**

---

## 🧬 OBJECT DEFINITION

```rust
/// The genesis record of Relay - preserves construction history
struct MasterPromptFilament {
    /// Unique identifier
    filament_id: String,  // "relay/master_prompt"
    
    /// When construction began
    created_at: Timestamp,
    
    /// Human author(s)
    authors: Vec<String>,
    
    /// Canonical purpose
    purpose: String,
    
    /// Language and abstraction level assumptions
    language_level: LanguageLevel,
    
    /// Ordered construction stages
    stages: Vec<ConstructionStage>,
    
    /// Reference to final navigation structure
    toc_ref: TocReference,
    
    /// How to faithfully reconstruct
    replay_instructions: String,
    
    /// Audit requirements
    audit_assertions: Vec<AuditAssertionType>,
}
```

---

## 📐 LANGUAGE BASELINE (LOCKED)

```rust
struct LanguageBaseline {
    /// Natural language used
    natural_language: String,  // "English"
    
    /// Reasoning complexity assumed
    abstraction_level: String,  // "High-complexity systems reasoning"
    
    /// Assumed reader capabilities
    assumed_reader: String,  // "Technical + governance-literate"
    
    /// Explicit primitive assumptions
    assumed_primitives: Vec<String>,
    
    /// Target AI capability
    target_capability: String,  // "GPT-4 / Claude Sonnet class or better"
    
    /// Forbidden reinterpretations
    forbidden_reinterpretations: Vec<String>,
    
    /// Allowed metaphors (only when mapped to physics)
    allowed_metaphors: Vec<String>,
}
```

**Assumed Primitives (Explicit):**
- Graph theory (DAGs, dependencies)
- Physics metaphors (conservation, causality)
- Append-only logs
- Event sourcing
- Deterministic replay
- Audit trails
- StarCraft game mechanics

**Forbidden Reinterpretations:**
- ❌ "Platform" (Relay is world model, not platform)
- ❌ "App" (Relay is coordination reality, not application)
- ❌ "Dashboard" (Relay is physics engine, not visualization)
- ❌ "Score" (Relay has no global scores/rankings)
- ❌ "User interface" (Relay is truth layer, UI is projection)
- ❌ "Database" (Relay is filament substrate, not CRUD)
- ❌ "Blockchain" (Relay uses append-only logs, not distributed consensus)

**Allowed Metaphors:**
- ✅ StarCraft economics (maps to coordination physics)
- ✅ Physics (conservation, causality, determinism)
- ✅ Immune system (audit as continuous verification)
- ✅ Civilization infrastructure (long-term viability)

**Critical Rule:**
> If a simplification removes physics constraints, it is forbidden.

**Why this matters:**
- Future LLMs may not share these primitives
- Explicit listing prevents semantic drift
- Prevents "Relay Lite" from accidentally emerging
- Different cultures/AI classes must preserve baseline
- Enables translation to new AI systems without simplification

---

## 🔄 CONSTRUCTION STAGE

```rust
struct ConstructionStage {
    /// Stage identifier
    stage_id: String,
    
    /// Order in sequence
    sequence_num: u32,
    
    /// Input prompt (verbatim)
    input_prompt: String,
    
    /// Response (verbatim or summarized)
    response_summary: String,
    
    /// Full response reference
    response_ref: Option<ArtifactRef>,
    
    /// Invariants discovered in this stage
    invariants_discovered: Vec<String>,
    
    /// Constraints locked
    constraints_locked: Vec<String>,
    
    /// Architecture commits produced
    architecture_refs: Vec<String>,
    
    /// Dependencies on prior stages
    depends_on: Vec<String>,
    
    /// Ideas explicitly discarded (and why)
    discarded_ideas: Vec<DiscardedIdea>,
    
    /// Context at this stage
    context_note: Option<String>,
}
```

---

## 📋 STAGES (21 TOTAL)

### **Stage 001: Foundation & Filaments**
**Input:** "Design append-only coordination substrate"  
**Invariants Discovered:**
- Filaments never mutate, only extend
- Filaments conserve value and history
- Zoom reveals spacing, not new semantics

**Architecture Produced:** c0 (Filament Physics)

---

### **Stage 002: Commit Semantics**
**Input:** "Define commit execution model"  
**Invariants Discovered:**
- Commits are execution, not tooling
- Commits require causal_refs
- No hidden operations

**Architecture Produced:** c1 (Commit Semantics)

---

### **Stage 003: Replayability**
**Input:** "Ensure deterministic replay"  
**Invariants Discovered:**
- Same commits → same state (always)
- No randomness in state derivation
- Timestamps are inputs, not mutators

**Architecture Produced:** c2 (Replayability)

---

### **Stage 004: Layer Separation**
**Input:** "Separate truth from rendering"  
**Invariants Discovered:**
- Layer 2 (Relay Physics) = truth
- Layer 3 (Frontend) = rendering
- Clear separation prevents UI-driven truth

**Architecture Produced:** c3 (Truth Substrate)

---

### **Stage 005: RenderSpec v1**
**Input:** "Define deterministic rendering contract"  
**Invariants Discovered:**
- Deterministic rendering (no randomness)
- Semantic materials (not RGB)
- Stable IDs
- No time-dependent geometry

**Architecture Produced:** c5 (RenderSpec v1)

---

### **Stage 006: Economic Primitives**
**Input:** "Define resource and coordination primitives"  
**Invariants Discovered:**
- Money = Authority Tokens
- Voting = Attention Signals
- Reputation = Merit
- Commitments = Promises

**Architecture Produced:** c6 (Economic Primitives)

---

### **Stage 007: StarCraft HUD Model**
**Input:** "Apply StarCraft economics to coordination"  
**Invariants Discovered:**
- HUD = personal constraints
- Globe = shared physical world
- 5 canonical gauges (orthogonal)
- Shopping = unit production

**Architecture Produced:** c7 (Coordination Gauges), c9 (StarCraft Model)

---

### **Stage 008: Delegated Influence**
**Input:** "Define voting as force lending"  
**Invariants Discovered:**
- Voting ≠ truth production
- Influence decays over time
- Votes revocable
- Scope-bound (not global)

**Architecture Produced:** c8 (Delegated Decaying Influence)

---

### **Stage 009: Ontological Foundation**
**Input:** "What are users? What is identity?"  
**Breakthrough:** Users ARE filament trees, identity IS a filament  
**Sacred Invariant:** No filament may collapse into scalar  
**Invariants Discovered:**
- Buildings ARE tiles (accumulate history)
- Buildings ARE units (tradable, governable)
- Proximity channels ubiquitous

**Architecture Produced:** c10 (Ontological Foundation)

---

### **Stage 010: Root AI Coherence**
**Input:** "How do multiple AI agents coordinate?"  
**Invariants Discovered:**
- Root AI maintains unbranched logic
- Specialized agents (3-5 max)
- Agents consult Root, not each other
- Root = coherence building

**Architecture Produced:** c11 (Root AI Coherence)

---

### **Stage 011: Cognitive Substrate**
**Input:** "How do humans steer AI transparently?"  
**Invariants Discovered:**
- Conversations ARE filaments
- Training IS deterministic compilation
- Steering IS explicit branch selection
- Learning IS append-only artifacts
- No hidden AI learning

**Architecture Produced:** c12 (Root AI Cognitive Substrate)

---

### **Stage 012: Rule-Based Zones**
**Input:** "How do laws apply spatially?"  
**Invariants Discovered:**
- Rules spatially/contextually bound
- No global rules by default
- Hierarchical voting (one level up)
- Enforcement modes explicit
- No retroactive enforcement

**Architecture Produced:** c13 (Rule-Based Zones)

---

### **Stage 013: Presence & Sessions**
**Input:** "How do humans interact in shared spaces?"  
**Invariants Discovered:**
- Presence = progressive disclosure
- Sessions = first-class spatial objects
- Triple binding (physical + cognitive + graphics)
- No forced identity exposure

**Architecture Produced:** c14 (Presence, Sessions, Co-Located Play)

---

### **Stage 014: Global Conflict**
**Input:** "How are military assets rendered?"  
**Invariants Discovered:**
- All critical force objects MUST exist
- LOD governs detail, not existence
- Lenses govern visibility
- No hidden force
- No invisible damage

**Architecture Produced:** c15 (Global Conflict & LOD Rendering)

---

### **Stage 015: Universal Audit**
**Input:** "How is reality verified?"  
**Invariants Discovered:**
- Audit = continuous immune system
- MaterialityRules define reality requirements
- Assertions require evidence
- Findings are scars (never deleted)
- No self-certification

**Architecture Produced:** c16 (Universal Audit & Assurance)

---

### **Stage 016-018: Communication & AI Modules**
**Input:** "How do languages align? How are prompts tracked?"  
**Concepts Developed:**
- MeaningFrame (language-neutral intent)
- TranslationBranch (semantic alignment)
- DialogContextBundle (paired dialogs)
- PromptBounceChain (agent hops)

**(Not yet locked as architecture commits; integration pending)**

---

### **Stage 019: Meta-Locks**
**Input:** "What final constraints prevent drift?"  
**10 Meta-Locks:**
1. No platform thinking
2. No global scores
3. No hidden defaults
4. Text ≠ truth
5. AI = proposals only
6. No copy-paste
7. Zones > convenience
8. Single LOD function
9. Accessibility as gate
10. Hierarchy exposed

---

### **Stage 020: Legibility at Scale**
**Input:** "How do humans navigate this system?"  
**Invariants Discovered:**
- CONTEXT-TABLE.json (machine-readable)
- Cross-link rules (no orphan claims)
- Reader paths (structured onboarding)

---

### **Stage 021: Genesis Block**
**Input:** "How do we preserve construction itself?"  
**Result:** MasterPromptFilament + MasterConversationFilament  
**Purpose:** Enable faithful reconstruction

---

## 🔁 REPLAY INSTRUCTIONS

### **If Relay Is Lost and Must Be Rebuilt:**

**1. Language Baseline:**
- Use English
- Assume high-complexity systems reasoning
- Target: GPT-4 / Claude Sonnet class LLM or better
- Verify assumed primitives still valid

**2. Replay Sequence:**
- Load `master_prompt.jsonl`
- Replay stages in order (001 → 021)
- **Do NOT skip stages**
- **Do NOT reinterpret** - preserve verbatim intent

**3. Verification Per Stage:**
- After each stage, verify invariants discovered match
- Check no contradictions introduced
- Ensure architecture commits produced match
- Validate dependencies satisfied

**4. Final Verification:**
- Generated CONTEXT-TABLE.json checksum must match
- All 17 architecture commits (c0-c16) must exist
- All meta-locks must be enforced
- No orphan documents
- All cross-links resolve

**5. Failure Modes:**
- **If language assumptions invalid:**  
  → Update LanguageLevel, replay from start
  
- **If invariants contradict:**  
  → **DO NOT PROCEED**, human intervention required
  
- **If checksum mismatch:**  
  → **DO NOT PROCEED**, corruption detected

**This protocol is non-negotiable. Semantic drift = system failure.**

---

## 📊 TOC REFERENCE

```rust
struct TocReference {
    /// Path to CONTEXT-TABLE.json
    path: String,  // "reference/CONTEXT-TABLE.json"
    
    /// SHA-256 hash for integrity
    checksum: String,
    
    /// Version
    version: String,  // "1.0.0"
}
```

**Checksum Calculation:**
```bash
sha256sum reference/CONTEXT-TABLE.json
```

**Checksum must match on reconstruction.**

---

## 🔍 AUDIT ASSERTIONS

### **Required Assertions:**

**1. Existence:**
- MasterPromptFilament exists and is complete
- All 21 stages documented

**2. Completeness:**
- All construction stages recorded
- No missing invariant discoveries
- All discarded ideas documented
- All architecture commits linked

**3. Temporal Integrity:**
- Stages ordered correctly (001-021)
- Dependencies preserved
- No retroactive edits
- Timestamps authentic

**4. Traceability:**
- Every architecture commit traces to stage
- Every invariant traces to discovery moment
- Every discarded idea has reason

**5. Replayability:**
- Following replay instructions produces equivalent Relay
- Language assumptions still valid
- No interpretation required

**6. Consistency:**
- No contradictions between stages
- Final TOC matches stage outputs
- Architecture dependency graph matches

---

## 🚫 FORBIDDEN OPERATIONS

**This filament must NEVER:**
- ❌ Be edited retroactively
- ❌ Have stages reordered
- ❌ Hide discarded ideas
- ❌ Summarize away critical details
- ❌ Be "cleaned up" for readability

**This filament may:**
- ✅ Be extended (append new stages if Relay evolves)
- ✅ Be referenced (by any document)
- ✅ Be audited (verify integrity)

---

## 📍 FILE STRUCTURE

```
relay/
└── filaments/
    └── master_prompt/
        ├── master_prompt.jsonl (append-only log)
        ├── stages/
        │   ├── 001_foundation.md
        │   ├── 002_commits.md
        │   ├── ... (all stages)
        │   └── 021_genesis_block.md
        └── README.md (human-readable guide)
```

---

## 🔗 RELATIONSHIPS

**MasterPromptFilament is the root of:**
- All architecture filaments (c0-c16)
- All invariants
- All meta-locks
- CONTEXT-TABLE.json

**MasterPromptFilament depends on:**
- Nothing (it is foundational)

---

## ✅ VERIFICATION CHECKLIST

**Before marking MasterPromptFilament complete:**
- [ ] All 21 stages documented
- [ ] All invariants traced to discovery
- [ ] All architecture commits linked
- [ ] All discarded ideas recorded
- [ ] Replay instructions complete
- [ ] TOC reference with checksum
- [ ] Audit assertions defined
- [ ] No retroactive edits
- [ ] Verbatim prompts preserved

---

**Refs:** ALL architecture commits (c0-c16)  
**Objects:** [MasterPromptFilament], [ConstructionStage]  
**Audit:** [Existence], [Completeness], [Temporal Integrity], [Replayability]

**END OF MASTER PROMPT SPECIFICATION**

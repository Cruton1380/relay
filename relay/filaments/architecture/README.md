# Architecture Filament

**Filament ID:** `architecture`  
**Purpose:** Track architectural decisions as immutable, traceable commits  
**Created:** 2026-01-28

---

## WHAT THIS IS

This filament is the **canonical record** of Relay's architectural decisions. Each commit is:

1. **Immutable** - Never edited, only superseded by new commits
2. **Traceable** - References prior commits (causal chain)
3. **Versioned** - Commit index provides timeline
4. **Auditable** - Shows "why" and "what changed"

---

## WHY THIS EXISTS

**Problem:** Architecture decisions scattered across chat/docs → drift + misalignment

**Solution:** Treat architecture as **coordination truth** (just like code commits)

**Benefits:**
- ✅ Stops architectural drift
- ✅ Makes future Claude sessions efficient (can read canonical commits)
- ✅ Demonstrates Relay's own use case (meta-coordination)
- ✅ Creates accountability/traceability for decisions

---

## FILAMENT STRUCTURE

Each commit is a markdown file: `0000_title.md`, `0001_title.md`, etc.

**Format:**
```markdown
# COMMIT N: Title

**Filament:** architecture
**Commit Index:** N
**Date:** YYYY-MM-DD
**Author:** system.architect
**Type:** ARCH_DECISION | API_CONTRACT | DESIGN_LOCK | ROADMAP_LOCK

---

## INVARIANT LOCKED

[One-sentence summary of what's being locked]

---

## DECISION

[Details...]

---

## CAUSAL REFS

- **Inputs:** [Prior commits this depends on]
- **Authority:** [Who approved this]
- **Evidence:** [PRs, docs, proofs]

---

**Status:** LOCKED | SUPERSEDED | DRAFT
```

---

## CURRENT COMMITS

### 0000: Render Responsibility Split
**Locks:** Layer 2 outputs data, Layer 3 renders pixels, Layer 1 is git-only  
**Status:** LOCKED

### 0001: Render Endpoints Contract
**Locks:** 4 endpoint families (events SSE, commits, units, render scenes)  
**Status:** LOCKED

### 0002: SSE Truth Stream Locks
**Locks:** No phantom events, truth stream completeness, deterministic replay  
**Status:** LOCKED

### 0003: RenderSpec v1 Stub
**Locks:** JSON schema format (stub, to be completed after PR #1.3)  
**Status:** DRAFT (stub only)

### 0004: Execution Order Decision
**Locks:** PR #1.3 → RenderSpec v1 → /render/* endpoints → Layer 3 frontend  
**Status:** LOCKED

### 0005: RenderSpec v1 Locked
**Locks:** JSON scene graph format (polyline filaments, semantic materials, deterministic)  
**Status:** LOCKED

### 0006: Economic Primitives Foundation
**Locks:** Money=transferable authority; Credit=visible delegation; Commitment≠Payment  
**Status:** LOCKED

### 0007: Coordination Gauges (StarCraft HUD Model)
**Locks:** Multiple orthogonal gauges (legitimacy, commitments, escrow, time, disputes)  
**Status:** LOCKED

### 0008: Delegated Decaying Influence (Fourth Primitive)
**Locks:** Authority is borrowed continuously; Influence decays; Voting as relationship  
**Status:** LOCKED

### 0009: Personal HUD + Physical Globe (StarCraft Model)
**Locks:** HUD=personal (my state); Globe=shared (world map); Shopping=production; Buildings=physical  
**Status:** LOCKED

### 0010: Ontological Foundation (Users as Filament Trees)
**Locks:** Users ARE filament trees (not accounts); Identity IS a filament; Buildings are space tiles; Buildings are units; Proximity channels everywhere; Sacred invariant: No scalar collapse  
**Status:** LOCKED

### 0011: Root AI Coherence Layer
**Locks:** Root AI maintains unbranched logic; Specialized agents (3-5 max) consult Root constantly; Agents talk through Root, not peer-to-peer; Root is a building; Root's guidance is append-only; Prevents agent drift, copy/paste hell, coordination collapse  
**Status:** LOCKED

### 0012: Root AI Cognitive Substrate
**Locks:** Conversations ARE filaments; Agent training IS deterministic compilation (Training Packs); Steering IS explicit branch selection (Logic Branches + Profiles); Learning IS append-only artifacts; Root HUD = Compiler/Router/Auditor; Profiles + Gates + Traces > raw weights; Header/proof zoom layers; Gates enforce physics  
**Status:** LOCKED

### 0013: Rule-Based Zones (Geographic + Contextual Constraint Regions)
**Locks:** Rules spatially + contextually bound to explicit zones; No global rules; Territory → ConstraintRegion; Boundaries → Explicit transitions; Enforcement → Structural modes; Hierarchical voting (one level up); Transition warnings + grace periods; No retroactive/discretionary enforcement; Zones are first-class physics objects  
**Status:** LOCKED

### 0014: Presence, Sessions, and Co-Located Play (Social Layer)
**Locks:** Presence as progressive identity disclosure (privacy ladder); Sessions as first-class spatial objects; Triple binding (physical + cognitive + graphics); Building beacons anchor world shards; Local + remote share cognitive space; Visitor arrival protocol; Public visibility ≠ identity exposure; Consent required for participation  
**Status:** LOCKED

### 0015: Global Conflict & LOD Reality Rendering
**Locks:** All critical force objects must exist as ForceUnits; LOD governs detail, not existence; Lenses govern visibility; Military assets → StarCraft units; Damage → DamageScar objects; No hidden force; No retroactive movement; No invisible damage; Early escalation detection; Legibility prevents catastrophe  
**Status:** LOCKED

### 0016: Universal Audit & Assurance Layer (Relay GAPP / NIST Equivalent)
**Locks:** Audit as immune system; Continuous (not periodic); Universal (all domains); Independent (provable); MaterialityRules define reality requirements; RealityCoverageMap detects blind spots; Assertions require evidence; Findings are scars; Opinions are scoped; No self-certification; No silent omission; No global scores; GAAP/IFRS/NIST/ISO equivalents  
**Status:** LOCKED

---

## HOW TO USE THIS

### For Development
1. **Read commits 0-4** to understand current architecture
2. **Propose new commit** if architecture needs to change
3. **Reference commit index** when discussing decisions (e.g., "per architecture@c2...")

### For Future Claude Sessions
1. **Read this README** for overview
2. **Read relevant commits** for specific decisions
3. **Never contradict locked commits** (propose superseding commit instead)

### For Testing/Validation
1. **Commits are assertions** - Test code should align with them
2. **Invariants are contracts** - Violations are bugs
3. **Causal refs are dependencies** - Breaking them requires new commit

---

## EVOLUTION RULES

### To Modify a Locked Decision
1. **Never edit existing commit** (immutable)
2. **Create new commit** that references old one
3. **Mark old commit as SUPERSEDED**
4. **Update dependent commits** if needed

### To Add New Decision
1. **Create new file** with next commit index
2. **Follow format above**
3. **Reference causal inputs**
4. **Update README** with summary

### To Propose Alternative
1. **Create branch commit** (e.g., `0004a_alternative.md`)
2. **Mark as DRAFT**
3. **Lock one alternative** and mark others REJECTED

---

## FUTURE INTEGRATION

**When Layer 2 is fully operational:**
- This markdown filament → JSONL format (`architecture.jsonl`)
- Each line = `{commitIndex, timestamp, author, title, patch, invariants}`
- Ingestible by Relay tooling (same as code commits)

**For now:** Markdown is canonical (easier to read/edit)

---

## QUESTIONS TO ASK CLAUDE

1. **Is this commit consistent with prior commits?**
2. **Does this decision create technical debt?**
3. **Are the invariants testable?**
4. **What's the rollback plan if this decision is wrong?**

---

**Filament Status:** ACTIVE  
**Next Commit:** (TBD - depends on PR #1.3 outcomes)  
**Maintainer:** system.architect (bootstrap phase)

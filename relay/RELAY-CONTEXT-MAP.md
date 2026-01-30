# RELAY CONTEXT MAP

**Purpose:** Entry point for all Relay documentation  
**Last Updated:** 2026-01-29  
**Status:** 🔒 Locked

---

## 🎯 START HERE

**If you don't know where to start, read this page.**  
**This is the map, not the territory.**

---

## 📍 WHAT IS RELAY?

**In 100 words:**  
Relay is a physics engine for coordination where filaments conserve history, buildings anchor capabilities, and zones constrain behavior. Work is filament advancement. Authority is delegated and decaying. Truth is auditable. Text is projection. Military assets are rendered. Conflicts are legible. Presence is progressive. AI proposes, never asserts. Audit runs continuously. Everything references something real. Nothing collapses into global scores. This is StarCraft economics applied to reality: constraints visible, power explicit, actions costly, history immutable.

**Read full:** `summaries/RELAY-IN-100-WORDS.md`

---

## 🗺️ WHAT EXISTS IN RELAY

### **Core Concepts:**
- **Filaments:** Append-only history (time dimension)
- **Buildings:** Spatial anchors with capabilities (space dimension)
- **Zones:** Constraint regions with rules
- **Commits:** Execution events (no silent operations)
- **Scars:** Permanent failure records
- **Lenses:** Visibility filters (LOD, conflict, audit)

### **Major Object Categories:**
- **Identity:** IdentityFilament, PresenceTier, AccessDecision
- **Coordination:** Task, Shipment, Session, Vote
- **Physical:** Building, ForceUnit, Zone
- **Cognitive:** MeaningFrame, TranslationBranch, DialogContextBundle
- **Verification:** AuditAssertion, MaterialityRule, AuditFinding
- **AI:** SCV, TrainingPack, LogicBranch, WeightProfile

**Read full catalog:** `reference/RELAY-OBJECTS-REFERENCE.md`

---

## 📚 HOW TO READ RELAY DOCUMENTS

**Critical understanding:**
1. **Text is never source of truth** → Filaments are truth, text is projection
2. **Zoom levels exist** → Layer 0 (headers), Layer 1 (proof), Layer 2 (evidence)
3. **No copy-paste** → Select, hold, bind (references, not duplicates)
4. **Everything traces** → Follow commit_ref, filament_ref, architecture commits
5. **Docs are layered** → Context → Physics → Projections → Reference → Implementation

**Read full guide:** `reference/HOW-TO-READ-RELAY.md`

---

## 🎯 WHICH DOCUMENT TO READ (BY ROLE)

### **👔 I am an Employee**
**Intent:** Understand how to work in Relay  
**Start:** `guides/WORK-IN-RELAY.md`  
**Time:** 30 minutes  
**You'll learn:** Work = filament commits, career = identity tree growth, pay flows to filaments, SCVs = force multipliers

---

### **👨‍💼 I am a Manager**
**Intent:** Understand scope holding and authority  
**Start:** `guides/SCOPE-AND-AUTHORITY-IN-RELAY.md`  
**Time:** 60 minutes  
**You'll learn:** Managers = scope holders, authority = delegated/temporary, power = voluntary attraction, hierarchy exposed not eliminated

---

### **📊 I am an Auditor**
**Intent:** Perform Relay audit (GAAP/IFRS/NIST background)  
**Start:** `AUDITOR-GUIDE-RELAY-GAPP.md`  
**Time:** 2 hours  
**You'll learn:** Relay GAPP (8 principles), NIST mapping, audit procedures, continuous audit, how to issue opinions

---

### **💻 I am a Developer**
**Intent:** Implement Relay features correctly  
**Start:** `guides/RELAY-DEVELOPER-GUIDE.md`  
**Time:** Half day  
**You'll learn:** Architecture dependencies, object schemas, forbidden patterns, testing requirements, PR roadmap

---

### **🏛️ I am a Policy Maker**
**Intent:** Understand governance and rule-making  
**Start:** `guides/GOVERNANCE-IN-RELAY.md`  
**Time:** 60 minutes  
**You'll learn:** Voting = delegated influence, zones = rule regions, hierarchical voting, no global authority

---

### **🎖️ I am a Defense Analyst**
**Intent:** Understand military asset tracking and conflict detection  
**Start:** `guides/MILITARY-AND-CONFLICT-IN-RELAY.md`  
**Time:** 90 minutes  
**You'll learn:** ForceUnits, LOD system, lenses, escalation detection, damage accountability

---

### **🤖 I am an AI Operator**
**Intent:** Work with Root AI and SCVs  
**Start:** `guides/AI-COLLABORATION-IN-RELAY.md`  
**Time:** 60 minutes  
**You'll learn:** Root HUD, specialized agents, training packs, dialog bundles, bounce tracking, gates

---

### **🔬 I want to Understand Architecture**
**Intent:** Deep dive into Relay physics  
**Start:** `architecture/ARCHITECTURE-INDEX.md`  
**Time:** 2-3 hours  
**You'll learn:** All 17 architecture commits (c0-c16), invariants, dependencies, what each enables/forbids

---

### **📖 I want Object Definitions**
**Intent:** Look up specific object schemas  
**Start:** `reference/RELAY-OBJECTS-REFERENCE.md`  
**Time:** As needed (reference)  
**You'll learn:** Canonical definitions, fields, mutations, invariants, governing commits

---

### **⚡ I want Quick Overview**
**Intent:** 5-minute understanding  
**Start:** `summaries/RELAY-IN-100-WORDS.md` → This page  
**Time:** 5 minutes  
**You'll learn:** High-level concepts, where to go deeper

---

## 🛤️ READER PATHS (MANUAL-STYLE SEQUENCES)

### **Path 1: Employee Onboarding (30 minutes)**
**Intent:** Understand how to work in Relay

**Sequence:**
1. `reference/HOW-TO-READ-RELAY.md` (5 min) → Zoom, hold/bind, filaments
2. `guides/WORK-IN-RELAY.md` (15 min) → What work means, advancement
3. `reference/RELAY-OBJECTS-REFERENCE.md#Task` (5 min) → Task details
4. `reference/RELAY-OBJECTS-REFERENCE.md#SCV` (5 min) → SCV force multipliers

**Stop condition:** Can create task, bind SCV, understand victory = dependency

---

### **Path 2: Manager Onboarding (60 minutes)**
**Intent:** Understand scope holding and authority

**Sequence:**
1. `reference/HOW-TO-READ-RELAY.md` (5 min)
2. `guides/SCOPE-AND-AUTHORITY-IN-RELAY.md` (20 min) → Manager role
3. `architecture/ARCHITECTURE-INDEX.md#c13` (10 min) → Rule-based zones
4. `architecture/ARCHITECTURE-INDEX.md#c8` (10 min) → Delegated influence
5. `reference/RELAY-OBJECTS-REFERENCE.md#Zone` (10 min)
6. `reference/RELAY-OBJECTS-REFERENCE.md#DelegatedInfluence` (5 min)

**Stop condition:** Can define scope, set constraints, resolve conflicts

---

### **Path 3: Auditor Certification (2 hours)**
**Intent:** Perform Relay audit

**Sequence:**
1. `reference/HOW-TO-READ-RELAY.md` (5 min)
2. `AUDITOR-GUIDE-RELAY-GAPP.md` (60 min) → Full audit process
3. `architecture/ARCHITECTURE-INDEX.md#c16` (20 min) → Universal audit layer
4. `reference/RELAY-OBJECTS-REFERENCE.md#AuditAssertion` (10 min)
5. `reference/RELAY-OBJECTS-REFERENCE.md#MaterialityRule` (10 min)
6. `reference/TRADITIONAL-STANDARDS-MAPPING.md` (15 min) → GAAP/NIST

**Stop condition:** Can define scope, test assertions, issue opinion

---

### **Path 4: Developer Implementation (Half day)**
**Intent:** Build Relay features correctly

**Sequence:**
1. `reference/HOW-TO-READ-RELAY.md` (5 min)
2. `guides/RELAY-DEVELOPER-GUIDE.md` (30 min)
3. `architecture/ARCHITECTURE-INDEX.md` (45 min) → Skim all c0-c16
4. `reference/RELAY-OBJECTS-REFERENCE.md` (60 min) → Deep dive objects
5. `implementation/PR-ROADMAP.md` (30 min)
6. Choose specific PR spec (60 min)

**Stop condition:** Can implement feature without violating physics

---

### **Path 5: Quick Reference (5 minutes)**
**Intent:** Understand Relay at high level

**Sequence:**
1. `summaries/RELAY-IN-100-WORDS.md` (1 min) → Elevator pitch
2. This page (2 min) → Navigation overview
3. `architecture/ARCHITECTURE-INDEX.md` (2 min) → Skim titles only

**Stop condition:** Know what Relay is, where to go deeper

---

## 📐 DOCUMENT LAYERS (STRUCTURE)

### **Layer 1: Context (Entry Points)**
- `RELAY-CONTEXT-MAP.md` ← You are here
- `summaries/RELAY-IN-100-WORDS.md`

### **Layer 2: Physics (Architecture)**
- `architecture/ARCHITECTURE-INDEX.md`
- `architecture/filaments/*.md` (c0-c16 specs)

### **Layer 3: Projections (Domain Guides)**
- `guides/WORK-IN-RELAY.md`
- `guides/SCOPE-AND-AUTHORITY-IN-RELAY.md`
- `AUDITOR-GUIDE-RELAY-GAPP.md`
- `guides/RELAY-DEVELOPER-GUIDE.md`
- `guides/GOVERNANCE-IN-RELAY.md`
- `guides/MILITARY-AND-CONFLICT-IN-RELAY.md`
- `guides/AI-COLLABORATION-IN-RELAY.md`

### **Layer 4: Reference (Technical)**
- `reference/RELAY-OBJECTS-REFERENCE.md`
- `reference/HOW-TO-READ-RELAY.md`
- `reference/RELAY-GAPP-SPECIFICATION.md`
- `reference/RELAY-NIST-CONTROLS.md`
- `reference/TRADITIONAL-STANDARDS-MAPPING.md`
- `reference/CONTEXT-TABLE.json` (machine-readable)

### **Layer 5: Implementation (Specs)**
- `implementation/PR-ROADMAP.md`
- `implementation/PR-*.md` (specific PRs)

---

## 🚫 READING ANTI-PATTERNS

**Don't:**
- ❌ Read linearly (start → end)
- ❌ Read everything (pick your path)
- ❌ Skip HOW-TO-READ-RELAY.md (critical for understanding)
- ❌ Trust narrative without refs (check cross-links)
- ❌ Assume text = truth (filaments = truth)

**Do:**
- ✅ Start with your role/intent
- ✅ Follow reader path for your audience
- ✅ Check cross-link refs
- ✅ Zoom into physics when needed
- ✅ Trace dependencies via CONTEXT-TABLE.json

---

## 🔗 CROSS-LINK VALIDATION

**All domain guides MUST:**
- Reference architecture commits (cX)
- Reference object definitions
- Reference audit assertion types (if material)

**Format:**
```markdown
---
**Refs:** [c0.Filaments], [c7.Tasks]  
**Objects:** [Task], [Commit]  
**Audit:** [Completeness], [Traceability]
```

**This prevents:** Orphan claims, narrative drift.

---

## 📊 NAVIGATION SHORTCUTS

**By question:**
- "How do I work here?" → `guides/WORK-IN-RELAY.md`
- "How do I audit this?" → `AUDITOR-GUIDE-RELAY-GAPP.md`
- "How do I build this?" → `guides/RELAY-DEVELOPER-GUIDE.md`
- "What is object X?" → `reference/RELAY-OBJECTS-REFERENCE.md`
- "What is architecture cX?" → `architecture/ARCHITECTURE-INDEX.md`
- "What are the invariants?" → `architecture/ARCHITECTURE-INDEX.md`
- "How do zones work?" → `architecture/filaments/0013_rule_based_zones.md`
- "How does audit work?" → `architecture/filaments/0016_universal_audit_assurance.md`

---

## 🎯 SYSTEM STATUS

**Architecture commits:** 17 (c0-c16)  
**Total invariants:** 107  
**Documentation files:** 25+  
**Implementation PRs:** 57+  
**Realm conversions:** 17/20 (85%)

**All physics locked. All systems integrated. All enforcement mechanisms active.**

---

## ✅ VERIFICATION

**Can you:**
- ✅ Find where to start in <30s? (This page)
- ✅ Trace any claim to physics? (Cross-links + CONTEXT-TABLE)
- ✅ Look up any object? (RELAY-OBJECTS-REFERENCE)
- ✅ Understand your role? (Domain guides)
- ✅ Navigate without getting lost? (Reader paths)

**If yes to all: system is legible at scale.**

---

## 🚀 NEXT STEPS

**After reading your entry document:**
1. Follow your Reader Path
2. Reference RELAY-OBJECTS as needed
3. Check ARCHITECTURE-INDEX when confused
4. Trace dependencies via cross-links

**For machine navigation:**
- Use `reference/CONTEXT-TABLE.json`
- Build HUD navigation (future)

---

**This is the map. Follow your path. Relay is navigable.**

**END OF CONTEXT MAP**

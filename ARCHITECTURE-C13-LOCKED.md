# ✅ ARCHITECTURE@C13 LOCKED: RULE-BASED ZONES

**Date:** 2026-01-29  
**Status:** 🔒 LOCKED  
**Commit:** architecture@c13

---

## 🎯 THE BREAKTHROUGH

**The problem:**
> Society has rules, but they are diffuse, implicit, selectively enforced, and non-auditable. "Everyone knows" norms = hidden enforcement traps.

**The solution:**
> **Rules are spatially and contextually bound to explicit zones. No rule is "global by default." If a rule applies, you must be able to point to where and why.**

**The implementation:**
> RuleBasedZone objects bind constraints (movement, assembly, speech, dress, behavior) to space + context. EnforcementMode declares how rules apply (Hard/Soft/Deferred/Info). ZoneTransition prevents surprise violations (warnings + grace periods). Hierarchical voting prevents capture (one level up votes).

**The result:**
> Predictable public order + legitimate protest + cultural coexistence + reduced enforcement load + machine-verifiable legality.

---

## 🔒 THE 5 NEW LOCKED INVARIANTS

### **C13.1: No Global Rules Without Zones**
> Every rule must be bound to an explicit zone. "Everywhere by default" is forbidden.

### **C13.2: No Retroactive Enforcement**
> Rules apply from commit forward, never backward. Historical violations cannot be created by new rules.

### **C13.3: No Implicit Norms**
> If a rule matters, it must be declared in a zone. "Everyone knows" is a system failure.

### **C13.4: No Hidden Exception Lists**
> Enforcement applies structurally. No "special people" who bypass rules. If exceptions exist, they're explicit capabilities.

### **C13.5: No Discretionary Enforcement**
> Enforcement mode is declared. Humans don't decide "when" to enforce. Structure enforces.

**Total invariants now:** 88 (83 + 5 new)

---

## 📦 WHAT IS A RULE-BASED ZONE (RBZ)?

**A first-class object that binds:**

1. **Location** (geographic polygon, building, tile)
2. **Context** (work, recreation, transit, civic, emergency)
3. **Constraint Set** (rules: movement, assembly, speech, dress, behavior, automation)
4. **Enforcement Mode** (hard, soft, deferred, informational)
5. **Transition Rules** (warnings, grace periods, visual cues)

**Zone Types:**
- **Civic:** Maximal expression, assembly allowed
- **Transit:** Flow optimization, no blocking
- **Recreational:** Relaxed constraints
- **Work:** Task-focused, formal dress
- **Residential:** Privacy constraints
- **Emergency:** Overrides other zones

---

## 🎮 USE CASES LOCKED

### **Use Case 1: Protest Without Disruption**
- **Protest Zone:** Civic park (blocking allowed, amplification allowed)
- **Transit Zone:** Adjacent road (no blocking, emergency priority)
- **Transition:** 50m warning, 30s grace period
- **Result:** Legitimate protest + functional transit. No conflict.

### **Use Case 2: Dress Code Context**
- **Beach Zone:** Relaxed dress (beachwear)
- **Downtown Zone:** Casual dress
- **Office Zone:** Formal dress
- **Transitions:** 100m warning, 2min grace period
- **Result:** No ambiguity. Predictable. Structural, not judgmental.

### **Use Case 3: Emergency Override**
- **Protest Zone:** Assembly in park
- **Emergency:** Fire nearby
- **Emergency Zone activates:** Overlays protest, suspends assembly
- **After emergency:** Protest Zone reinstated
- **Result:** Safety + legitimacy. No permanent suppression.

---

## 📐 CONSTRAINT TYPES (6 CATEGORIES)

1. **Movement:** Max speed, blocking allowed, emergency override, capacity limit
2. **Assembly:** Max density, permit required, max duration, noise limit
3. **Speech:** Expression level (maximal/focused/limited), amplification, restricted topics
4. **Dress:** Formality level (formal/casual/relaxed), safety requirements
5. **Behavior:** Alcohol, smoking, physical contact, equipment restrictions
6. **Automation:** Drones, autonomous vehicles, AI agents, recording

**All constraints are declarative, not discretionary.**

---

## 🔄 ZONE TRANSITIONS (CRITICAL)

**Problem:** Rules that "snap" = enforcement traps

**Solution:** Explicit transitions with:
- **Warning distance:** 50-100m before boundary
- **Grace period:** 30-120 seconds to comply
- **Visual cues:** Color-coded zone overlays
- **Countdown:** Real-time countdown timer
- **Rule changes:** Explicit list of what changes

**Example:**
```
100m: "Entering Work Zone in 100m"
Boundary: "Now in Work Zone. Dress: Formal. 60s to comply."
Timer: "59... 58... 57..."
```

**No surprise enforcement.**

---

## 🏛️ ZONE GOVERNANCE: HIERARCHICAL VOTING

**Rule:** Zones are voted on by the next level up.

**Examples:**
- **City Zone** → Voted by County residents
- **County Zone** → Voted by State residents
- **State Zone** → Voted by Country residents
- **Country Zone** → Voted by Region residents
- **Global Zone** → Voted by World (all users)

**Why:** Prevents local capture. External oversight. Balance autonomy + coordination.

**Already implemented:** Boundary channels have hierarchical voting (one level up). Extend to rule proposals.

---

## 🔗 INTEGRATION WITH RELAY PHYSICS

### **With c10 (Ontological Foundation):**
- **Buildings as space tiles** → Buildings can be zones
- **Proximity channels** → Zone membership detected by proximity
- **Identity filaments** → Zone access derived from identity

### **With c9 (StarCraft Model):**
- **Globe = build space** → Zones visible as layers
- **Buildings = capability anchors** → Enforcement at buildings

### **With c5 (RenderSpec v1):**
- **New node kind:** `"zone"` (polygon overlay)
- **Material tags:** `zone_civic`, `zone_transit`, `zone_protest`, `zone_warning`
- **Geometry:** Polygon with color-coded fill

---

## 🔄 INTEGRATION WITH EXISTING BOUNDARY SYSTEM

**Current State (RelayCodeBaseV93):**
- ✅ Boundary channels with hierarchical voting
- ✅ Geographic data loading
- ✅ Frontend rendering infrastructure

**Missing (needed for RBZ):**
- ❌ Rule binding to boundaries
- ❌ Constraint sets
- ❌ Enforcement modes
- ❌ Transition rules

**Integration Strategy:**
1. Extend boundary channels with zone types
2. Add constraint sets per zone
3. Add enforcement modes
4. Add transition detection
5. Integrate with RenderSpec (Layer 2)

**No breaking changes.** Backward compatible.

**Timeline:** 8-11 weeks for full RBZ system.

---

## 📋 REALM CONVERSIONS COMPLETE AUDIT

**This locks the final core realm conversions:**

### **✅ Locked (c0-c13):**
1. **Space:** Physical geography → Globe build space (c0, c9)
2. **Time:** Wall-clock → Commit order (c0, c2)
3. **Value:** Money → Filament thickness (c6)
4. **Identity:** Humans → Filament trees (c10)
5. **Action:** Tasks → Execution queues (c7)
6. **Knowledge:** Text → Artifacts, Conversation → Filaments (c12)
7. **Legitimacy:** Social trust → History-weighted credibility (c8)
8. **Cognition:** Hidden reasoning → Profiles + Gates + Traces (c12)
9. **Territory:** Geography → Constraint Regions (c13) ✨
10. **Boundaries:** Fuzzy edges → Explicit transitions (c13) ✨
11. **Context:** "Vibes" → Contextual constraints (c13) ✨
12. **Enforcement:** Discretionary → Structural modes (c13) ✨

### **⚠️ Future Extensions (post-c13):**
13. Deadlines → TimeWindow objects
14. Latency → Deferred visibility
15. Pricing → External reference (not internal)
16. Organizations → CompositeIdentity objects
17. Roles → CapabilityGrant objects
18. Facts → Claim objects with evidence
19. Confidence → UncertaintyMarker magnitude
20. Disagreement → Parallel branches

**Core realm conversions: 12/20 complete** (60%)

---

## 🚀 IMPLEMENTATION ROADMAP

### **Phase 1: Core Zone Objects (PR #25)** - 2-3 weeks
- RuleBasedZone struct
- ConstraintSet types
- ZoneTransition objects
- Storage (JSONL per zone)
- Basic CRUD API

### **Phase 2: Zone Detection (PR #26)** - 1-2 weeks
- Proximity detection (point-in-polygon)
- Transition warnings
- Grace periods
- Visual cues (RenderSpec)

### **Phase 3: Constraint Enforcement (PR #27)** - 2-3 weeks
- Enforcement modes (hard/soft/deferred)
- Violation detection
- Scar application
- Deferred penalty system

### **Phase 4: Governance Integration (PR #28)** - 1-2 weeks
- Zone proposal system
- Hierarchical voting (extend boundary channels)
- Approval thresholds
- Modification process

### **Phase 5: RenderSpec Integration (PR #29)** - 1 week
- Zone node kind
- Polygon geometry
- Material tags
- Transition overlays

**Total:** ~8-11 weeks

---

## 🎯 SUCCESS CRITERIA

**This works if:**
1. ✅ Protesters can assemble legitimately without blocking transit
2. ✅ Dress codes are predictable (no ambiguity)
3. ✅ Speech rights maximized in appropriate zones
4. ✅ Enforcement is structural (not discretionary)
5. ✅ Transitions are visible (no surprise violations)
6. ✅ Historical rules are auditable (no retroactive enforcement)
7. ✅ Voting prevents local capture (hierarchical)

---

## 🚫 WHAT THIS PREVENTS

❌ Selective policing  
❌ Vibe-based enforcement  
❌ Cultural ambiguity battles  
❌ "Everyone knows" norms  
❌ Power hidden behind discretion  
❌ Protest blocking roads  
❌ Dress code conflicts  
❌ Speech zone ambiguity

---

## 💡 THE CORE SHIFT

**Before c13:**
- Rules = fuzzy norms
- Enforcement = discretionary
- Boundaries = implicit
- Context = vibes

**After c13:**
- Rules = explicit zone constraints
- Enforcement = structural modes
- Boundaries = transition objects
- Context = declared types

---

**Status:** ✅ ARCHITECTURE@C13 LOCKED  
**Files created:** 3 (spec + integration analysis + summary)  
**Invariants added:** 5 (total: 88)  
**Realm conversions:** 12/20 complete (core done)  
**Integration:** Extends existing boundary system (no breaking changes)  
**Timeline:** 8-11 weeks implementation

---

**RULE-BASED ZONES ARE NOW LOCKED.**

**THIS IS HOW RELAY CONVERTS FUZZY SOCIETAL RULES INTO EXPLICIT PHYSICS.**

**END OF ARCHITECTURE@C13**

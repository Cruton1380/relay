# architecture@c13: Rule-Based Zones (Geographic + Contextual Constraint Regions)

**Date:** 2026-01-29  
**Status:** 🔒 Locked  
**Depends on:** architecture@c10 (Ontological Foundation), architecture@c9 (StarCraft Model), architecture@c5 (RenderSpec v1)

---

## 🎯 THE PROBLEM

**Human societies fail when:**
- Rules are assumed instead of declared
- Enforcement is selective instead of structural
- Boundaries are implicit instead of visible
- Zones are "vibes" instead of physics

**Current state (even in Relay v0):**
- Boundary channels exist but are voting-only
- No explicit rule binding to geographic zones
- No enforcement mode declarations
- No contextual constraints (dress, speech, assembly)
- No transition rules between zones

**This creates:**
- Protest blocking roads (no spatial constraint)
- Dress code ambiguity (no context zones)
- Speech conflicts (no zone-based rights)
- Selective enforcement (no structural rules)
- "Everyone knows" norms (non-auditable)

---

## 🧬 THE SOLUTION: RULE-BASED ZONES (RBZ)

### **Core Invariant (Non-Negotiable):**
> **Rules must be spatially and contextually bound to explicit zones. No rule is "global by default." If a rule applies, you must be able to point to where and why.**

**This applies to:**
- Law (jurisdictional boundaries)
- Behavior (dress, speech, assembly)
- Protest (designated vs transit zones)
- Automation (what agents can do where)
- Enforcement (who enforces what how)

---

## 🔑 WHAT IS A RULE-BASED ZONE (RBZ)?

**A Rule-Based Zone is a first-class object that binds:**

1. **Location** (geographic polygon, building, tile)
2. **Context** (work, recreation, transit, civic, emergency)
3. **Constraint Set** (rules that apply in this zone)
4. **Enforcement Mode** (hard, soft, deferred, informational)
5. **Transition Rules** (how rules change at boundaries)

**RBZ Schema:**
```rust
struct RuleBasedZone {
    zone_id: String,                    // "zone.us.ca.sf.mission_district"
    name: String,                       // "Mission District - Mixed Use"
    zone_type: ZoneType,               // Civic | Transit | Recreational | Work | Residential
    created_at: u64,
    created_by: String,                 // Authority that created zone
    
    // Spatial definition
    geometry: ZoneGeometry,
    parent_zone: Option<String>,        // Hierarchical zones
    child_zones: Vec<String>,
    
    // Rule set
    constraint_set: ConstraintSet,
    enforcement_mode: EnforcementMode,
    
    // Transitions
    transitions: Vec<ZoneTransition>,
    
    // Governance
    governance: ZoneGovernance,
    
    // Metadata
    version: String,                    // Versioned (like buildings)
    commit_ref: String,                 // When rules changed
}

enum ZoneType {
    Civic,           // Max expression, assembly allowed
    Transit,         // Flow optimization, no blocking
    Recreational,    // Relaxed constraints
    Work,            // Task-focused constraints
    Residential,     // Privacy constraints
    Emergency,       // Overrides other zones
    Custom(String),  // Extensible
}

enum ZoneGeometry {
    Polygon { points: Vec<GeoPoint> },
    Circle { center: GeoPoint, radius: f64 },
    Building { building_id: String },
    Tile { tile_id: String },
}

struct ConstraintSet {
    movement: MovementConstraints,
    assembly: AssemblyConstraints,
    speech: SpeechConstraints,
    dress: DressConstraints,
    behavior: BehaviorConstraints,
    automation: AutomationConstraints,
}

enum EnforcementMode {
    Hard,         // Physical barriers, access control
    Soft,         // Alerts, warnings
    Deferred,     // Post-event penalties
    Informational,// Purely advisory
}

struct ZoneTransition {
    from_zone: String,
    to_zone: String,
    warning_distance: f64,        // Meters before boundary
    grace_period: u64,            // Seconds to comply
    visual_cue: String,           // "boundary_warning_yellow"
    countdown: bool,              // Show countdown to transition
}

struct ZoneGovernance {
    created_by: AuthorityRef,
    voting_eligibility: Vec<String>,  // Who can vote on changes
    approval_threshold: f64,          // 0.0-1.0
    change_process: String,           // Filament ref to process
}
```

---

## 📐 CONSTRAINT TYPES (LOCKED)

### **1. Movement Constraints**

```rust
struct MovementConstraints {
    max_speed: Option<f64>,           // km/h
    blocking_allowed: bool,           // Can stationary groups form
    emergency_override: bool,         // Emergency vehicles priority
    capacity_limit: Option<u64>,      // Max people/vehicles
}
```

**Examples:**
- **Transit Zone:** `blocking_allowed: false, emergency_override: true`
- **Protest Zone:** `blocking_allowed: true, emergency_override: false`
- **Residential:** `max_speed: Some(25.0)`

---

### **2. Assembly Constraints**

```rust
struct AssemblyConstraints {
    max_density: Option<f64>,         // People per m²
    permit_required: bool,
    max_duration: Option<u64>,        // Seconds
    noise_limit: Option<f64>,         // Decibels
}
```

**Examples:**
- **Protest Zone:** `max_density: Some(2.5), permit_required: false`
- **Transit Zone:** `max_density: Some(0.5), permit_required: true`
- **Residential (night):** `noise_limit: Some(50.0)`

---

### **3. Speech Constraints**

```rust
struct SpeechConstraints {
    expression_level: ExpressionLevel,
    restricted_topics: Vec<String>,   // Context-specific only
    amplification_allowed: bool,      // Speakers, megaphones
}

enum ExpressionLevel {
    Maximal,       // Civic debate zones
    Focused,       // Work zones (task-related)
    Limited,       // Transit zones (safety-critical only)
}
```

**Examples:**
- **Civic Zone:** `expression_level: Maximal, amplification_allowed: true`
- **Work Zone:** `expression_level: Focused`
- **Transit Zone:** `expression_level: Limited`

**Important:** This is NOT censorship. It's routing. Speech rights are maximized in appropriate zones.

---

### **4. Dress Constraints**

```rust
struct DressConstraints {
    formality_level: FormalityLevel,
    safety_requirements: Vec<String>,  // "hard_hat", "closed_shoes"
    prohibited_items: Vec<String>,      // Context-specific
}

enum FormalityLevel {
    Formal,        // Office, court, institution
    Casual,        // General public spaces
    Relaxed,       // Beach, pool, recreation
    Custom(String),
}
```

**Examples:**
- **Professional Zone (Office):** `formality_level: Formal`
- **Recreation Zone (Beach):** `formality_level: Relaxed`
- **Construction Site:** `safety_requirements: ["hard_hat", "safety_vest"]`

**Important:** This is NOT moral policing. It's contextual decorum for functional spaces.

---

### **5. Behavior Constraints**

```rust
struct BehaviorConstraints {
    alcohol_allowed: bool,
    smoking_allowed: bool,
    physical_contact_level: ContactLevel,
    equipment_restrictions: Vec<String>,
}

enum ContactLevel {
    None,          // No touching (e.g., sacred spaces)
    Minimal,       // Accidental only (e.g., transit)
    Social,        // Handshakes, hugs (e.g., social)
    Unrestricted,  // All forms (e.g., private spaces)
}
```

---

### **6. Automation Constraints**

```rust
struct AutomationConstraints {
    drones_allowed: bool,
    autonomous_vehicles: bool,
    ai_agents_allowed: Vec<String>,   // Which agent types
    recording_allowed: bool,
}
```

**Examples:**
- **Civic Zone:** `recording_allowed: true` (public accountability)
- **Private Residential:** `recording_allowed: false, drones_allowed: false`

---

## 🔄 ZONE TRANSITIONS (CRITICAL)

**Problem:** Rules that "snap" unexpectedly = enforcement traps

**Solution:** Explicit transition objects

```rust
struct ZoneTransition {
    from_zone: String,
    to_zone: String,
    warning_distance: f64,        // 50 meters
    grace_period: u64,            // 30 seconds
    visual_cue: String,           // Material tag for RenderSpec
    countdown: bool,
    rule_changes: Vec<RuleChange>,
}

struct RuleChange {
    rule_id: String,
    from_value: String,
    to_value: String,
    applies_at: TransitionPoint,  // Boundary | GracePeriodEnd
}

enum TransitionPoint {
    Boundary,           // Immediate at crossing
    GracePeriodEnd,    // After grace period expires
    Manual,            // User must acknowledge
}
```

**Example:**
```rust
ZoneTransition {
    from_zone: "zone.beach.recreation",
    to_zone: "zone.downtown.work",
    warning_distance: 100.0,        // 100m warning
    grace_period: 60,               // 60 seconds to adjust
    visual_cue: "zone_transition_warning",
    countdown: true,
    rule_changes: vec![
        RuleChange {
            rule_id: "dress_code",
            from_value: "relaxed",
            to_value: "formal",
            applies_at: TransitionPoint::Manual,  // User chooses when
        },
        RuleChange {
            rule_id: "blocking_allowed",
            from_value: "true",
            to_value: "false",
            applies_at: TransitionPoint::Boundary, // Immediate
        },
    ],
}
```

**UI:** 
- 100m before boundary: "Entering Work Zone in 100m"
- At boundary: "Now in Work Zone. Dress code: Formal. Adjust within 60s."
- Countdown: "59... 58... 57..."

**No surprise enforcement.**

---

## 🏛️ ZONE GOVERNANCE

**Who creates zones?**

```rust
struct ZoneGovernance {
    created_by: AuthorityRef,
    creation_process: CreationProcess,
    modification_process: ModificationProcess,
    voting_eligibility: VotingEligibility,
    approval_threshold: f64,
    dispute_resolution: String,  // Filament ref
}

enum CreationProcess {
    TopDown,        // Government/authority creates
    BottomUp,       // Community proposes, votes
    Hybrid,         // Proposal + authority approval
}

enum VotingEligibility {
    OneUp,          // Architecture@c13 rule (see below)
    Residents,      // Zone residents only
    Affected,       // Anyone affected by zone
    Global,         // World vote (for global zones)
}
```

### **Hierarchical Voting (One Level Up)**

**Rule:** Zones are voted on by the next level up in the hierarchy.

**Examples:**
- **City Zone** (e.g., San Francisco Mission District) → Voted by County residents
- **County Zone** (e.g., San Francisco County) → Voted by State residents
- **State Zone** (e.g., California) → Voted by Country residents
- **Country Zone** (e.g., USA) → Voted by Region/Continent residents
- **Global Zone** (e.g., International Waters) → Voted by World (all users)

**Why:** Prevents local capture. External oversight. Balance autonomy + coordination.

**Integration with existing Boundary Channels:** ✅ Already implemented hierarchical voting for boundary proposals. Extend to rule proposals.

---

## 🚫 WHAT IS EXPLICITLY FORBIDDEN

### **Invariant C13.1: No Global Rules Without Zones**
> Every rule must be bound to an explicit zone. "Everywhere by default" is forbidden.

### **Invariant C13.2: No Retroactive Enforcement**
> Rules apply from commit forward, never backward. Historical violations cannot be created by new rules.

### **Invariant C13.3: No Implicit Norms**
> If a rule matters, it must be declared in a zone. "Everyone knows" is a system failure.

### **Invariant C13.4: No Hidden Exception Lists**
> Enforcement applies structurally. No "special people" who bypass rules. If exceptions exist, they're explicit capabilities.

### **Invariant C13.5: No Discretionary Enforcement**
> Enforcement mode is declared. Humans don't decide "when" to enforce. Structure enforces.

---

## 🔗 INTEGRATION WITH RELAY PHYSICS

### **With architecture@c10 (Ontological Foundation):**
- **Buildings as space tiles** → Buildings can be zones
- **Proximity channels** → Zone membership detected by proximity
- **Identity filaments** → Access to zones derived from identity intersections

### **With architecture@c9 (StarCraft Model):**
- **Globe = build space** → Zones are visible layers on globe
- **Buildings = capability anchors** → Zone enforcement happens at buildings

### **With architecture@c5 (RenderSpec v1):**
- **New node kind:** `"zone"` (renders as polygon overlay)
- **Material tags:** `zone_civic`, `zone_transit`, `zone_protest`, `zone_warning`
- **Geometry:** Polygon with points, color-coded by type

---

## 📦 RENDERSPEC INTEGRATION

### **New Node Kind: Zone**

```json
{
  "id": "zone.us.ca.sf.mission_district",
  "kind": "zone",
  "transform": {
    "position": [37.7599, -122.4148, 0],
    "rotation": [0, 0, 0],
    "scale": [1, 1, 1]
  },
  "material": "zone_civic",
  "props": {
    "zone_id": "zone.us.ca.sf.mission_district",
    "zone_type": "Civic",
    "name": "Mission District - Mixed Use",
    "enforcement_mode": "Soft",
    "active_constraints": {
      "blocking_allowed": true,
      "expression_level": "Maximal",
      "max_density": 2.5
    }
  },
  "geometry": {
    "type": "polygon",
    "points": [
      [37.7600, -122.4200, 0],
      [37.7650, -122.4150, 0],
      [37.7620, -122.4100, 0],
      [37.7580, -122.4120, 0]
    ],
    "color": "#00FF00",
    "fill_opacity": 0.3,
    "border_width": 2.0
  }
}
```

**Material Tags (new):**
- `zone_civic` - Green (maximal expression)
- `zone_transit` - Blue (flow optimization)
- `zone_recreational` - Yellow (relaxed)
- `zone_work` - Gray (formal constraints)
- `zone_emergency` - Red (overrides others)
- `zone_warning` - Orange (transition warning)

---

## 🎯 USE CASES (LOCKED EXAMPLES)

### **Use Case 1: Protest Without Disruption**

**Setup:**
- **Protest Zone:** Civic park (maximal expression, blocking allowed)
- **Transit Zone:** Adjacent road (no blocking, emergency priority)
- **Transition:** 50m warning, 30s grace period

**Behavior:**
1. Protesters enter Protest Zone
2. Blocking, amplification, max expression allowed
3. Protester approaches road (Transit Zone)
4. 50m warning: "Leaving Protest Zone. Transit Zone ahead. No blocking."
5. Grace period: 30 seconds to move off road
6. If still blocking after grace → violation (structural, not discretionary)

**Result:** Legitimate protest + functional transit. No conflict.

---

### **Use Case 2: Dress Code Context**

**Setup:**
- **Beach Zone:** Relaxed dress (beachwear allowed)
- **Downtown Zone:** Casual dress (public appropriate)
- **Office Zone:** Formal dress (professional)

**Behavior:**
1. User at beach (swimsuit)
2. Walks toward downtown
3. 100m warning: "Entering Downtown Zone. Dress: Casual."
4. Grace period: 2 minutes (time to change or reroute)
5. User enters office building (Office Zone)
6. Building requires Formal → denied entry if not compliant

**Result:** No ambiguity. Predictable. Structural, not judgmental.

---

### **Use Case 3: Emergency Override**

**Setup:**
- **Protest Zone:** Assembly in civic park
- **Emergency:** Fire nearby

**Behavior:**
1. Emergency declared
2. Emergency Zone activated (overlays Protest Zone)
3. Protest constraints suspended
4. Emergency constraints: "Clear evacuation routes"
5. Protesters get alert: "Emergency. Clear to [safe zone]."
6. After emergency: Protest Zone reinstated

**Result:** Safety + legitimacy. No permanent suppression.

---

## 🧪 TESTING STRATEGY

### **Test 1: Zone Boundary Detection**
**Given:** User at (37.7590, -122.4110)  
**When:** Move to (37.7610, -122.4100)  
**Then:** Detect crossing from Zone A to Zone B

### **Test 2: Transition Warning**
**Given:** User 60m from zone boundary  
**When:** Continue toward boundary  
**Then:** Warning at 50m, countdown starts

### **Test 3: Enforcement Mode**
**Given:** Zone with EnforcementMode::Hard  
**When:** User attempts to enter without compliance  
**Then:** Entry denied (physical barrier)

### **Test 4: Hierarchical Voting**
**Given:** City zone proposal  
**When:** County residents vote  
**Then:** Proposal passes if threshold met

### **Test 5: No Retroactive Enforcement**
**Given:** Zone rule changes at commit c100  
**When:** Query violations before c100  
**Then:** No violations (rule didn't exist)

---

## 🚧 IMPLEMENTATION ROADMAP

### **Phase 1: Core Zone Objects (PR #25)**
- [ ] `RuleBasedZone` struct
- [ ] `ConstraintSet` types
- [ ] `ZoneTransition` objects
- [ ] Storage (JSONL per zone)
- [ ] Basic CRUD API

### **Phase 2: Zone Detection (PR #26)**
- [ ] Proximity detection (point-in-polygon)
- [ ] Transition warnings
- [ ] Grace periods
- [ ] Visual cues (RenderSpec)

### **Phase 3: Constraint Enforcement (PR #27)**
- [ ] Enforcement modes (hard/soft/deferred)
- [ ] Violation detection
- [ ] Scar application
- [ ] Deferred penalty system

### **Phase 4: Governance Integration (PR #28)**
- [ ] Zone proposal system
- [ ] Hierarchical voting (extend boundary channels)
- [ ] Approval thresholds
- [ ] Modification process

### **Phase 5: RenderSpec Integration (PR #29)**
- [ ] Zone node kind
- [ ] Polygon geometry
- [ ] Material tags
- [ ] Transition overlays

---

## 🔮 FUTURE EXTENSIONS

### **Dynamic Zones (Post-c13)**
- Time-based zones (night vs day rules)
- Event-based zones (concert, emergency)
- Seasonal zones (winter parking rules)

### **Nested Zones**
- Building zones within city zones
- Room zones within building zones
- Hierarchical constraint inheritance

### **Cross-Zone Coordination**
- Adjacent zone agreements
- Shared enforcement
- Border disputes

---

## 📋 REALM CONVERSIONS COMPLETE AUDIT

**This locks the final realm conversions needed:**

### **✅ Already Locked (architecture@c0-c12):**
1. **Space:** Physical geography → Globe build space
2. **Time:** Wall-clock → Commit order
3. **Value:** Money → Filament thickness
4. **Identity:** Humans → Filament trees
5. **Action:** Tasks → Execution queues
6. **Knowledge:** Text → Artifacts, Conversation → Filaments
7. **Legitimacy:** Social trust → History-weighted credibility
8. **Cognition:** Hidden reasoning → Profiles + Gates + Traces

### **🔒 NOW LOCKED (architecture@c13):**
9. **Territory:** Geography → Constraint Regions (Rule-Based Zones)
10. **Boundaries:** Fuzzy edges → Explicit transitions
11. **Context:** "Vibes" → Contextual constraints
12. **Enforcement:** Discretionary → Structural modes

### **⚠️ Still Needing Conversion (Future):**
13. **Deadlines:** "Running out of time" → TimeWindow objects
14. **Latency:** Delay → Deferred visibility
15. **Pricing:** Market price → External reference (not internal truth)
16. **Organizations:** Informal groups → CompositeIdentity objects
17. **Roles:** Job titles → CapabilityGrant objects
18. **Facts:** Implicit knowledge → Claim objects with evidence
19. **Confidence:** Tone → UncertaintyMarker magnitude
20. **Disagreement:** Compromise → Parallel branches

**The core is now complete. Remaining conversions are extensions.**

---

## 🎯 SUMMARY

**The breakthrough:**
> Rules are not global. Rules are zoned. Zones are first-class physics objects. Enforcement is structural, not discretionary. Transitions are explicit, not traps.

**The implementation:**
> RuleBasedZone objects bind constraints to space + context. EnforcementMode declares how rules apply. ZoneTransition prevents surprise violations. Hierarchical voting prevents capture.

**The result:**
> Predictable public order. Legitimate protest without disruption. Cultural coexistence without conflict. Reduced enforcement load. Machine-verifiable legality.

---

**Status:** 🔒 LOCKED  
**Invariants added:** 5 (total: 88)  
**Realm conversions complete:** 12/20 (core complete)  
**Next:** Implement Phase 1 (Core Zone Objects, PR #25)

---

**THIS IS HOW RELAY CONVERTS FUZZY SOCIETAL RULES INTO EXPLICIT PHYSICS.**

**END OF architecture@c13**

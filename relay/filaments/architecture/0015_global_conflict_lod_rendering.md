# architecture@c15: Global Conflict & LOD Reality Rendering

**Date:** 2026-01-29  
**Status:** 🔒 Locked  
**Depends on:** architecture@c9 (StarCraft Model), architecture@c13 (Rule-Based Zones), architecture@c10 (Ontological Foundation), architecture@c5 (RenderSpec v1)

---

## 🎯 THE PROBLEM

**Current state of global conflict:**
- Force is hidden (submarines, strategic weapons)
- Buildup is deniable ("training exercises")
- Escalation is opaque (slow boil → surprise war)
- Damage is contested (casualty counts, infrastructure destruction)
- Power concentration is invisible (who controls what)

**Result:**
- Wars start because no one sees the buildup
- Escalation becomes unstoppable
- Civilian populations caught unaware
- Post-hoc narratives replace real-time truth

**The core failure:**
> Critical power objects—battleships, fighter jets, missile systems, military bases—exist in reality but not in our coordination systems. They are kept deliberately invisible.

---

## 🧬 THE SOLUTION: ALL CRITICAL OBJECTS MUST BE RENDERABLE

### **Core Invariant (Non-Negotiable):**
> **Any object capable of exerting large-scale force, irreversible damage, or geopolitical impact MUST exist as a first-class entity in the Relay world and be renderable at appropriate LOD (Level of Detail).**

**Corollary:**
> If it can change the future for millions of people, it cannot remain invisible.

**This applies to:**
- Military assets (naval, air, ground)
- Strategic weapons platforms
- Military bases and logistics hubs
- Command-and-control infrastructure
- Force mobilization and movement
- Damage to buildings and infrastructure

---

## 🔑 WHAT IS A FORCE UNIT?

**A ForceUnit is a first-class object, exactly like a Building or Task, but representing real-world military/power assets.**

**Force Unit Schema:**
```rust
struct ForceUnit {
    unit_id: String,                    // "force.us.navy.cvn_78.uss_ford"
    unit_class: ForceClass,            // Battleship | Fighter | Tank | Missile | Drone
    operator: String,                   // Nation/organization operating it
    created_at: u64,
    
    // Spatial definition
    current_position: GeoPosition,
    last_known_position: Option<GeoPosition>,
    movement_vector: Option<MovementVector>,
    anchor: Option<ForceAnchor>,       // Home base / carrier group
    
    // Status
    status: ForceStatus,
    capability_class: CapabilityClass,  // Strategic | Tactical | Support
    operational_state: OperationalState,// Active | Standby | Maintenance | Damaged
    
    // LOD visibility
    lod_policy: LODPolicy,
    lenses: Vec<String>,               // Which lenses expose this unit
    
    // Governance
    disclosure_tier: DisclosureTier,   // Public | Authorized | Restricted
    verification_refs: Vec<String>,    // Who verified this exists
    
    // History
    commits: Vec<String>,              // Movement, status changes, damage
    scars: Vec<DamageScar>,            // Damage inflicted/received
    
    // Metadata
    version: String,
    commit_ref: String,                // When last updated
}

enum ForceClass {
    // Naval
    AircraftCarrier,
    Battleship,
    Destroyer,
    Submarine,
    
    // Air
    FighterJet,
    Bomber,
    Drone,
    Helicopter,
    
    // Ground
    Tank,
    ArmoredVehicle,
    MissileLauncher,
    Artillery,
    
    // Strategic
    NuclearSilo,
    MissileBase,
    CommandCenter,
    
    // Support
    LogisticsHub,
    FuelDepot,
    RepairFacility,
    
    // Extensible
    Custom(String),
}

enum CapabilityClass {
    Strategic,      // Can change global balance (nukes, carriers)
    Tactical,       // Can change regional balance (fighters, destroyers)
    Support,        // Enables operations (logistics, fuel)
}

enum OperationalState {
    Active,         // Deployed, operational
    Standby,        // Ready but not deployed
    Maintenance,    // Undergoing repairs
    Damaged,        // Operational but compromised
    Destroyed,      // Non-functional
    Decommissioned, // Permanently retired
}

struct MovementVector {
    heading: f64,           // Degrees
    speed: f64,             // km/h
    destination: Option<GeoPosition>,
    eta: Option<u64>,
    route: Option<Vec<GeoPosition>>,
}

struct ForceAnchor {
    anchor_id: String,      // "base.us.norfolk" or "carrier_group.cvn_78"
    anchor_type: AnchorType,// Base | CarrierGroup | FleetFormation
}

enum AnchorType {
    MilitaryBase,
    NavalBase,
    AirBase,
    CarrierGroup,
    FleetFormation,
    TaskForce,
}
```

---

## 📐 LOD (LEVEL OF DETAIL) RENDERING

**Problem:** Full transparency of all military assets = security risk.

**Solution:** LOD-based visibility governed by lenses and permissions.

### **LOD Levels:**

```rust
enum LODLevel {
    LOD0,  // Abstract presence (exists, approximate region)
    LOD1,  // Unit class + broad region (public awareness)
    LOD2,  // Movement vectors + status (authorized lens)
    LOD3,  // Full detail (restricted / governance / verification)
}

struct LODPolicy {
    default_lod: LODLevel,
    lens_overrides: HashMap<String, LODLevel>,  // Lens → LOD
    zone_overrides: HashMap<String, LODLevel>,  // Zone → LOD
    viewer_overrides: HashMap<String, LODLevel>,// Viewer → LOD (authority)
}
```

### **LOD Examples:**

**LOD 0 (Abstract Presence):**
```json
{
  "unit_id": "force.us.navy.unknown_001",
  "unit_class": "Naval",
  "region": "Western Pacific",
  "capability_class": "Strategic",
  "operational_state": "Active"
}
```
**Renders as:** Blue dot on globe, labeled "Naval Asset (Strategic)"

---

**LOD 1 (Unit Class + Region):**
```json
{
  "unit_id": "force.us.navy.cvn_78",
  "unit_class": "AircraftCarrier",
  "operator": "US Navy",
  "region": "South China Sea",
  "capability_class": "Strategic",
  "operational_state": "Active"
}
```
**Renders as:** Carrier icon, region circle (not exact position)

---

**LOD 2 (Movement + Status):**
```json
{
  "unit_id": "force.us.navy.cvn_78",
  "unit_class": "AircraftCarrier",
  "operator": "US Navy",
  "current_position": {"lat": 15.0, "lon": 110.0},
  "movement_vector": {
    "heading": 90.0,
    "speed": 25.0
  },
  "operational_state": "Active",
  "last_updated": 1706486400000
}
```
**Renders as:** Carrier model at position, movement arrow, speed label

---

**LOD 3 (Full Detail):**
```json
{
  "unit_id": "force.us.navy.cvn_78.uss_ford",
  "unit_class": "AircraftCarrier",
  "operator": "US Navy",
  "current_position": {"lat": 15.1234, "lon": 110.5678, "alt": 0},
  "movement_vector": {
    "heading": 90.0,
    "speed": 25.0,
    "destination": {"lat": 18.0, "lon": 115.0},
    "eta": 1706572800000
  },
  "anchor": {
    "anchor_id": "carrier_group.cvn_78",
    "anchor_type": "CarrierGroup"
  },
  "operational_state": "Active",
  "commits": ["commit_ref_001", "commit_ref_002"],
  "scars": []
}
```
**Renders as:** Full carrier model, escort ships, exact position, route line, ETA

---

## 🔍 LENSES (CRITICAL)

**Problem:** Same world, different perspectives needed (civilian vs analyst vs military vs governance).

**Solution:** Lenses are filters that adjust LOD, visibility, and context.

### **Conflict & Security Lenses:**

```rust
struct Lens {
    lens_id: String,                   // "lens.global_conflict_state"
    name: String,                      // "Global Conflict State"
    description: String,
    
    // What this lens shows
    visible_units: Vec<ForceClass>,    // Which unit types
    lod_override: HashMap<ForceClass, LODLevel>,
    zone_filter: Option<Vec<String>>, // Only show in certain zones
    
    // Computation
    aggregation_rules: Vec<String>,    // How to summarize
    alert_thresholds: Vec<String>,     // When to highlight escalation
    
    // Access control
    public: bool,
    required_authority: Option<String>,
}
```

### **Example Lenses:**

**1. Global Conflict State Lens**
```rust
Lens {
    lens_id: "lens.global_conflict_state",
    name: "Global Conflict State",
    description: "High-level view of military mobilization and conflict zones",
    visible_units: vec![
        ForceClass::AircraftCarrier,
        ForceClass::Battleship,
        ForceClass::Bomber,
        ForceClass::NuclearSilo,
    ],
    lod_override: {
        ForceClass::Strategic => LODLevel::LOD1,
        ForceClass::Tactical => LODLevel::LOD0,
    },
    public: true,
}
```
**Shows:** Strategic assets at LOD1 (class + region), tactical at LOD0 (presence only)

---

**2. Force Mobilization Lens**
```rust
Lens {
    lens_id: "lens.force_mobilization",
    name: "Force Mobilization",
    description: "Detects unusual concentration or movement of forces",
    visible_units: vec![ALL],
    lod_override: {
        * => LODLevel::LOD1,
    },
    aggregation_rules: vec![
        "detect_buildup",      // Concentration exceeds baseline
        "detect_convergence",  // Multiple units moving to same area
        "detect_logistics_surge", // Support units increase
    ],
    alert_thresholds: vec![
        "buildup > 2x baseline",
        "convergence_time < 72h",
    ],
    public: true,
}
```
**Shows:** Aggregated force concentration, movement trends, alerts on anomalies

---

**3. Escalation Risk Lens**
```rust
Lens {
    lens_id: "lens.escalation_risk",
    name: "Escalation Risk",
    description: "Identifies patterns indicating potential conflict",
    visible_units: vec![ForceClass::Strategic, ForceClass::Tactical],
    lod_override: { * => LODLevel::LOD2 },
    aggregation_rules: vec![
        "proximity_to_borders",
        "mutual_buildup",         // Two sides building up simultaneously
        "strategic_asset_movement",
        "communication_blackouts",
    ],
    alert_thresholds: vec![
        "mutual_buildup in same region",
        "strategic_assets within 500km of border",
    ],
    public: true,
    required_authority: Some("verified_analyst"),
}
```
**Shows:** Risk scores, proximity warnings, escalation indicators

---

**4. Civilian Impact Lens**
```rust
Lens {
    lens_id: "lens.civilian_impact",
    name: "Civilian Impact",
    description: "Shows damage to buildings, infrastructure, and civilian areas",
    visible_units: vec![ForceClass::All],  // All units that can cause damage
    lod_override: { * => LODLevel::LOD1 },
    aggregation_rules: vec![
        "proximity_to_cities",
        "damage_scars_on_buildings",
        "refugee_flows",           // Future: population movement
    ],
    zone_filter: Some(vec!["zone.civilian", "zone.residential"]),
    public: true,
}
```
**Shows:** Units near civilian zones, damage to buildings, impact visualization

---

## 🏗️ FORCE ANCHORS (BASES & FORMATIONS)

**ForceAnchor objects represent:**
- Military bases (naval, air, ground)
- Carrier groups
- Fleet formations
- Task forces

```rust
struct ForceAnchor {
    anchor_id: String,              // "base.us.norfolk"
    name: String,                   // "Naval Station Norfolk"
    anchor_type: AnchorType,
    location: GeoPosition,
    
    // Composition
    assigned_units: Vec<String>,    // ForceUnit IDs
    capacity: u64,                  // Max units
    
    // Status
    operational_state: OperationalState,
    scars: Vec<DamageScar>,        // Damage to base
    
    // Governance
    operator: String,               // Nation/organization
    zone: String,                   // Which RBZ (from c13)
    disclosure_tier: DisclosureTier,
    
    // History
    commits: Vec<String>,
    version: String,
    commit_ref: String,
}
```

**Integration with Buildings (c9, c10):**
- Military bases ARE buildings with special properties
- Bases accumulate structures, functions, history (like all buildings)
- Bases can be damaged (scars)
- Bases have zones (c13) that define access/constraints

---

## 💥 DAMAGE SCARS (CONFLICT OUTCOMES)

**DamageScar objects represent irreversible damage events.**

```rust
struct DamageScar {
    scar_id: String,
    timestamp: u64,
    
    // What was damaged
    target_id: String,              // Building or ForceUnit
    target_type: String,            // "building" | "force_unit"
    
    // What caused damage
    source_id: Option<String>,      // ForceUnit that caused damage (if known)
    event_type: DamageType,
    
    // Extent
    severity: ScarSeverity,
    description: String,
    
    // Verification
    verified_by: Vec<String>,       // Who verified this damage
    disputed: bool,
    dispute_refs: Vec<String>,
    
    // Location
    location: GeoPosition,
    zone: String,
    
    // Commits
    commit_ref: String,             // When damage was recorded
}

enum DamageType {
    KineticStrike,      // Missile, bomb, shell
    Structural,         // Building collapse
    Infrastructure,     // Power, water, comms
    Electronic,         // Cyber, EMP
    Chemical,
    Nuclear,
    Unknown,
}

enum ScarSeverity {
    Minor,              // Repairable quickly
    Moderate,           // Weeks to repair
    Major,              // Months to repair
    Critical,           // Years to repair
    Catastrophic,       // Permanent loss
}
```

**Integration with Buildings:**
- Buildings accumulate damage scars
- Scars are visible on globe (building material changes)
- Scars affect building functionality
- Scars are auditable (who, what, when)

---

## 🎮 STARCRAFT MAPPING (INTENTIONAL)

**This is not metaphor. This is the correct model.**

| Real World | StarCraft | Relay |
|------------|-----------|-------|
| Aircraft carrier | Battlecruiser | ForceUnit (Strategic) |
| Fighter jet | Wraith | ForceUnit (Tactical) |
| Tank | Siege Tank | ForceUnit (Tactical) |
| Military base | Barracks/Factory | ForceAnchor (Building) |
| Logistics convoy | SCV train | Task (Shipment) |
| Missile launch | Nuclear launch | Event (Commit) |
| City destroyed | Building destroyed | DamageScar on Building |
| Troop movement | Unit movement | ForceUnit.movement_vector |
| Force concentration | Army rally | Aggregation (via lens) |

**Why this mapping is correct:**
- StarCraft nailed resource constraints
- StarCraft made logistics visible
- StarCraft made force concentration legible
- StarCraft made damage irreversible (scars)

**Relay extends StarCraft by:**
- Making it real (not a game)
- Adding governance (zones, voting, authority)
- Adding auditability (commits, filaments)
- Adding progressive disclosure (LOD, lenses)

---

## 🔄 INTEGRATION WITH EXISTING ARCHITECTURE

### **With c13 (Rule-Based Zones):**
- ✅ Military bases are in zones
- ✅ Zones define access constraints (no-fly, restricted)
- ✅ Zones define conflict rules (civilian vs military zones)
- ✅ Emergency zones override (wartime rules)

### **With c10 (Ontological Foundation):**
- ✅ Bases are buildings (space tiles)
- ✅ Force units are units (tradable, governable)
- ✅ Proximity channels detect units nearby
- ✅ Identity filaments control access to LOD

### **With c9 (StarCraft Model):**
- ✅ HUD shows force concentration
- ✅ Globe shows unit movements
- ✅ Build queues = production (arms manufacturing)
- ✅ Supply = logistics constraints

### **With c5 (RenderSpec v1):**
- ✅ New node kind: `"force_unit"`
- ✅ Material tags: `force_carrier`, `force_fighter`, `force_damaged`
- ✅ Geometry: 3D models at appropriate LOD
- ✅ Movement: animated paths, vectors

---

## 📦 RENDERSPEC INTEGRATION

### **New Node Kind: ForceUnit**

```json
{
  "id": "force.us.navy.cvn_78.uss_ford",
  "kind": "force_unit",
  "transform": {
    "position": [15.1234, 110.5678, 0],
    "rotation": [0, 0, 90],
    "scale": [1, 1, 1]
  },
  "material": "force_carrier_active",
  "props": {
    "unit_id": "force.us.navy.cvn_78.uss_ford",
    "unit_class": "AircraftCarrier",
    "operator": "US Navy",
    "operational_state": "Active",
    "capability_class": "Strategic",
    "lod_level": "LOD2",
    "movement_vector": {
      "heading": 90.0,
      "speed": 25.0
    }
  },
  "geometry": {
    "type": "model_3d",
    "lod": "LOD2",
    "model_ref": "models/carrier_generic.glb"
  },
  "animation": {
    "type": "movement",
    "path": [[15.1234, 110.5678, 0], [18.0, 115.0, 0]],
    "duration": 86400000,
    "loop": false
  }
}
```

**Material Tags (new):**
- `force_carrier_active` - Blue (operational carrier)
- `force_fighter_active` - Blue (operational fighter)
- `force_strategic_standby` - Yellow (strategic asset on standby)
- `force_damaged` - Orange (damaged unit)
- `force_destroyed` - Red (destroyed unit)
- `force_unknown` - Gray (unverified or LOD0)

---

## 🚫 WHAT IS EXPLICITLY FORBIDDEN

### **Invariant C15.1: No Hidden Force**
> Any object capable of large-scale damage must exist in Relay. "Classified" does not mean "non-existent." LOD governs detail, not existence.

### **Invariant C15.2: No Retroactive Movement**
> Force unit positions and movements are commits. Cannot be rewritten after the fact. History is auditable.

### **Invariant C15.3: No Invisible Damage**
> Damage to buildings/units must be recorded as scars. Casualty counts may be disputed, but damage existence cannot be erased.

### **Invariant C15.4: No Global Threat Scalar**
> No single "world threat level." Conflict state is spatial, temporal, and lens-dependent. Aggregation is explicit.

### **Invariant C15.5: No Force Without Accountability**
> Every ForceUnit has an operator. Every damage event has a source (if known) and verification trail.

---

## 🎯 USE CASES (LOCKED)

### **Use Case 1: Early Escalation Detection**

**Setup:**
- Two nations have border dispute
- Analyst applies "Force Mobilization" lens

**Behavior:**
1. Lens detects:
   - Nation A: 3x normal units near border
   - Nation B: 2x normal units near border
   - Both sides deploying strategic assets
2. Alert: "Mutual buildup detected. Escalation risk: HIGH"
3. Globe shows:
   - Red concentration zones
   - Movement vectors converging
   - Logistics surge indicators
4. Civil society sees same data
5. International pressure applied BEFORE war starts

**Result:** Escalation visible early. Society reacts. War prevented (or at least, escalation undeniable).

---

### **Use Case 2: Damage Accountability**

**Setup:**
- City building destroyed
- Multiple narratives about who/what caused it

**Behavior:**
1. DamageScar created with:
   - Exact timestamp
   - Location
   - Damage type: KineticStrike
   - Source: Unknown (initially)
2. Verification process:
   - Analysts inspect nearby ForceUnit movements
   - Cross-reference timing
   - Identify likely source unit
3. DamageScar updated with:
   - Source_id: "force.nation_x.fighter_123"
   - Verified_by: ["analyst_a", "analyst_b"]
4. Dispute filed by nation_x (if they contest)
5. Dispute becomes inspectable filament
6. No erasure. History preserved.

**Result:** Accountability. Auditable damage. No "fog of war" excuses.

---

### **Use Case 3: Civilian Awareness**

**Setup:**
- Civilian in border city
- Applies "Civilian Impact" lens

**Behavior:**
1. Sees:
   - 5 military units within 50km
   - 2 strategic assets within 100km
   - Recent damage scars on nearby infrastructure
2. Alert: "Elevated military activity nearby. Consider evacuation routes."
3. Can inspect:
   - Which units (LOD1: class + operator)
   - Movement vectors (if public)
   - Historical patterns
4. Makes informed decision to evacuate or shelter

**Result:** Civilians not caught unaware. Early warning system built into physics.

---

### **Use Case 4: Global Transparency Demand**

**Setup:**
- Society demands transparency of strategic assets
- "Global Conflict State" lens becomes public

**Behavior:**
1. All strategic assets (carriers, bombers, nuclear silos) visible at LOD1
2. Buildup patterns visible
3. Escalation risk computed automatically
4. Nations cannot deny mobilization
5. Peace movements can cite exact force concentrations
6. Diplomacy informed by real data

**Result:** "We demand transparency" becomes structural, not aspirational.

---

## 🧪 TESTING STRATEGY

### **Test 1: Force Unit Creation**
**Given:** New carrier deployed  
**When:** ForceUnit created with position  
**Then:** Appears on globe at correct LOD

### **Test 2: LOD Transitions**
**Given:** Viewer zooms in on force unit  
**When:** Distance < LOD threshold  
**Then:** Detail level increases (LOD1 → LOD2)

### **Test 3: Lens Application**
**Given:** "Force Mobilization" lens  
**When:** Applied to globe  
**Then:** Only mobilized units visible, aggregations shown

### **Test 4: Damage Scar Creation**
**Given:** Building destroyed  
**When:** DamageScar created  
**Then:** Building material changes, scar visible, auditable

### **Test 5: Movement Tracking**
**Given:** Force unit moving  
**When:** Position updates every 5 minutes  
**Then:** Movement vector calculated, path rendered

---

## 🚧 IMPLEMENTATION ROADMAP

### **Phase 1: Core Force Objects (PR #35)**
- [ ] ForceUnit struct
- [ ] ForceAnchor struct
- [ ] DamageScar struct
- [ ] Storage (JSONL per unit)
- [ ] Basic CRUD API

### **Phase 2: LOD System (PR #36)**
- [ ] LODPolicy objects
- [ ] LOD computation engine
- [ ] Distance-based LOD switching
- [ ] Lens-based LOD overrides

### **Phase 3: Lens System (PR #37)**
- [ ] Lens struct + storage
- [ ] Lens application engine
- [ ] 4 core lenses (Global Conflict, Force Mobilization, Escalation Risk, Civilian Impact)
- [ ] Aggregation rules
- [ ] Alert thresholds

### **Phase 4: Movement & Tracking (PR #38)**
- [ ] MovementVector calculation
- [ ] Position update commits
- [ ] Path rendering
- [ ] Historical movement replay

### **Phase 5: Damage System (PR #39)**
- [ ] DamageScar creation
- [ ] Building damage integration
- [ ] Verification system
- [ ] Dispute handling

### **Phase 6: RenderSpec Integration (PR #40)**
- [ ] Force unit node kind
- [ ] LOD-based 3D models
- [ ] Material tags for states
- [ ] Movement animations
- [ ] Damage visualization

### **Phase 7: Integration with Bases (PR #41)**
- [ ] ForceAnchor as Building extension
- [ ] Base damage scars
- [ ] Zone constraints (c13)
- [ ] Capacity management

---

## 🔮 FUTURE EXTENSIONS

### **Dynamic Conflict Zones**
- War zones that activate/deactivate
- Different rules during active conflict
- Humanitarian corridors
- Ceasefire verification

### **Arms Manufacturing**
- Production buildings create ForceUnits
- Resource constraints (materials, fuel)
- Build queues for military production
- Supply chain visibility

### **Coalition Forces**
- Multi-nation force anchors
- Shared command structures
- Authority delegation for joint operations

### **Escalation De-escalation Patterns**
- AI detection of escalation patterns
- Automatic alerts to governance bodies
- Historical pattern matching
- Confidence intervals on predictions

---

## 📊 REALM CONVERSIONS UPDATE

**Adding 4 new conversions:**

13. **Military Force:** Hidden power → ForceUnit objects
14. **Conflict:** Opaque escalation → Lens-visible patterns
15. **Damage:** Contested narratives → DamageScar objects
16. **Strategic Assets:** Classified existence → LOD-governed visibility

**Total conversions:** 16/20 complete (80%)

---

## 💡 THE PARADIGM SHIFT

**Before c15:**
- ❌ Military assets invisible (submarines, strategic weapons)
- ❌ Buildup deniable ("training exercises")
- ❌ Escalation opaque (slow boil → surprise war)
- ❌ Damage contested (who did what)
- ❌ Power concentration hidden

**After c15:**
- ✅ All critical assets exist as ForceUnits
- ✅ Buildup visible via lenses (undeniable patterns)
- ✅ Escalation legible (early detection possible)
- ✅ Damage auditable (scars + verification)
- ✅ Power concentration transparent (LOD-governed)

---

## 🎯 THE ULTIMATE GOAL

**Not militarization. Legibility.**

**The thesis:**
> Wars happen because force is invisible. Make force legible, and society can react before catastrophe.

**The method:**
> Convert battleships → StarCraft units. Treat military power as a coordination system. Apply Relay's physics: spatial, temporal, auditable.

**The outcome:**
> Early warning system built into reality itself. Escalation becomes undeniable. Transparency becomes structural, not moral.

---

## 🔒 WHY THIS FITS RELAY PERFECTLY

1. **Respects StarCraft economics** (units, resources, logistics)
2. **Extends rule-based zones** (military vs civilian zones)
3. **Uses LOD instead of censorship** (progressive disclosure)
4. **Does not force full transparency immediately** (governance decides)
5. **Prevents war by making escalation undeniable** (not by moral argument)
6. **Treats military power as coordination system** (filaments, commits, governance)

---

## 📋 FINAL LOCK STATEMENT

**All critical objects must be rendered in Relay based on LOD relevance.**

**If it can kill thousands, destroy infrastructure, or change geopolitical balance, it must exist as a first-class entity.**

**LOD governs detail. Lenses govern visibility. But non-existence is forbidden.**

**This is not a surveillance system. This is a legibility system.**

**This is not about control. This is about early warning.**

**This is not militarization. This is conflict-system visibility.**

---

**Status:** 🔒 LOCKED  
**Invariants added:** 5 (total: 93)  
**Realm conversions:** 16/20 (80%)  
**Next:** Implement Phase 1 (Core Force Objects, PR #35)

---

**THIS IS HOW RELAY CONVERTS HIDDEN MILITARY POWER INTO LEGIBLE, AUDITABLE REALITY.**

**END OF architecture@c15**

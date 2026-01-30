# architecture@c14: Presence, Sessions, and Co-Located Play (Social Layer)

**Date:** 2026-01-29  
**Status:** 🔒 Locked  
**Depends on:** architecture@c13 (Rule-Based Zones), architecture@c10 (Ontological Foundation), architecture@c9 (StarCraft Model), PR #9 (Identity Filaments)

---

## 🎯 THE PROBLEM

**Current "social" systems fail because:**
- Presence = fixed profile (not relationship-dependent)
- Identity = all-or-nothing disclosure (no progressive trust)
- Gaming = separate from work/life (disconnected experiences)
- Local + remote = different modes (not unified space)
- Location = GPS coords (not cognitive anchor)

**Result:**
- Privacy violations (forced identity exposure)
- Social friction (context collapse)
- Isolation (digital vs physical divide)
- Coordination failure (local vs remote teams siloed)

**The core failure:**
> Humans exist in physical space AND cognitive space simultaneously, but current systems force a choice: be present physically OR digitally, not both.

---

## 🧬 THE SOLUTION: TRIPLE BINDING + PROGRESSIVE PRESENCE

### **Core Invariant (Non-Negotiable):**
> **Physical proximity + Cognitive channel + Graphics anchor = Triple binding. Presence is progressive identity disclosure computed per viewer, per zone, per session. Sessions are first-class spatial objects requiring consent. Local + remote users share the same cognitive space.**

**This creates:**
- Privacy by design (relationship-dependent visibility)
- Unified experience (gaming = work = coordination)
- Local + remote parity (same cognitive space)
- Earned trust (progressive disclosure through participation)

---

## 🔑 THE TRIPLE BINDING

**What makes Relay feel like reality + RTS + internet simultaneously:**

### **Layer 1: Physical Proximity**
- Wi-Fi / BLE / UWB / Hotspot detection
- Defines a **Proximity Region**
- Who is physically nearby

### **Layer 2: Cognitive Channel**
- Shared session space + rules
- Defines what you can DO together
- Who is cognitively nearby

### **Layer 3: Graphics Anchor**
- Building beacon on 3D Relay globe
- Defines where it APPEARS in the world
- How others see your presence

### **Plus: Remote Cognitive Join**
- Layer 4: People worldwide attach to same beacon
- Same cognitive space, different latency
- Locals get richer rendering, remotes get command seats

**Result:**
> Chinese player flies to France gaming hub → everyone knows he's legitimate → local teams + remote teams share same arena → intimacy through earned trust, not forced disclosure.

---

## 📐 PRESENCE AS PROGRESSIVE IDENTITY DISCLOSURE

**Problem:** Current systems force all-or-nothing identity exposure.

**Solution:** Presence tiers computed dynamically based on relationship, zone, and session.

### **Presence Tier Model:**

```rust
struct PresenceView {
    viewer_id: String,              // Who is looking
    target_id: String,              // Who is being seen
    context: PresenceContext,       // Where/when
    tier: PresenceTier,             // Computed tier
    allowed_branches: Vec<String>,  // Which identity branches visible
    last_updated: u64,
}

enum PresenceTier {
    Tier0_Invisible,     // Count only, no identity
    Tier1_Beacon,        // Anonymous glyph
    Tier2_Handle,        // Public label + minimal badges
    Tier3_BranchPreview, // Top-level categories only
    Tier4_TreeSlice,     // Specific allowed filaments
    Tier5_FullTree,      // Complete identity tree (rare)
}

struct PresenceContext {
    zone: String,                   // Which RBZ (from c13)
    session: Option<String>,        // Active session (if any)
    proximity_distance: f64,        // Physical distance (meters)
    relationship_edges: Vec<String>,// Existing relationships
}
```

### **How Tier is Computed:**

```rust
fn compute_presence_tier(
    viewer_id: &str,
    target_id: &str,
    context: &PresenceContext,
    identity_store: &IdentityStore,
) -> PresenceTier {
    // Step 1: Check explicit user preferences
    if let Some(tier) = target.get_preference_for(viewer_id) {
        return tier;
    }
    
    // Step 2: Check zone defaults
    let zone_default = context.zone.presence_policy.default_tier;
    
    // Step 3: Check relationship edges
    let relationship_tier = match identity_store.get_relationship(viewer_id, target_id) {
        Some(Relationship::Friend) => Tier5_FullTree,
        Some(Relationship::Teammate) => Tier4_TreeSlice,
        Some(Relationship::Acquaintance) => Tier3_BranchPreview,
        Some(Relationship::Blocked) => Tier0_Invisible,
        None => zone_default,
    };
    
    // Step 4: Check session participation
    if let Some(session_id) = &context.session {
        if session.participants.contains(viewer_id) {
            return std::cmp::max(relationship_tier, Tier4_TreeSlice);
        }
    }
    
    // Step 5: Apply proximity modifiers
    if context.proximity_distance < 10.0 {
        return std::cmp::max(relationship_tier, Tier2_Handle);
    }
    
    // Return computed tier
    relationship_tier
}
```

**Invariant:**
> Public visibility ≠ full identity. Session observability ≠ identity exposure.

---

## 🎮 SESSIONS AS FIRST-CLASS OBJECTS

**A Session is a spatial anchor where users interact, play, work, or spectate.**

### **Session Schema:**

```rust
struct Session {
    session_id: String,             // "session.france.paris.hub01.duel_001"
    session_type: SessionType,
    created_at: u64,
    
    // Spatial anchor
    anchor: BuildingBeacon,         // Where in the world
    location: GeoPosition,          // Precise position
    zone: String,                   // Which RBZ (from c13)
    
    // Participants
    participants: Vec<Participant>,
    spectators: Vec<UserId>,
    max_participants: u64,
    max_spectators: Option<u64>,
    
    // Rendering
    rendering_mode: RenderMode,
    visibility: SessionVisibility,
    
    // Rules
    consent_required: bool,         // Always true for Duel
    spectate_policy: SpectatorPolicy,
    zone_constraints: Vec<String>,  // From RBZ (c13)
    
    // State
    status: SessionStatus,
    start_commit: String,
    end_commit: Option<String>,
    
    // Outcomes
    commits: Vec<String>,           // All actions/events
    scars: Vec<String>,             // Failures/disputes
    artifacts: Vec<String>,         // Created objects
}

enum SessionType {
    Duel,           // 1v1 competition
    CoopRTS,        // Multi-user RTS command
    SharedBuild,    // Collaborative 3D editing
    Spectate,       // Watch-only
    Custom(String),
}

enum RenderMode {
    ThreeDInAir,        // AR/shared space hologram
    TwoDScreen,         // Shared screen/display
    ThreeDPlatform,     // Shared 3D editing surface
}

enum SessionStatus {
    Pending,        // Invitation sent, awaiting consent
    Active,         // In progress
    Paused,         // Temporarily suspended
    Completed,      // Successfully ended
    Cancelled,      // Terminated early
}

struct Participant {
    user_id: String,
    role: ParticipantRole,
    join_commit: String,
    leave_commit: Option<String>,
    consent_given: bool,
}

enum ParticipantRole {
    Host,           // Created session
    Player,         // Active participant
    Commander,      // RTS command role
    Builder,        // 3D editing role
    Spectator,      // Watching only
}

struct SpectatorPolicy {
    allowed: bool,
    requires_consent: bool,
    anonymous: bool,            // Spectators see gameplay but no names
    max_spectators: Option<u64>,
}
```

### **Session Invariants:**

1. **Consent Required:** Battle/duel cannot be forced. Opt-in only.
2. **Bystander Visibility:** Session existence + type always visible high-level.
3. **Privacy Preserved:** Spectators see session, not necessarily full identity.
4. **Logged as Commits:** All actions, outcomes, disputes = filament events.

---

## 🏗️ BUILDING BEACONS (WORLD SHARD DOORWAYS)

**BuildingBeacon objects anchor sessions in physical + cognitive + graphics space.**

### **Building Beacon Schema:**

```rust
struct BuildingBeacon {
    beacon_id: String,              // "beacon.france.paris.gaming_hub_01"
    name: String,                   // "Paris Gaming Hub"
    created_at: u64,
    
    // Physical
    building_ref: String,           // Links to Building (c9/c10)
    location: GeoPosition,
    proximity_range: f64,           // Detection radius (meters)
    
    // Cognitive
    shared_space_id: String,        // Unique cognitive space ID
    active_sessions: Vec<String>,   // Session IDs
    
    // Participants
    local_participants: Vec<UserId>,    // Physically present
    remote_participants: Vec<UserId>,   // Cognitively joined
    capacity: BeaconCapacity,
    
    // Rules
    zone: String,                   // Which RBZ (from c13)
    presence_policy: PresencePolicy,// Default tiers
    session_types_allowed: Vec<SessionType>,
    
    // State
    status: BeaconStatus,
    commits: Vec<String>,
}

struct BeaconCapacity {
    max_local: u64,             // Physical space limit
    max_remote: Option<u64>,    // Cognitive join limit (optional)
    current_local: u64,
    current_remote: u64,
}

struct PresencePolicy {
    default_tier: PresenceTier,
    zone_override: Option<PresenceTier>,
    session_unlock_tier: Option<PresenceTier>,
}

enum BeaconStatus {
    Active,         // Accepting participants
    Full,           // At capacity
    Inactive,       // Temporarily closed
    Maintenance,    // Undergoing updates
}
```

### **What Beacons Enable:**

1. **Local Cluster Density:** See how many people at a location
2. **Remote Join Streams:** Worldwide users "plug into" beacon
3. **Session Volumes:** Hover in/around building in 3D
4. **Zone Integration:** Beacon inherits zone rules (c13)

**Visual on Globe:**
- Building lights up when beacon active
- Glow intensity = participant density
- Session volumes visible as holograms
- Remote join streams = glowing lines from other regions

---

## 🚪 VISITOR ARRIVAL PROTOCOL

**When a user travels to a new location (e.g., Chinese player → France gaming hub):**

### **Arrival Flow:**

```rust
struct VisitorArrival {
    visitor_id: String,
    beacon_id: String,
    arrival_time: u64,
    
    // Step 1: Detection
    physical_detection: DetectionMethod,    // Wi-Fi, BLE, UWB
    
    // Step 2: Default Presence
    initial_tier: PresenceTier,             // Tier1 or Tier2 by default
    
    // Step 3: Auto-Match Relationships
    matched_relationships: Vec<Relationship>,
    
    // Step 4: Earned Unlock
    unlock_events: Vec<UnlockEvent>,
    
    // Commits
    arrival_commit: String,
    departure_commit: Option<String>,
}

enum DetectionMethod {
    WiFiProximity,
    BluetoothLE,
    UltraWideBand,
    ManualCheckin,
}

struct UnlockEvent {
    unlock_type: UnlockType,
    tier_granted: PresenceTier,
    granted_to: Vec<UserId>,        // Who gets to see more
    reason: String,
    commit_ref: String,
}

enum UnlockType {
    SessionParticipation,   // Played/worked together
    MutualConsent,          // Both agreed to share more
    LocalGovernance,        // Required by hub rules
    TimeInZone,             // Spent time, earned trust
}
```

### **Example Flow:**

1. **Arrival Detection:**
   - Physical: Wi-Fi connects to hub
   - Beacon registers new participant
   - Initial presence: Tier2 (Handle + badges)

2. **Auto-Match Relationships:**
   - Identity store checks for existing relationships
   - Friends see Tier4 immediately
   - Teammates see Tier3 immediately
   - Strangers see Tier2

3. **Bystander View:**
   - "Verified participant arrived"
   - Handle: "Chen_StarCraft_Pro"
   - Badges: ["Verified Player", "Tournament Winner"]
   - No full identity tree

4. **Earned Unlock:**
   - Chen joins CoopRTS session with local team
   - After 30 minutes → session members unlock Tier4
   - Can see relevant identity branches (gaming history, team affiliations)
   - Others still see Tier2

5. **Result:**
   > "Everyone knows him intimately" = Close allies see deeply (Tier4-5), bystanders see legitimacy (Tier2), others learn through participation.

**Not surveillance. Progressive trust.**

---

## 🎮 SESSION TYPES (DETAILED)

### **1. Duel (1v1 Competition)**

**Example:** Street Fighter, but YOU are the character (motion capture, AR).

```rust
struct DuelSession {
    // Base session fields...
    duel_type: DuelType,
    rounds: u64,
    current_round: u64,
    scores: HashMap<UserId, u64>,
    winner: Option<UserId>,
}

enum DuelType {
    FightingGame,       // Motion-capture combat
    StrategyGame,       // Chess, Go, etc.
    RacingGame,         // Physical or virtual
    Custom(String),
}
```

**Invariants:**
- ✅ Mutual consent required (no forced battles)
- ✅ Spectators can watch (if allowed)
- ✅ Outcomes logged as commits
- ✅ Disputes create scars (cheating accusations)

---

### **2. Co-op RTS Command**

**Example:** Million players commanding StarCraft units against Zerg.

```rust
struct CoopRTSSession {
    // Base session fields...
    command_structure: CommandStructure,
    units_controlled: HashMap<UserId, Vec<UnitId>>,
    objectives: Vec<Objective>,
    battle_state: BattleState,
}

struct CommandStructure {
    supreme_commander: Option<UserId>,
    regional_commanders: Vec<UserId>,
    squad_leaders: Vec<UserId>,
    soldiers: Vec<UserId>,
}

struct Objective {
    objective_id: String,
    description: String,
    assigned_to: Vec<UserId>,
    status: ObjectiveStatus,
}
```

**Key Features:**
- Local players get low-latency control
- Remote players join as commanders/squad leaders
- Shared cognitive space (all see same battle state)
- Command hierarchy prevents chaos

---

### **3. Shared Build/Edit**

**Example:** Two designers manipulating 3D object together, bystanders can watch.

```rust
struct SharedBuildSession {
    // Base session fields...
    project_id: String,
    artifact: SharedArtifact,
    edit_permissions: EditPermissions,
    version_history: Vec<String>,      // Commit refs
}

struct SharedArtifact {
    artifact_id: String,
    artifact_type: ArtifactType,        // 3D model, document, code, etc.
    current_state: String,              // JSON or binary
    locked_by: Option<UserId>,          // Pessimistic locking
}

enum ArtifactType {
    ThreeDModel,
    Document,
    Code,
    Design,
    Custom(String),
}

struct EditPermissions {
    owner: UserId,
    editors: Vec<UserId>,
    viewers: Vec<UserId>,
}
```

**Key Features:**
- All edits = versioned commits
- Merge conflicts resolved via governance
- Bystanders see silhouette + "Build in progress"
- Can request spectate/join if allowed

---

### **4. Spectate**

**Watch-only sessions. Always allowed at high level (existence visible), detail depends on permissions.**

```rust
struct SpectateSession {
    target_session_id: String,
    spectator_tier: SpectateTier,
    anonymous: bool,                // Spectator sees gameplay but no names
}

enum SpectateTier {
    Silhouette,     // Session exists, no detail
    Overview,       // See gameplay, no player names
    Full,           // See everything (players, scores, etc.)
}
```

---

## 🎨 RENDERING MODES

### **A) 3D In-Air (AR / Shared Space)**

**Session volume appears between participants.**

- **Participants see:** Interactive object/game + allowed presence tiers
- **Bystanders see:** Hologram silhouette + type label ("Duel", "Build Session")
- **Spectators see:** Gameplay (if allowed), no names (if anonymous)

**RenderSpec:**
```json
{
  "id": "session.duel_001.volume",
  "kind": "session_volume",
  "transform": {
    "position": [48.8566, 2.3522, 2.0],
    "rotation": [0, 0, 0],
    "scale": [3.0, 3.0, 3.0]
  },
  "material": "session_duel_active",
  "geometry": {
    "type": "hologram",
    "opacity": 0.7
  }
}
```

---

### **B) 2D Shared Screen**

**Both participants bind to same session stream.**

- Physical display in location
- Or shared device view
- Bystanders see session marker in space

---

### **C) Shared 3D Editing Platform**

**Project object is session's core artifact.**

- All edits = versioned commits
- Merge/scar rules apply
- Reversible by governance

---

## 🚨 SAFETY & ANTI-HARASSMENT

### **Safety Locks (Required):**

```rust
struct SafetyPolicy {
    // Rate limiting
    max_invites_per_hour: u64,
    max_declines_before_block: u64,
    
    // User modes
    do_not_disturb: bool,
    block_list: Vec<UserId>,
    avoid_list: Vec<UserId>,        // Soft block (invisible to each other)
    
    // Location rules
    zone_constraints: Vec<String>,  // From RBZ (c13)
    allowed_session_types: Vec<SessionType>,
    quiet_hours: Option<TimeWindow>,
}
```

### **Enforcement:**

1. **Mutual Consent:** Duels require both parties to accept
2. **Rate Limits:** Max 10 invites/hour (anti-spam)
3. **Do Not Disturb:** No invites received
4. **Block/Avoid:** Hides you + prevents requests
5. **Zone Rules:** Library = no duels, Gaming hub = all types allowed

**Violations = Scars (logged events), not silent moderation.**

---

## 🔗 INTEGRATION WITH EXISTING ARCHITECTURE

### **With c13 (Rule-Based Zones):**
- ✅ Sessions constrained by zone type
- ✅ Beacons inherit zone rules
- ✅ Enforcement modes apply to sessions
- ✅ Transitions visible when moving between zones

### **With c10 (Ontological Foundation):**
- ✅ Buildings as beacons
- ✅ Proximity channels = session detection
- ✅ Identity filaments = presence computation

### **With c9 (StarCraft Model):**
- ✅ HUD shows nearby sessions
- ✅ Globe shows beacon activity
- ✅ Sessions = production queues

### **With PR #9 (Identity Filaments):**
- ✅ Access derivation = presence tier computation
- ✅ Attestations = tier unlocks
- ✅ Bindings = session participation grants

---

## 📦 RENDERSPEC INTEGRATION

### **New Node Kinds:**

**1. Session Volume:**
```json
{
  "id": "session.duel_001",
  "kind": "session_volume",
  "material": "session_duel_active",
  "geometry": {"type": "hologram"}
}
```

**2. Building Beacon:**
```json
{
  "id": "beacon.paris.hub01",
  "kind": "building_beacon",
  "material": "beacon_active",
  "props": {
    "local_count": 15,
    "remote_count": 47,
    "active_sessions": 3
  }
}
```

**Material Tags:**
- `session_duel_active` - Red (1v1 combat)
- `session_coop_active` - Blue (team collaboration)
- `session_build_active` - Green (shared creation)
- `beacon_active` - Pulsing white (beacon online)
- `beacon_full` - Orange (at capacity)

---

## 🚧 IMPLEMENTATION ROADMAP

**Phase 1: Presence System (PR #30)** - 2-3 weeks
**Phase 2: Session Objects (PR #31)** - 2-3 weeks
**Phase 3: Building Beacons (PR #32)** - 1-2 weeks
**Phase 4: Visitor Arrival (PR #33)** - 1-2 weeks
**Phase 5: Rendering Integration (PR #34)** - 1-2 weeks

**Total:** ~9-12 weeks

---

## 🎯 SUCCESS CRITERIA

**This works if:**
1. ✅ Users can see different presence tiers based on relationship
2. ✅ Sessions require consent (no forced interaction)
3. ✅ Local + remote share same cognitive space
4. ✅ Visitor arrival = progressive trust (not forced disclosure)
5. ✅ Bystanders see sessions without identity exposure
6. ✅ Gaming = work = coordination (unified experience)

---

**Status:** 🔒 LOCKED  
**Invariants added:** 7 (total: 100)  
**Next:** Implement Phase 1 (Presence System, PR #30)

---

**THIS IS HOW RELAY UNIFIES PHYSICAL + COGNITIVE SPACE.**

**END OF architecture@c14**

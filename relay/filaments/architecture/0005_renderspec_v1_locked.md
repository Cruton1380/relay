# COMMIT E: RenderSpec v1 Locked

**Filament:** `architecture`  
**Commit Index:** 5  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** SPEC_LOCK

---

## INVARIANT LOCKED

**RenderSpec v1 is the canonical JSON format for Layer 2→3 scene graphs. Material tags are semantic strings. Geometry is deterministic.**

---

## DECISION

RenderSpec v1 is now **LOCKED** as the contract between Layer 2 (Relay Physics) and Layer 3 (Frontend rendering).

**Key Principles:**
1. **Semantic, not artistic** - Layer 2 outputs semantic tags, Layer 3 interprets visually
2. **Deterministic** - Same truth → same scene graph (no randomness, no `Date.now()`)
3. **Purely derived** - Generated from truth objects only (commits, units, filaments)
4. **Versioned** - `schema_version` field enables evolution without breaking clients
5. **Extensible** - Unknown fields ignored (forward compatibility)

---

## TOP-LEVEL ENVELOPE

```json
{
  "schema_version": "relay-render-v1",
  "generated_from": {
    "event_id": 42,
    "commit_ref": "work.W123@c7",
    "timestamp": "2026-01-28T10:30:00Z"
  },
  "nodes": [...],
  "links": [...],
  "animation_intents": [...]
}
```

### Fields

**`schema_version`** (required, string)
- Format: `"relay-render-v1"`
- Layer 3 MUST check this field
- If version unknown → log warning + render with best effort

**`generated_from`** (required, object)
- `event_id` (optional, u64) - Master event log position
- `commit_ref` (optional, string) - If derived from specific commit
- `timestamp` (required, RFC3339) - When this scene was generated
- **Determinism rule:** Same inputs → same `generated_from`

**`nodes`** (required, array)
- Scene graph nodes (units, filaments, timeboxes, scars, etc.)
- See Node Schema below

**`links`** (optional, array)
- Causal links between nodes (for forensic chamber)
- Each link: `{ from_id, to_id, link_type, material }`

**`animation_intents`** (optional, array)
- Animation triggers keyed by `event_id`
- See Animation Intents below

---

## NODE SCHEMA

Each node has:

```json
{
  "id": "unit.alice.001",
  "kind": "unit",
  "transform": {
    "position": [0.0, 0.0, 0.0],
    "rotation": [0.0, 0.0, 0.0, 1.0],
    "scale": [1.0, 1.0, 1.0]
  },
  "material": "scv_working",
  "props": {
    "state": "Working",
    "attached_filament": "work.W123"
  }
}
```

### Fields

**`id`** (required, string)
- Stable identifier (e.g., `"unit.alice.001"`, `"filament.work.W123"`, `"timebox.work.W123@c7"`)
- MUST be unique within scene
- MUST be stable across regenerations (deterministic)

**`kind`** (required, string)
- Node type: `"unit" | "filament" | "timebox" | "scar" | "anchor" | "globe" | "label"`
- Layer 3 uses this to select renderer

**`transform`** (required, object)
- `position` (required, [f32; 3]) - World-space coordinates
- `rotation` (required, [f32; 4]) - Quaternion (x, y, z, w)
- `scale` (optional, [f32; 3]) - Defaults to [1, 1, 1]

**`material`** (required, string)
- **Semantic tag** (not RGB values)
- Examples: `"scv_working"`, `"encrypted_channel"`, `"commit_accepted"`
- Layer 3 maps to shaders/materials
- Unknown materials → fallback to default

**`props`** (optional, object)
- Node-specific properties (free-form)
- **Keys are versioned** - New keys added without breaking old clients
- Examples: `state`, `attached_filament`, `commit_ref`, `op_type`

---

## NODE KIND SPECIFICATIONS

### Unit (SCV Agent)

```json
{
  "id": "unit.alice.001",
  "kind": "unit",
  "transform": {...},
  "material": "scv_working",
  "props": {
    "state": "Working",
    "attached_filament": "work.W123",
    "current_task_filament": "work.W123",
    "rank": "Engineer"
  }
}
```

**Material tags:**
- `scv_idle` - Unit idle (no task)
- `scv_moving` - Unit moving to task
- `scv_working` - Unit attached and working
- `scv_blocked` - Unit blocked (output rejected)
- `scv_awaiting` - Unit awaiting authority

**Props:**
- `state` (string) - Matches `UnitState` enum
- `attached_filament` (string, optional) - Current filament
- `current_task_filament` (string, optional) - Assigned task
- `rank` (string, optional) - "Engineer" | "Manager" | "Admin"

---

### Filament (Thread/Channel)

**LOCKED:** Filaments use **polyline segments** representation

```json
{
  "id": "filament.work.W123",
  "kind": "filament",
  "geometry": {
    "points": [
      [0.0, 0.0, 0.0],
      [1.0, 0.5, 0.2],
      [2.0, 1.0, 0.4],
      [3.0, 1.5, 0.6]
    ],
    "radii": [0.05, 0.05, 0.05, 0.05],
    "commit_indices": [1, 2, 3, 4]
  },
  "material": "encrypted_channel",
  "props": {
    "filament_id": "work.W123",
    "head_commit_index": 4,
    "rotation_params": {
      "twist_rate": 0.5,
      "direction": "clockwise"
    }
  },
  "timeboxes": [...]
}
```

**Why polyline:**
- Fast in Three.js (`THREE.TubeGeometry`)
- Works in Blender (curve from points)
- Deterministic (no spline interpolation ambiguity)

**Geometry:**
- `points` (required, array of [f32; 3]) - Polyline vertices
- `radii` (optional, array of f32) - Radius at each point (tube thickness)
- `commit_indices` (optional, array of u64) - Which commit each segment represents

**Material tags:**
- `encrypted_channel` - Twisted rope (private comms)
- `public_filament` - Straight thread (public work)
- `delegation_chain` - Chain links (authority grants)

**Props:**
- `filament_id` (string) - Filament identifier
- `head_commit_index` (u64) - Latest commit on this filament
- `rotation_params` (object, optional) - For encrypted channels:
  - `twist_rate` (f32) - Twists per unit length
  - `direction` (string) - "clockwise" | "counterclockwise"

**Timeboxes:** Array of timebox nodes (see below)

---

### Timebox (Commit Cube)

```json
{
  "id": "timebox.work.W123@c3",
  "kind": "timebox",
  "transform": {
    "position": [1.0, 0.5, 0.2],
    "rotation": [0.0, 0.0, 0.0, 1.0],
    "scale": [0.1, 0.1, 0.1]
  },
  "material": "commit_accepted",
  "props": {
    "commit_ref": "work.W123@c3",
    "commit_index": 3,
    "op_type": "OUTPUT_PROPOSED",
    "author_unit_ref": "unit.alice.001",
    "verification_state": "accepted"
  }
}
```

**Material tags:**
- `commit_accepted` - Green glow
- `commit_rejected` - Red flash

**Props:**
- `commit_ref` (string) - Full commit reference
- `commit_index` (u64) - Index on filament
- `op_type` (string) - Operation type (e.g., "OUTPUT_PROPOSED")
- `author_unit_ref` (string) - Unit that created commit
- `verification_state` (string) - "accepted" | "rejected"

**Position rule:** Place on filament at `points[commit_index]`

---

### Scar (Rejection Mark)

```json
{
  "id": "scar.work.W123@c5",
  "kind": "scar",
  "transform": {
    "position": [2.0, 1.0, 0.4],
    "rotation": [0.0, 0.0, 0.0, 1.0],
    "scale": [0.05, 0.05, 0.05]
  },
  "material": "scar_authority_denied",
  "props": {
    "at_commit_index": 5,
    "reason": "AUTHORITY_DENIED",
    "details": "Unit not authorized for OUTPUT_PROPOSED"
  }
}
```

**Material tags:**
- `scar_authority_denied` - Red mark
- `scar_schema_invalid` - Yellow mark
- `scar_ref_invalid` - Orange mark

---

### Globe (World Container)

```json
{
  "id": "globe.world",
  "kind": "globe",
  "transform": {
    "position": [0.0, 0.0, 0.0],
    "rotation": [0.0, 0.0, 0.0, 1.0],
    "scale": [10.0, 10.0, 10.0]
  },
  "material": "world_surface",
  "props": {
    "radius": 10.0,
    "segments": 64
  }
}
```

---

## LINKS (CAUSAL GRAPH)

For forensic chamber visualization:

```json
{
  "from_id": "timebox.work.W123@c7",
  "to_id": "timebox.work.W123@c6",
  "link_type": "input",
  "material": "causal_link_input"
}
```

**Fields:**
- `from_id` (required, string) - Source node
- `to_id` (required, string) - Target node
- `link_type` (required, string) - "input" | "authority" | "evidence"
- `material` (required, string) - Semantic tag

**Material tags:**
- `causal_link_input` - Blue arrow
- `causal_link_authority` - Gold chain
- `causal_link_evidence` - Purple dotted line

---

## ANIMATION INTENTS

**LOCKED:** Animations are keyed by `event_id` (deterministic triggers)

```json
{
  "target_id": "unit.alice.001",
  "trigger_event_id": 42,
  "animation_type": "pulse",
  "duration_ms": 500,
  "params": {
    "intensity": 1.0,
    "color": "green"
  }
}
```

### Fields

**`target_id`** (required, string)
- Which node to animate

**`trigger_event_id`** (required, u64)
- When SSE delivers `event_id: 42`, start this animation
- Deterministic: same event → same animation

**`animation_type`** (required, string)
- Locked types:
  - `"pulse"` - Expand/contract (work completion)
  - `"glow"` - Brightness increase (working state)
  - `"twist"` - Rotation (encryption)
  - `"decay"` - Fade out (terminal state)
  - `"explode"` - Particle burst (SCV channel cancel)
  - `"flash_error"` - Red pulse (commit rejected)

**`duration_ms`** (required, u32)
- Animation duration in milliseconds

**`params`** (optional, object)
- Animation-specific parameters
- Free-form, but common keys:
  - `intensity` (f32, 0-1)
  - `color` (string, semantic like "green" | "red")
  - `amplitude` (f32)

---

## DETERMINISM RULES (LOCKED)

### Rule 1: No Randomness
```javascript
// ❌ BAD (non-deterministic)
position: [Math.random(), Math.random(), Math.random()]

// ✅ GOOD (deterministic from hash)
position: deterministicHash(unit.id).toPosition()
```

### Rule 2: No Current Time
```javascript
// ❌ BAD (depends on when generated)
glow_intensity: Math.sin(Date.now() / 1000)

// ✅ GOOD (animation intent keyed by event_id)
animation_intents: [{
  trigger_event_id: 42,
  animation_type: "glow",
  duration_ms: 1000
}]
```

### Rule 3: Derived from Truth Only
```javascript
// ✅ GOOD (derived from unit state)
material: unit.state === "Working" ? "scv_working" : "scv_idle"

// ❌ BAD (external state)
material: userPreferences.theme === "dark" ? "scv_dark" : "scv_light"
```

---

## MATERIAL TAG CONVENTIONS (LOCKED)

### Naming Convention
```
<domain>_<semantic_state>

Examples:
- scv_working (unit + working state)
- encrypted_channel (filament + encryption)
- commit_accepted (timebox + accepted state)
```

### Namespacing (Future)
```
relay:scv_working
custom:my_material

// v1: No namespace prefix (implicit "relay:")
// v2+: Namespaces allowed for custom materials
```

---

## EXTENSIBILITY RULES (LOCKED)

### Adding New Node Kinds
```json
// v2 adds new kind "portal"
{
  "id": "portal.001",
  "kind": "portal",
  ...
}
```

**Layer 3 behavior:**
- If `kind` unknown → render as placeholder cube OR skip
- MUST NOT crash

### Adding New Fields
```json
// v2 adds "visibility" field
{
  "id": "unit.alice.001",
  "kind": "unit",
  "visibility": 0.5,  // ← New field
  ...
}
```

**Layer 3 behavior:**
- If field unknown → ignore
- MUST NOT crash

### Schema Version Handling
```javascript
const schema_version = scene.schema_version;

if (schema_version === "relay-render-v1") {
  renderV1(scene);
} else if (schema_version === "relay-render-v2") {
  renderV2(scene);
} else {
  console.warn(`Unknown schema version: ${schema_version}, rendering with v1 fallback`);
  renderV1(scene);
}
```

---

## CANONICAL EXAMPLE (FULL SCENE)

See `relay/fixtures/renderspec_v1_canonical.json`

**Includes:**
1. One SCV unit (working state)
2. One filament with 3 timeboxes (work.W123@c1-3)
3. One encryption twist animation intent (triggered by event:42)
4. Causal links between timeboxes

---

## ENDPOINT CONTRACTS

### `/render/world` - Complete World Scene

```
GET /api/relay-physics/render/world
```

**Response:** RenderSpec v1 with:
- Globe
- All units (current state)
- All filaments (polyline geometry + timeboxes)
- Animation intents (recent events)

**Use Case:** StarCraft HUD main view

---

### `/render/commit/:ref` - Forensic Chamber Scene

```
GET /api/relay-physics/render/commit/work.W123@c7
```

**Response:** RenderSpec v1 with:
- Central timebox (the commit)
- Causal link nodes (inputs, authority, evidence)
- Author unit (position at commit time)
- Evidence badges

**Use Case:** "Click time cube → open forensic chamber"

---

### `/render/filament/:id` - Single Thread Scene

```
GET /api/relay-physics/render/filament/work.W123
```

**Response:** RenderSpec v1 with:
- Filament polyline (all commits)
- Timeboxes (all commits on this filament)
- Scars (rejections)

**Use Case:** Thread detail view

---

## TESTING GOLDEN FIXTURES

**Location:** `relay/fixtures/renderspec_v1/`

```
minimal_world.json        - 1 unit, 1 filament, 1 timebox
forensic_chamber.json     - 1 timebox + causal links
encrypted_channel.json    - 1 filament with twist animation
scar_rejection.json       - 1 filament with scar mark
```

**Validation:**
```rust
#[test]
fn test_renderspec_v1_schema() {
    let json = include_str!("../../fixtures/renderspec_v1/minimal_world.json");
    let spec: RenderSpec = serde_json::from_str(json).unwrap();
    
    assert_eq!(spec.schema_version, "relay-render-v1");
    assert!(!spec.nodes.is_empty());
    // ... validate required fields
}
```

---

## CAUSAL REFS

- **Inputs:**
  - `architecture@c0` (Layer split)
  - `architecture@c1` (Render endpoints)
  - `architecture@c3` (RenderSpec stub)
  - PR #1.3 (Commit fetch shows real data)
- **Authority:** system.architect
- **Evidence:**
  - `PR-1.3-COMMIT-FETCH.md` (Shows `CommitEvent` structure)
  - Three.js `TubeGeometry` documentation (Polyline rendering)
  - Blender curve-from-points (Cinematic pipeline compatibility)

---

## WHAT THIS LOCKS

1. ✅ **Top-level envelope** (schema_version, generated_from, nodes, links, animation_intents)
2. ✅ **Node schema** (id, kind, transform, material, props)
3. ✅ **Filament geometry** (polyline segments - fast + deterministic)
4. ✅ **Material tags** (semantic strings like "scv_working")
5. ✅ **Animation intents** (keyed by event_id)
6. ✅ **Determinism rules** (no randomness, no Date.now(), derived from truth only)
7. ✅ **Extensibility** (forward compatible, unknown fields ignored)

---

## WHAT THIS UNLOCKS

### ✅ For Layer 3 Frontend
- Clear contract (know exactly what Layer 2 will return)
- Multiple frontends can implement independently (web, desktop, film)
- Deterministic rendering (same truth → same scene)

### ✅ For Layer 2 Implementation
- `/render/world` knows what to generate
- `/render/commit/:ref` knows what to generate
- `/render/filament/:id` knows what to generate

### ✅ For Testing
- Golden fixtures can validate schema
- Visual regression tests can compare scenes
- Determinism can be unit-tested

---

**Status:** ✅ LOCKED  
**Supersedes:** `architecture@c3` (RenderSpec stub)  
**Next:** Implement `/render/*` endpoints (Layer 2) OR start Layer 3 frontend

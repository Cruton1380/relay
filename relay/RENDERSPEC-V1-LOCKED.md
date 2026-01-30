# ✅ RenderSpec v1 LOCKED

**Date:** 2026-01-28  
**Status:** LOCKED  
**Reference:** `architecture@c5`

---

## WHAT WAS LOCKED

**RenderSpec v1 is now the canonical contract between Layer 2 (Relay Physics) and Layer 3 (Frontend rendering).**

---

## KEY DECISIONS

### 1. Filament Geometry: Polyline Segments ✅

**Decision:** Filaments use polyline representation (array of points)

**Why:**
- Fast in Three.js (`TubeGeometry`)
- Works in Blender (curve from points)
- Deterministic (no spline interpolation ambiguity)

**Format:**
```json
{
  "geometry": {
    "points": [[x,y,z], ...],
    "radii": [r, ...],
    "commit_indices": [1, 2, 3, ...]
  }
}
```

---

### 2. Material Tags: Semantic Strings ✅

**Decision:** Materials are semantic tags (not RGB values)

**Examples:**
- `"scv_working"` - Unit working state
- `"encrypted_channel"` - Filament with encryption
- `"commit_accepted"` - Timebox accepted state

**Layer 3 maps to visuals:**
```javascript
const materialMap = {
  "scv_working": new THREE.MeshStandardMaterial({ color: 0x00ff00, emissive: 0x00aa00 }),
  "encrypted_channel": new TwistedRopeMaterial({ twist_rate: 0.5 }),
  // ...
};
```

---

### 3. Animation Intents: Keyed by event_id ✅

**Decision:** Animations triggered by `event_id` (deterministic)

**Format:**
```json
{
  "target_id": "unit.alice.001",
  "trigger_event_id": 42,
  "animation_type": "pulse",
  "duration_ms": 500,
  "params": {...}
}
```

**Locked animation types:**
- `pulse` - Expand/contract
- `glow` - Brightness increase
- `twist` - Rotation
- `decay` - Fade out
- `explode` - Particle burst
- `flash_error` - Red pulse

---

### 4. Determinism Rules ✅

**Rules:**
1. ❌ No `Math.random()` - Use deterministic hash from IDs
2. ❌ No `Date.now()` - Use `event_id` triggers
3. ✅ Derived from truth only (commits, units, filaments)

**Example:**
```javascript
// ❌ BAD
position: [Math.random(), Math.random(), Math.random()]

// ✅ GOOD
position: deterministicPositionFromHash(unit.id)
```

---

### 5. Extensibility: Forward Compatible ✅

**Rules:**
1. Unknown `kind` → Render placeholder or skip
2. Unknown fields → Ignore (don't crash)
3. Unknown materials → Fallback to default
4. Check `schema_version` → Render with appropriate version handler

---

## FILES CREATED

```
relay/filaments/architecture/
└── 0005_renderspec_v1_locked.md   (4,500 lines - full spec)

relay/fixtures/
├── renderspec_v1_canonical.json   (200 lines - full example)
└── renderspec_v1_minimal.json     (30 lines - simplest valid scene)

relay/
└── RENDERSPEC-V1-LOCKED.md        (this file - summary)
```

---

## NODE KINDS (LOCKED)

| Kind | Purpose | Example |
|------|---------|---------|
| `unit` | SCV agent | Working engineer |
| `filament` | Thread/channel | Work filament with polyline |
| `timebox` | Commit cube | OUTPUT_PROPOSED commit |
| `scar` | Rejection mark | AUTHORITY_DENIED error |
| `anchor` | Filament endpoint | Head/tail markers |
| `globe` | World container | Procedural sphere |
| `label` | Text overlay | Unit name, commit ref |

---

## MATERIAL TAG CONVENTIONS (LOCKED)

### Naming Pattern
```
<domain>_<semantic_state>
```

### Examples
```
scv_idle              - Unit, idle state
scv_working           - Unit, working state
scv_blocked           - Unit, blocked state
encrypted_channel     - Filament, encrypted
public_filament       - Filament, public
delegation_chain      - Filament, authority grants
commit_accepted       - Timebox, accepted
commit_rejected       - Timebox, rejected
scar_authority_denied - Scar, authority error
causal_link_input     - Link, input dependency
world_surface         - Globe material
```

---

## ENDPOINTS (TO BE IMPLEMENTED)

### `/render/world` - Complete Scene
```
GET /api/relay-physics/render/world
```

**Returns:** RenderSpec v1 with globe + all units + all filaments

**Use Case:** StarCraft HUD main view

---

### `/render/commit/:ref` - Forensic Chamber
```
GET /api/relay-physics/render/commit/work.W123@c7
```

**Returns:** RenderSpec v1 with timebox + causal links + author unit

**Use Case:** "Click time cube → open forensic chamber"

---

### `/render/filament/:id` - Thread Detail
```
GET /api/relay-physics/render/filament/work.W123
```

**Returns:** RenderSpec v1 with filament + timeboxes + scars

**Use Case:** Single thread visualization

---

## VALIDATION

### Schema Validation (JSON Schema Draft 2020-12)

```json
{
  "$schema": "http://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "required": ["schema_version", "generated_from", "nodes"],
  "properties": {
    "schema_version": { "const": "relay-render-v1" },
    "generated_from": {
      "type": "object",
      "required": ["timestamp"]
    },
    "nodes": {
      "type": "array",
      "items": { "$ref": "#/$defs/node" }
    }
  }
}
```

### Rust Validation Test

```rust
#[test]
fn test_renderspec_v1_minimal() {
    let json = include_str!("../../fixtures/renderspec_v1_minimal.json");
    let spec: RenderSpec = serde_json::from_str(json).unwrap();
    
    assert_eq!(spec.schema_version, "relay-render-v1");
    assert!(!spec.nodes.is_empty());
    assert_eq!(spec.nodes[0].kind, "unit");
}
```

---

## THREE.JS IMPLEMENTATION GUIDE

### Basic Renderer

```javascript
import * as THREE from 'three';

class RelayRenderer {
  constructor(scene, camera) {
    this.scene = scene;
    this.camera = camera;
    this.materialMap = this.createMaterialMap();
  }
  
  createMaterialMap() {
    return {
      'scv_working': new THREE.MeshStandardMaterial({
        color: 0x00ff00,
        emissive: 0x00aa00,
        emissiveIntensity: 0.5
      }),
      'encrypted_channel': new THREE.MeshStandardMaterial({
        color: 0x8800ff,
        metalness: 0.8
      }),
      'commit_accepted': new THREE.MeshStandardMaterial({
        color: 0x00ff00,
        emissive: 0x00ff00,
        emissiveIntensity: 0.3
      }),
      // ... more materials
    };
  }
  
  async renderScene(renderSpecUrl) {
    const spec = await fetch(renderSpecUrl).then(r => r.json());
    
    if (spec.schema_version !== 'relay-render-v1') {
      console.warn(`Unknown schema version: ${spec.schema_version}`);
    }
    
    for (const node of spec.nodes) {
      this.renderNode(node);
    }
    
    for (const link of spec.links || []) {
      this.renderLink(link);
    }
  }
  
  renderNode(node) {
    let mesh;
    
    switch (node.kind) {
      case 'unit':
        mesh = this.createUnitMesh(node);
        break;
      case 'filament':
        mesh = this.createFilamentMesh(node);
        break;
      case 'timebox':
        mesh = this.createTimeboxMesh(node);
        break;
      default:
        mesh = this.createPlaceholderMesh(node);
    }
    
    // Apply transform
    mesh.position.set(...node.transform.position);
    mesh.quaternion.set(...node.transform.rotation);
    if (node.transform.scale) {
      mesh.scale.set(...node.transform.scale);
    }
    
    this.scene.add(mesh);
  }
  
  createFilamentMesh(node) {
    const { points, radii } = node.geometry;
    const curve = new THREE.CatmullRomCurve3(
      points.map(p => new THREE.Vector3(...p))
    );
    
    const geometry = new THREE.TubeGeometry(
      curve,
      points.length * 10, // segments
      radii ? radii[0] : 0.05, // radius
      8, // radial segments
      false // closed
    );
    
    const material = this.materialMap[node.material] || this.materialMap['default'];
    return new THREE.Mesh(geometry, material);
  }
}
```

---

## WHAT THIS UNLOCKS

### ✅ For Layer 2 Implementation
- Clear contract for `/render/*` endpoints
- Know exactly what JSON to generate
- Golden fixtures for testing

### ✅ For Layer 3 Frontend
- Stable API (no guessing)
- Can build renderer independently
- Multiple frontends (web, desktop, film) use same spec

### ✅ For Visual Language
- Material tags → shaders mapping is Layer 3 responsibility
- Can iterate on visuals without changing Layer 2
- Deterministic (same truth → same scene)

---

## NEXT STEPS

### Option A: Implement `/render/*` Endpoints (Layer 2)
**Effort:** 2-3 days  
**Delivers:** Complete Layer 2→3 API

### Option B: Start Layer 3 Frontend (Three.js)
**Effort:** 1-2 weeks  
**Delivers:** Visual proof-of-concept

### Option C: Both in Parallel
**Team 1:** Implement `/render/world` (backend)  
**Team 2:** Build Three.js renderer with mock data (frontend)

---

**Status:** ✅ **LOCKED AND READY**  
**Reference:** `architecture@c5` for full spec  
**Canonical Example:** `relay/fixtures/renderspec_v1_canonical.json`
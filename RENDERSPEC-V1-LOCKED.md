# RenderSpec v1 - LOCKED SPECIFICATION

**Version:** 1.0  
**Status:** 🔒 LOCKED  
**Date:** 2026-01-29  
**Architecture:** Based on architecture@c5 (RenderSpec v1 Locked)

---

## 🎯 PURPOSE

**RenderSpec v1 is the canonical JSON format for Layer 2 → Layer 3 scene graphs.**

**What it is:**
- A deterministic, versioned contract between Relay Physics (Layer 2) and rendering systems (Layer 3)
- A data structure describing "what to render," not "how to render it"
- A stable, immutable format for a given physics state

**What it is NOT:**
- Not a rendering engine (Layer 3's job)
- Not an artistic playground (no freestyle interpretation)
- Not versioned per-feature (schema version is explicit and locked)
- Not RGB-based (materials are semantic tags, not colors)

---

## 🔒 NON-NEGOTIABLE INVARIANTS

### **Invariant 1: Determinism**
> **Same physics state → Same RenderSpec JSON (byte-for-byte identical)**

**Rules:**
- No `Date.now()` or `rand()` in generation
- No non-deterministic floating point (use stable algorithms)
- No environment-dependent output (same inputs → same JSON everywhere)

**Why:** Replay, audit, and cross-language consistency require determinism.

---

### **Invariant 2: Semantic Materials (No RGB)**
> **Material tags are semantic strings, not RGB hex codes**

**Correct:**
```json
"material": "unit_active"
"material": "filament_causal"
"material": "building_vendor"
```

**FORBIDDEN:**
```json
"material": "#FF5733"
"material": "rgb(255, 87, 51)"
"material": { "r": 255, "g": 87, "b": 51 }
```

**Why:** Layer 3 chooses colors based on theme. Layer 2 provides meaning, not aesthetics.

---

### **Invariant 3: Stable IDs**
> **Node IDs are derived from physics state, not generated randomly**

**Rules:**
- IDs must be reproducible (same entity → same ID)
- Use deterministic hashing (documented algorithm)
- No UUIDs, no random suffixes

**Why:** Diffing, caching, and identity tracking require stable IDs.

---

### **Invariant 4: No Unknown Top-Level Keys**
> **RenderSpec v1 has a fixed set of top-level keys. No freestyle additions.**

**Allowed top-level keys:**
- `schema_version` (required)
- `generated_from` (required)
- `nodes` (required)
- `metadata` (optional, but schema-defined)

**Forbidden:**
- Any other keys (e.g., `custom_data`, `experimental_feature`)

**Why:** Schema drift prevention. If you need new fields, propose RenderSpec v2.

---

### **Invariant 5: Filaments Have Stable Timebox IDs**
> **Filament nodes must include deterministic timebox IDs**

**Rules:**
- Timeboxes derived from commit history (commit index, hash)
- No random timebox generation
- Timebox order stable (same filament → same order)

**Why:** Forensic navigation requires stable temporal anchors.

---

## 📐 SCHEMA STRUCTURE

### **Top-Level Envelope**

```typescript
interface RenderSpec {
  schema_version: "relay-render-v1";  // MUST be exactly this string
  generated_from: GeneratedFrom;      // Provenance
  nodes: Node[];                      // Scene graph (array of nodes)
  metadata?: Metadata;                // Optional metadata
}

interface GeneratedFrom {
  endpoint: string;                   // "/render/world", "/render/filament/:id"
  timestamp: number;                  // Unix ms (when generated)
  commit_ref?: string;                // If rendering specific commit
  filament_id?: string;               // If rendering specific filament
  event_id?: number;                  // If rendering at specific event
}

interface Metadata {
  total_nodes: number;
  total_edges?: number;
  bounding_box?: BoundingBox;
}

interface BoundingBox {
  min: [number, number, number];      // [x, y, z]
  max: [number, number, number];
}
```

**Required fields (top-level):**
- `schema_version` (MUST be `"relay-render-v1"`)
- `generated_from` (MUST include `endpoint` and `timestamp`)
- `nodes` (MUST be an array, may be empty)

---

### **Node Structure**

```typescript
interface Node {
  id: string;                         // Stable, deterministic ID
  kind: NodeKind;                     // Type of node
  transform: Transform;               // Position, rotation, scale
  material: string;                   // Semantic material tag
  props: Record<string, any>;         // Kind-specific properties
  geometry?: Geometry;                // Optional geometry (polylines, etc.)
}

type NodeKind =
  | "globe"
  | "unit"
  | "filament"
  | "timebox"
  | "building"
  | "task"
  | "shipment"
  | string;  // Extensible, but document new kinds

interface Transform {
  position: [number, number, number]; // [x, y, z]
  rotation: [number, number, number]; // [pitch, yaw, roll] in degrees
  scale: [number, number, number];    // [x, y, z]
}

type Geometry =
  | { type: "polyline"; points: [number, number, number][]; color: string; width: number }
  | { type: "mesh"; mesh_ref: string };

// Note: Polyline color is still semantic (e.g., "#00FF00" for "active"),
// but may be RGB for technical reasons (line rendering).
// Prefer semantic where possible.
```

**Required fields (node):**
- `id` (non-empty string)
- `kind` (one of known kinds)
- `transform` (MUST have `position`, `rotation`, `scale`)
- `material` (semantic tag string)
- `props` (object, may be empty `{}`)

**Optional fields (node):**
- `geometry` (for polylines, meshes, etc.)

---

### **Node Kinds (v1)**

#### **1. Globe**
Root node representing the Earth.

```json
{
  "id": "globe",
  "kind": "globe",
  "transform": {
    "position": [0, 0, 0],
    "rotation": [0, 0, 0],
    "scale": [1, 1, 1]
  },
  "material": "globe_base",
  "props": {
    "radius": 6371000,
    "texture_ref": "earth_base"
  }
}
```

---

#### **2. Unit (SCV Worker)**
Represents a human/AI agent in the world.

```json
{
  "id": "unit.alice.123",
  "kind": "unit",
  "transform": {
    "position": [40.7128, -74.0060, 0],
    "rotation": [0, 45, 0],
    "scale": [1, 1, 1]
  },
  "material": "unit_active",
  "props": {
    "unit_id": "unit.alice.123",
    "display_name": "Alice",
    "state": "idle"
  }
}
```

**Material tags:**
- `unit_active` (working)
- `unit_idle` (waiting)
- `unit_moving` (in transit)

---

#### **3. Filament**
Represents a filament (append-only log) as a polyline in space.

```json
{
  "id": "filament.work.W123",
  "kind": "filament",
  "transform": {
    "position": [0, 0, 0],
    "rotation": [0, 0, 0],
    "scale": [1, 1, 1]
  },
  "material": "filament_causal",
  "props": {
    "filament_id": "work.W123",
    "commit_count": 47
  },
  "geometry": {
    "type": "polyline",
    "points": [
      [0, 0, 0],
      [1, 2, 3],
      [2, 4, 6]
    ],
    "color": "#00FF00",
    "width": 2.0
  }
}
```

**Material tags:**
- `filament_causal` (standard causality)
- `filament_authority` (delegated authority)
- `filament_money` (currency transfers)

---

#### **4. Timebox**
Represents a commit in a filament as a 3D box.

```json
{
  "id": "timebox.work.W123.c42",
  "kind": "timebox",
  "transform": {
    "position": [1, 2, 3],
    "rotation": [0, 0, 0],
    "scale": [0.5, 0.5, 0.5]
  },
  "material": "timebox_commit",
  "props": {
    "commit_ref": "work.W123@c42",
    "commit_index": 42,
    "timestamp": 1706486400000,
    "author": "alice",
    "op_type": "TASK_COMPLETED"
  }
}
```

**Material tags:**
- `timebox_commit` (standard commit)
- `timebox_rejected` (rejected by verifier)
- `timebox_authority` (authority grant)

---

#### **5. Building**
Represents a physical building on the globe.

```json
{
  "id": "building.vendor.istore.nyc_001",
  "kind": "building",
  "transform": {
    "position": [40.7614, -73.9776, 0],
    "rotation": [0, 0, 0],
    "scale": [1, 1, 1]
  },
  "material": "building_vendor",
  "props": {
    "building_id": "building.vendor.istore.nyc_001",
    "building_type": "Vendor",
    "name": "Apple Store - Fifth Avenue",
    "status": "Active",
    "geo_anchor": {
      "lat": 40.7614,
      "lon": -73.9776,
      "alt": 0
    }
  }
}
```

**Material tags:**
- `building_vendor` (commercial vendor)
- `building_civic` (government/public)
- `building_logistics` (warehouse/hub)
- `building_partnership` (collaboration HQ)

---

#### **6. Task**
Represents a task in a build queue.

```json
{
  "id": "task.T456",
  "kind": "task",
  "transform": {
    "position": [40.7614, -73.9776, 10],
    "rotation": [0, 0, 0],
    "scale": [1, 1, 1]
  },
  "material": "task_in_progress",
  "props": {
    "task_id": "task.T456",
    "task_type": "PurchaseOrder",
    "state": "Packing",
    "progress_percentage": 45.0,
    "requester_ref": "user.alice",
    "building_ref": "building.vendor.istore.nyc_001"
  }
}
```

**Material tags:**
- `task_queued` (waiting)
- `task_in_progress` (active)
- `task_completed` (done)
- `task_failed` (error)

---

#### **7. Shipment**
Represents a moving logistics unit (drone).

```json
{
  "id": "shipment.S789",
  "kind": "shipment",
  "transform": {
    "position": [40.7500, -73.9700, 100],
    "rotation": [0, 45, 0],
    "scale": [1, 1, 1]
  },
  "material": "shipment_in_transit",
  "props": {
    "shipment_id": "shipment.S789",
    "task_ref": "task.T456",
    "origin_building": "building.vendor.istore.nyc_001",
    "destination_building": "building.logistics.warehouse_42",
    "state": "InTransit",
    "progress_percentage": 67.0,
    "carrier_type": "Drone",
    "route": [
      {"lat": 40.7614, "lon": -73.9776, "alt": 100},
      {"lat": 40.7500, "lon": -73.9700, "alt": 100}
    ]
  },
  "geometry": {
    "type": "polyline",
    "points": [
      [40.7614, -73.9776, 100],
      [40.7500, -73.9700, 100]
    ],
    "color": "#00FF00",
    "width": 2.0
  }
}
```

**Material tags:**
- `shipment_created` (not yet moving)
- `shipment_in_transit` (flying)
- `shipment_arrived` (delivered)
- `shipment_failed` (error)

---

## 🧮 DETERMINISTIC POSITION ALGORITHM

### **Hash-Based Positioning (Cross-Language Stable)**

**For entities without explicit geo coordinates**, use deterministic hashing to assign positions.

**Algorithm:**
```
Input: entity_id (string)
Output: [x, y, z] position (numbers)

1. UTF-8 encode entity_id
2. Compute SHA-256 hash
3. Take first 12 bytes (96 bits)
4. Split into 3 × 32-bit integers (big-endian)
5. Map each integer to normalized range:
   x = (int1 % 10000) / 10000.0 * 360.0 - 180.0  // Longitude [-180, 180]
   y = (int2 % 10000) / 10000.0 * 180.0 - 90.0   // Latitude [-90, 90]
   z = (int3 % 10000) / 10000.0 * 100.0          // Altitude [0, 100]
6. Return [y, x, z]  // Note: [lat, lon, alt] order
```

**Implementation (Rust):**
```rust
use sha2::{Sha256, Digest};

fn deterministic_position(entity_id: &str) -> [f32; 3] {
    let mut hasher = Sha256::new();
    hasher.update(entity_id.as_bytes());
    let hash = hasher.finalize();
    
    let int1 = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);
    let int2 = u32::from_be_bytes([hash[4], hash[5], hash[6], hash[7]]);
    let int3 = u32::from_be_bytes([hash[8], hash[9], hash[10], hash[11]]);
    
    let lon = (int1 % 10000) as f32 / 10000.0 * 360.0 - 180.0;
    let lat = (int2 % 10000) as f32 / 10000.0 * 180.0 - 90.0;
    let alt = (int3 % 10000) as f32 / 10000.0 * 100.0;
    
    [lat, lon, alt]
}
```

**Implementation (TypeScript/JavaScript):**
```typescript
import { createHash } from 'crypto';

function deterministicPosition(entityId: string): [number, number, number] {
  const hash = createHash('sha256').update(entityId, 'utf8').digest();
  
  const int1 = hash.readUInt32BE(0);
  const int2 = hash.readUInt32BE(4);
  const int3 = hash.readUInt32BE(8);
  
  const lon = (int1 % 10000) / 10000.0 * 360.0 - 180.0;
  const lat = (int2 % 10000) / 10000.0 * 180.0 - 90.0;
  const alt = (int3 % 10000) / 10000.0 * 100.0;
  
  return [lat, lon, alt];
}
```

**Test vectors:**
```
Input: "unit.alice.123"
Expected: [lat, lon, alt]  // (compute with reference implementation)

Input: "building.vendor.istore.nyc_001"
Expected: [lat, lon, alt]  // (compute with reference implementation)
```

**Invariant:** Same input → Same output (across Rust, JS, Python, etc.)

---

## 📦 MATERIAL TAG REGISTRY

**All material tags MUST be registered here. No freestyle tags allowed.**

### **Units**
- `unit_active` - Unit is working
- `unit_idle` - Unit is waiting
- `unit_moving` - Unit is in transit
- `unit_selected` - Unit is selected by user

### **Filaments**
- `filament_causal` - Standard causality filament
- `filament_authority` - Delegated authority filament
- `filament_money` - Currency transfer filament
- `filament_commitment` - Commitment/promise filament

### **Timeboxes**
- `timebox_commit` - Standard commit
- `timebox_rejected` - Rejected by verifier
- `timebox_authority` - Authority grant
- `timebox_selected` - Selected for inspection

### **Buildings**
- `building_vendor` - Commercial vendor
- `building_civic` - Government/public building
- `building_logistics` - Warehouse/distribution hub
- `building_partnership` - Collaboration HQ
- `building_community` - Community space

### **Tasks**
- `task_queued` - Waiting in queue
- `task_in_progress` - Currently executing
- `task_completed` - Successfully completed
- `task_failed` - Failed execution

### **Shipments**
- `shipment_created` - Created but not yet moving
- `shipment_in_transit` - Currently flying/moving
- `shipment_arrived` - Delivered successfully
- `shipment_failed` - Delivery failed

### **Globe**
- `globe_base` - Standard Earth representation

**To add new material tags:** Propose via architecture commit or RenderSpec v2.

---

## ✅ VALIDATION RULES

### **Schema Validator (Pseudo-Code)**

```typescript
function validateRenderSpec(spec: any): ValidationResult {
  const errors: string[] = [];
  
  // 1. Check schema_version
  if (spec.schema_version !== "relay-render-v1") {
    errors.push("Invalid schema_version (must be 'relay-render-v1')");
  }
  
  // 2. Check required top-level fields
  if (!spec.generated_from) errors.push("Missing generated_from");
  if (!spec.nodes) errors.push("Missing nodes");
  if (!Array.isArray(spec.nodes)) errors.push("nodes must be array");
  
  // 3. Check generated_from
  if (!spec.generated_from?.endpoint) errors.push("Missing generated_from.endpoint");
  if (typeof spec.generated_from?.timestamp !== "number") errors.push("Invalid generated_from.timestamp");
  
  // 4. Validate each node
  for (const [index, node] of spec.nodes.entries()) {
    if (!node.id) errors.push(`Node ${index}: missing id`);
    if (!node.kind) errors.push(`Node ${index}: missing kind`);
    if (!node.transform) errors.push(`Node ${index}: missing transform`);
    if (!node.material) errors.push(`Node ${index}: missing material`);
    if (!node.props) errors.push(`Node ${index}: missing props`);
    
    // Validate transform
    if (!Array.isArray(node.transform?.position) || node.transform.position.length !== 3) {
      errors.push(`Node ${index}: invalid transform.position`);
    }
    if (!Array.isArray(node.transform?.rotation) || node.transform.rotation.length !== 3) {
      errors.push(`Node ${index}: invalid transform.rotation`);
    }
    if (!Array.isArray(node.transform?.scale) || node.transform.scale.length !== 3) {
      errors.push(`Node ${index}: invalid transform.scale`);
    }
    
    // Check material is registered (optional strict mode)
    if (STRICT_MODE && !isRegisteredMaterial(node.material)) {
      errors.push(`Node ${index}: unknown material tag '${node.material}'`);
    }
  }
  
  // 5. No unknown top-level keys
  const allowedKeys = ["schema_version", "generated_from", "nodes", "metadata"];
  for (const key of Object.keys(spec)) {
    if (!allowedKeys.includes(key)) {
      errors.push(`Unknown top-level key: ${key}`);
    }
  }
  
  return {
    valid: errors.length === 0,
    errors,
  };
}
```

---

## 🧪 CANONICAL FIXTURES

**See separate files:**
- `fixtures/renderspec_v1_canonical.json` - Complete example
- `fixtures/world_minimal.json` - Minimal valid world

**These fixtures are golden references. Any RenderSpec generator MUST pass these tests.**

---

## 🚫 ANTI-PATTERNS (FORBIDDEN)

### **❌ Don't: Patch missing fields in Layer 3**
```typescript
// BAD: Frontend inventing defaults
if (!node.material) {
  node.material = "default";  // FORBIDDEN
}
```

**Why:** Layer 2 is the source of truth. If a field is missing, Layer 2 is broken.

---

### **❌ Don't: Use RGB colors for materials**
```json
{
  "material": "#FF5733"  // FORBIDDEN
}
```

**Why:** Materials are semantic tags. Layer 3 chooses colors.

---

### **❌ Don't: Generate random IDs**
```typescript
// BAD: Non-deterministic IDs
const id = `unit_${Math.random()}`;  // FORBIDDEN
```

**Why:** IDs must be stable for diffing and caching.

---

### **❌ Don't: Use Date.now() in generation**
```typescript
// BAD: Time-dependent output
const position = [Date.now() % 360, 0, 0];  // FORBIDDEN
```

**Why:** Determinism requires same input → same output.

---

### **❌ Don't: Add custom top-level keys**
```json
{
  "schema_version": "relay-render-v1",
  "custom_feature": "experimental"  // FORBIDDEN
}
```

**Why:** Schema drift. Propose RenderSpec v2 if you need new fields.

---

## 📏 SCHEMA EVOLUTION RULES

### **When to create RenderSpec v2:**
- New node kinds (beyond v1 list)
- New required top-level fields
- Breaking changes to node structure
- Removal of fields

### **When to use RenderSpec v1.x (minor):**
- New optional fields (backward compatible)
- New material tags (extend registry)
- Clarifications (non-breaking)

### **Migration path:**
- Layer 2 can emit multiple versions (v1 + v2)
- Layer 3 declares supported versions
- Negotiation via HTTP headers (optional)

---

## 🎯 SUMMARY

**RenderSpec v1 is:**
- ✅ Deterministic (same state → same JSON)
- ✅ Semantic (materials are tags, not RGB)
- ✅ Stable (IDs are hashed, not random)
- ✅ Locked (schema is frozen, no drift)
- ✅ Cross-language (hash algorithm documented)

**RenderSpec v1 is NOT:**
- ❌ A rendering engine (Layer 3's job)
- ❌ Artistic (no freestyle interpretation)
- ❌ Mutable (no patching in Layer 3)
- ❌ RGB-based (semantic tags only)

**If you violate these rules, you're not "being creative" —**  
**You're breaking the Layer 2 → Layer 3 contract.**

---

**Status:** 🔒 LOCKED  
**Version:** 1.0  
**Schema:** `relay-render-v1`  
**Next:** RenderSpec v2 (when needed)

**END OF RENDERSPEC V1 LOCKED SPECIFICATION**

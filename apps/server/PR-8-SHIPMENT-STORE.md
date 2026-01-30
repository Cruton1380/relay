# PR #8: Shipment Store (Drone Layer)

**Status:** 🚧 In Progress  
**Estimated time:** 6-8 hours  
**Architecture references:** `architecture@c9` (Shopping as Production), `architecture@c10` (Buildings as Units)  
**Vision references:** `THE-RELAY-EXPERIENCE.md` Moments 11-13  
**Depends on:** PR #6 (Buildings), PR #7 (Tasks)

---

## 🎯 GOAL

Implement **shipments as moving units** (drones) that visualize logistics on the 3D globe.

**Core insight from architecture@c9:**
> "Shipments = units (drones moving along routes)"

Shipments are NOT tracking numbers.  
Shipments are NOT abstract state.  
Shipments ARE physical drone units flying from origin to destination.

---

## 📦 WHAT IS A SHIPMENT?

**Definition:**
A shipment is a physical logistics unit (drone) that:
- Spawns at an origin building (when task reaches "dispatched" state)
- Follows a deterministic route across the globe
- Carries task payload
- Updates position over time (interpolation)
- Arrives at destination building
- Triggers task completion

**Lifecycle:**
```
Task reaches "Dispatched"
    → Shipment created (drone spawns at origin)
    → Position updates (drone flies route)
    → Shipment arrives (drone lands at destination)
    → Task reaches "Delivered"
```

---

## 🏗️ NEW TRUTH ENTITIES

### 1. Shipment

```rust
pub struct Shipment {
    pub shipment_id: ShipmentId,           // "shipment.drone_001.1738103456"
    pub task_ref: TaskId,                   // Linked task
    pub origin_building: BuildingId,        // Where drone spawned
    pub destination_building: BuildingId,   // Where drone is going
    
    pub current_position: GeoPosition,      // Current lat/lon/alt
    pub route: Vec<GeoPosition>,            // Waypoints (deterministic)
    
    pub state: ShipmentState,               // Created → InTransit → Arrived → Failed
    pub progress_percentage: f32,           // 0.0 to 1.0
    
    pub created_at: u64,                    // Timestamp (spawn time)
    pub estimated_arrival: u64,             // Timestamp (ETA)
    pub actual_arrival: Option<u64>,        // Timestamp (actual landing)
    
    pub carrier_type: CarrierType,          // Drone, Ground, Air (future)
    
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub props: serde_json::Map<String, serde_json::Value>,
}
```

### 2. ShipmentId

```rust
pub struct ShipmentId(pub String);
```

**Format:** `shipment.{carrier_type}.{timestamp}`

**Examples:**
- `shipment.drone.1738103456`
- `shipment.ground.1738103789`

**Determinism rule:** ID generation uses timestamp + carrier type (no randomness).

---

### 3. ShipmentState

```rust
pub enum ShipmentState {
    Created,    // Spawned, not yet moving
    InTransit,  // Currently flying route
    Arrived,    // Landed at destination
    Failed,     // Delivery failed (lost, damaged, etc.)
}
```

**State transitions:**
```
Created → InTransit → Arrived
Created → InTransit → Failed
```

**Invalid transitions:**
- Arrived → InTransit (can't undeliver)
- Failed → Arrived (can't recover from failure without new shipment)

---

### 4. GeoPosition

```rust
pub struct GeoPosition {
    pub lat: f64,        // Latitude (-90 to 90)
    pub lon: f64,        // Longitude (-180 to 180)
    pub alt: f64,        // Altitude in meters (0 for ground)
}
```

**Used for:**
- Building geo_anchor (static position)
- Shipment current_position (dynamic, interpolated)
- Route waypoints (deterministic path)

---

### 5. CarrierType

```rust
pub enum CarrierType {
    Drone,      // Air delivery (default)
    Ground,     // Truck/car delivery
    Air,        // Plane delivery (future)
}
```

**MVP:** Only `Drone` implemented.

**Future:** Different carrier types have different speeds, altitudes, and route constraints.

---

## 🔄 NEW EVENTS

### 1. SHIPMENT_CREATED

```rust
ShipmentCreated {
    shipment: Shipment,
    triggered_by_task: TaskId,
}
```

**When fired:**
- Task progresses to `TaskState::Dispatched`
- Building has capacity to spawn drone
- Route calculated successfully

**What happens:**
- Shipment appended to event log
- ShipmentStore creates new shipment
- Task updated with `shipment_ref` in props
- SSE emits to all clients

---

### 2. SHIPMENT_POSITION_UPDATED

```rust
ShipmentPositionUpdated {
    shipment_id: ShipmentId,
    new_position: GeoPosition,
    progress_percentage: f32,
}
```

**When fired:**
- Time-based position calculation (every 5 seconds or on request)
- Frontend requests `/render/world` (recalculates all shipment positions)

**Determinism:**
- Position derived from: `created_at`, `estimated_arrival`, `route`, `current_time`
- Same inputs → same position (no randomness)

**Note:** This event is optional. Position can be recalculated on-demand from shipment data.

---

### 3. SHIPMENT_ARRIVED

```rust
ShipmentArrived {
    shipment_id: ShipmentId,
    actual_arrival: u64,
}
```

**When fired:**
- Current time >= estimated_arrival
- Shipment reaches destination building

**What happens:**
- Shipment state → `Arrived`
- Task progresses to `TaskState::Delivered`
- Escrow releases
- SSE emits completion event

---

### 4. SHIPMENT_FAILED

```rust
ShipmentFailed {
    shipment_id: ShipmentId,
    reason: String,
}
```

**When fired:**
- Shipment lost/damaged
- Building unavailable at destination
- Manual cancellation

**What happens:**
- Shipment state → `Failed`
- Task state → `Failed`
- Task `failure_count` increments
- Escrow handling (depends on failure reason)

---

## 🌐 POSITION INTERPOLATION

**The key algorithm: deterministic position calculation**

```rust
/// Calculate current position of shipment based on time elapsed
pub fn calculate_current_position(
    shipment: &Shipment,
    current_time: u64,
) -> GeoPosition {
    let elapsed = current_time.saturating_sub(shipment.created_at);
    let total_duration = shipment.estimated_arrival.saturating_sub(shipment.created_at);
    
    if total_duration == 0 {
        return shipment.route[0].clone(); // Not moving yet
    }
    
    let progress = (elapsed as f64 / total_duration as f64).min(1.0);
    
    // Linear interpolation along route
    let total_segments = shipment.route.len() - 1;
    if total_segments == 0 {
        return shipment.route[0].clone();
    }
    
    let segment_progress = progress * (total_segments as f64);
    let segment_index = segment_progress.floor() as usize;
    let within_segment = segment_progress - segment_index as f64;
    
    if segment_index >= total_segments {
        return shipment.route.last().unwrap().clone(); // Arrived
    }
    
    let start = &shipment.route[segment_index];
    let end = &shipment.route[segment_index + 1];
    
    GeoPosition {
        lat: start.lat + (end.lat - start.lat) * within_segment,
        lon: start.lon + (end.lon - start.lon) * within_segment,
        alt: start.alt + (end.alt - start.alt) * within_segment,
    }
}
```

**Key properties:**
- ✅ Deterministic (same time → same position)
- ✅ No server-side timer (calculated on-demand)
- ✅ Client can extrapolate between updates
- ✅ Supports multi-segment routes
- ✅ Smooth linear interpolation

---

## 🛣️ ROUTE GENERATION

**MVP approach: Simple great circle route**

```rust
/// Generate simple 2-waypoint route (origin → destination)
pub fn generate_simple_route(
    origin: &GeoAnchor,
    destination: &GeoAnchor,
    carrier_type: CarrierType,
) -> Vec<GeoPosition> {
    let alt = match carrier_type {
        CarrierType::Drone => 100.0,  // 100m altitude
        CarrierType::Ground => 0.0,
        CarrierType::Air => 10000.0,  // 10km altitude
    };
    
    vec![
        GeoPosition {
            lat: origin.lat,
            lon: origin.lon,
            alt,
        },
        GeoPosition {
            lat: destination.lat,
            lon: destination.lon,
            alt,
        },
    ]
}
```

**Future enhancements:**
- Multi-waypoint routes (avoid obstacles)
- Great circle vs rhumb line calculation
- Terrain avoidance
- Weather-based routing

---

## 📡 API ENDPOINTS

### 1. GET /api/relay-physics/shipments

**Query params:**
- `task_ref` - Filter by linked task
- `origin_building` - Filter by origin
- `destination_building` - Filter by destination
- `state` - Filter by state (created, in_transit, arrived, failed)

**Response:**
```json
[
  {
    "shipment_id": "shipment.drone.1738103456",
    "task_ref": "task.apple_store.iphone_15_pro.1738103400",
    "origin_building": "building.apple_store.nyc_001",
    "destination_building": "building.alice_home.001",
    "current_position": {
      "lat": 40.7589,
      "lon": -73.9851,
      "alt": 100.0
    },
    "route": [
      { "lat": 40.7628, "lon": -73.9741, "alt": 100.0 },
      { "lat": 40.7589, "lon": -73.9851, "alt": 100.0 }
    ],
    "state": "in_transit",
    "progress_percentage": 0.65,
    "created_at": 1738103456,
    "estimated_arrival": 1738104056,
    "carrier_type": "drone"
  }
]
```

**Cache:** `private, max-age=2` (very short - positions update frequently)

---

### 2. POST /api/relay-physics/shipments

**Body:**
```json
{
  "task_ref": "task.apple_store.iphone_15_pro.1738103400",
  "carrier_type": "drone"
}
```

**Logic:**
1. Verify task exists and is in "Dispatched" state
2. Get origin building from task.building_ref
3. Get destination building from task.requester_ref (user's current location)
4. Generate route
5. Calculate ETA (based on distance + carrier speed)
6. Create shipment
7. Emit `SHIPMENT_CREATED` event
8. Update task with `shipment_ref` in props

**Response:**
```json
{
  "result": "ShipmentCreated",
  "event_id": 125,
  "shipment": { ... }
}
```

**Error cases:**
- `TASK_NOT_FOUND` - task_ref invalid
- `TASK_NOT_READY` - task not in Dispatched state
- `BUILDING_NOT_FOUND` - origin or destination invalid
- `SHIPMENT_CREATION_FAILED` - store rejected

---

### 3. GET /api/relay-physics/shipments/:id

**Response:**
```json
{
  "shipment_id": "shipment.drone.1738103456",
  "task_ref": "task.apple_store.iphone_15_pro.1738103400",
  "current_position": { ... },
  "state": "in_transit",
  "progress_percentage": 0.65,
  ...
}
```

**Note:** `current_position` is calculated on-demand based on current server time.

---

### 4. PATCH /api/relay-physics/shipments/:id/arrive

**Body:** (empty or optional arrival time)

**Logic:**
1. Verify shipment exists
2. Mark shipment as Arrived
3. Update linked task to Delivered
4. Emit events
5. Trigger escrow release (future)

**Response:**
```json
{
  "result": "ShipmentArrived",
  "event_id": 126,
  "shipment": { ... }
}
```

---

## 🎨 RENDERSPEC INTEGRATION

### Shipment Node

```json
{
  "id": "shipment.drone.1738103456",
  "kind": "shipment",
  "transform": {
    "position": [40.7589, -73.9851, 100.0],
    "rotation": [0, 45, 0],
    "scale": [1, 1, 1]
  },
  "material": "shipment_in_transit",
  "geometry": {
    "polyline": {
      "points": [
        [40.7628, -73.9741, 100.0],
        [40.7589, -73.9851, 100.0]
      ],
      "color": "#00FFFF",
      "width": 2.0
    }
  },
  "props": {
    "shipment_id": "shipment.drone.1738103456",
    "task_ref": "task.apple_store.iphone_15_pro.1738103400",
    "state": "in_transit",
    "progress_percentage": 0.65,
    "carrier_type": "drone",
    "route": [...],
    "estimated_arrival": 1738104056
  }
}
```

**Material tags:**
- `shipment_created` - Just spawned (pulsing)
- `shipment_in_transit` - Moving (animated)
- `shipment_arrived` - Landed (fading out)
- `shipment_failed` - Failed (red, stopped)

**Geometry:**
- Route visualized as polyline (thin arc connecting buildings)
- Drone position at current interpolated point
- Optional trail (fading line behind drone)

---

## 🏪 SHIPMENTSTORE INTERFACE

```rust
pub struct ShipmentStore {
    shipments: HashMap<ShipmentId, Shipment>,
}

impl ShipmentStore {
    pub fn new() -> Self;
    
    // CRUD operations
    pub fn create(&mut self, shipment: Shipment) -> Result<(), String>;
    pub fn get(&self, id: &ShipmentId) -> Option<&Shipment>;
    pub fn update_position(&mut self, id: &ShipmentId, position: GeoPosition, progress: f32) -> Result<(), String>;
    pub fn arrive(&mut self, id: &ShipmentId, arrival_time: u64) -> Result<(), String>;
    pub fn fail(&mut self, id: &ShipmentId, reason: String) -> Result<(), String>;
    
    // Query operations
    pub fn list_all(&self) -> Vec<Shipment>;
    pub fn list_by_task(&self, task_id: &TaskId) -> Vec<Shipment>;
    pub fn list_by_origin(&self, building_id: &BuildingId) -> Vec<Shipment>;
    pub fn list_by_destination(&self, building_id: &BuildingId) -> Vec<Shipment>;
    pub fn list_by_state(&self, state: ShipmentState) -> Vec<Shipment>;
    pub fn list_active(&self) -> Vec<Shipment>; // Created or InTransit
    
    // Replay for determinism
    pub fn replay_from_events(&mut self, events: &[EventLogEntry]);
    
    // Position calculation
    pub fn calculate_current_position(&self, id: &ShipmentId, current_time: u64) -> Option<GeoPosition>;
}
```

---

## 🔗 INTEGRATION WITH TASKS

### When Task Progresses to "Dispatched"

**In `progress_task_handler` (main_integration.rs):**

```rust
// After task.progress() succeeds
if to_state == TaskState::Dispatched {
    // Auto-create shipment
    let shipment = create_shipment_for_task(&task, &building_store).await?;
    
    // Emit SHIPMENT_CREATED event
    let event = RelayEvent::ShipmentCreated {
        shipment: shipment.clone(),
        triggered_by_task: task.task_id.clone(),
    };
    event_log.append(event.clone())?;
    events.emit_with_id(event_id, event);
    
    // Update task with shipment_ref
    task.props.insert("shipment_ref".to_string(), json!(shipment.shipment_id.as_str()));
}
```

---

### When Shipment Arrives

**In `arrive_shipment_handler` (main_integration.rs):**

```rust
// After shipment.arrive() succeeds
if let Some(task_ref) = &shipment.task_ref {
    // Progress linked task to Delivered
    task_store.complete(task_ref)?;
    
    // Emit TaskCompleted event
    let event = RelayEvent::TaskCompleted {
        task_id: task_ref.clone(),
    };
    event_log.append(event.clone())?;
    events.emit_with_id(event_id, event);
}
```

---

## 🧪 TESTING STRATEGY

### Unit Tests (5)

1. `test_create_shipment` - Basic creation
2. `test_calculate_position` - Interpolation accuracy
3. `test_shipment_lifecycle` - Created → InTransit → Arrived
4. `test_invalid_state_transition` - Arrived → InTransit fails
5. `test_replay_from_events` - Deterministic reconstruction

### Integration Tests (Manual)

1. Create task → Progress to Dispatched → Verify shipment auto-created
2. List shipments → Verify position updates over time
3. Arrive shipment → Verify task completes
4. Restart server → Verify shipments replayed correctly
5. `/render/world` → Verify shipment nodes present with correct positions

---

## 📊 SUCCESS CRITERIA

| Criterion | Status |
|-----------|--------|
| Shipments auto-created when task dispatched | ⏳ TODO |
| Position interpolation deterministic | ⏳ TODO |
| `/render/world` includes shipment nodes | ⏳ TODO |
| Shipment arrival triggers task completion | ⏳ TODO |
| Replay reconstructs shipments | ⏳ TODO |
| Route visualization in RenderSpec | ⏳ TODO |
| Position updates without server timer | ⏳ TODO |
| Tests pass | ⏳ TODO |

---

## 📁 FILE STRUCTURE

### New Files
```
apps/server/src/relay_physics/
├── shipment_store.rs        (ShipmentStore implementation)
└── route_generator.rs       (Route calculation utilities)
```

### Modified Files
```
apps/server/src/relay_physics/
├── types.rs                 (+ Shipment types)
├── events.rs                (+ Shipment events)
├── mod.rs                   (+ shipment_store module)
├── renderspec_generator.rs  (+ create_shipment_node)
└── main_integration.rs      (+ ShipmentStore, endpoints, task integration)
```

---

## 🎯 IMPLEMENTATION ORDER

1. ✅ Create this spec document
2. ⏳ Add Shipment types to `types.rs`
3. ⏳ Implement `ShipmentStore` in `shipment_store.rs`
4. ⏳ Add Shipment events to `events.rs`
5. ⏳ Create `route_generator.rs` utilities
6. ⏳ Update `renderspec_generator.rs` with shipment nodes
7. ⏳ Add shipment endpoints to `main_integration.rs`
8. ⏳ Integrate with task progression (auto-create on dispatch)
9. ⏳ Write unit tests
10. ⏳ Test manually

**Estimated time per step:** 30-60 minutes each  
**Total:** 6-8 hours

---

## 🔮 WHAT THIS ENABLES

**From THE-RELAY-EXPERIENCE.md:**

**Moment 11: Watching Logistics Happen (This is the magic)**
> On the globe: A drone unit spawns at the iStore building. A glowing polyline route appears. The drone lifts off and starts moving toward you.

**Backend:** ✅ FULLY SPECIFIED
- Shipment spawns when task dispatched
- Route calculated and stored
- Position interpolated over time
- RenderSpec outputs drone node + route polyline

**Moment 12: While You Wait: Life Continues**
> While the drone is en route, your agent finishes a task, a vote ticks closer to resolution, a peer sends a message.

**Backend:** ✅ SUPPORTED
- Shipments update independently (no blocking)
- Position calculation on-demand (no timer)
- SSE pushes updates to clients

**Moment 13: Delivery and Completion**
> The drone arrives. It descends. The task completes. Escrow releases. The filament closes cleanly.

**Backend:** ✅ FULLY SPECIFIED
- Shipment arrive endpoint
- Task completion linkage
- Event log closure
- RenderSpec reflects completion

---

## 💬 THE VISION

**What we're building:**

A world where logistics is NOT:
- ❌ A tracking number you paste into a website
- ❌ An abstract "order status" field
- ❌ A notification that something happened elsewhere

But instead:
- ✅ A physical unit you can see on the globe
- ✅ A route you can inspect
- ✅ A position that updates in real-time
- ✅ A delivery you can watch happen

**This is StarCraft's Overlord flying minerals across the map.**

**This is coordination made visceral.**

---

## 🚀 READY TO IMPLEMENT

Say: **"Implement PR #8 - Shipment types"** to start step 2.

---

**END OF PR #8 SPECIFICATION**

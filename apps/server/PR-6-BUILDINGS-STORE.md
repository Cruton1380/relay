# PR #6: Buildings Store (Physical World Entities)

**Date:** 2026-01-28  
**Status:** IN PROGRESS  
**Reference:** `architecture@c9` - Personal HUD + Physical Globe

---

## GOAL

Add deterministic, replayable store of physical buildings on the globe and output them through RenderSpec.

**Key principle:** Buildings are first-class world entities with physical locations.

---

## NEW TRUTH ENTITIES

### Building

**Fields:**
```rust
pub struct Building {
    pub building_id: BuildingId,          // Stable identifier
    pub building_type: BuildingType,      // vendor | partnership | civic | logistics | community
    pub geo_anchor: GeoAnchor,            // Physical location (lat/lon/altitude)
    pub owner_ref: Option<UnitId>,        // Optional owner
    pub catalog: Vec<CatalogItem>,        // Products/services (if vendor)
    pub status: BuildingStatus,           // active | disabled
    pub props: serde_json::Map<String, JsonValue>, // Free-form metadata
}

pub struct BuildingId(String);

pub enum BuildingType {
    Vendor,
    Partnership,
    Civic,
    Logistics,
    Community,
}

pub struct GeoAnchor {
    pub lat: f64,
    pub lng: f64,
    pub altitude: Option<f64>, // Meters above sea level
}

pub enum BuildingStatus {
    Active,
    Disabled, // Anti-spoof flag (future)
}

pub struct CatalogItem {
    pub product_id: String,
    pub name: String,
    pub price: f64, // USD (or token amount)
    pub production_time_seconds: u64,
    pub delivery_time_seconds: u64,
    pub in_stock: bool,
    pub metadata: serde_json::Map<String, JsonValue>,
}
```

---

## NEW EVENTS (APPEND-ONLY)

### 1. BUILDING_REGISTERED

**When:** New building added to world  
**Payload:**
```json
{
  "building_id": "building.apple_store.nyc_001",
  "building_type": "vendor",
  "geo_anchor": {
    "lat": 40.7637,
    "lng": -73.9722,
    "altitude": 10.0
  },
  "owner_ref": "unit.apple.admin",
  "catalog": [
    {
      "product_id": "iphone_15_pro",
      "name": "iPhone 15 Pro",
      "price": 999.00,
      "production_time_seconds": 3600,
      "delivery_time_seconds": 7200,
      "in_stock": true
    }
  ],
  "status": "active"
}
```

**Causality:** No prerequisites (bootstrap event)

---

### 2. BUILDING_UPDATED

**When:** Building metadata changed  
**Payload:**
```json
{
  "building_id": "building.apple_store.nyc_001",
  "updates": {
    "status": "disabled",
    "catalog": [...]  // Updated catalog
  }
}
```

**Causality:** References previous `BUILDING_REGISTERED` or `BUILDING_UPDATED`

---

### 3. BUILDING_CATALOG_UPDATED

**When:** Catalog changed (products added/removed/modified)  
**Payload:**
```json
{
  "building_id": "building.apple_store.nyc_001",
  "catalog": [
    {
      "product_id": "iphone_16_pro",
      "name": "iPhone 16 Pro",
      "price": 1099.00,
      ...
    }
  ]
}
```

---

## API ENDPOINTS

### Truth Endpoints

#### GET /api/relay-physics/buildings

**Purpose:** List all buildings

**Response:**
```json
[
  {
    "building_id": "building.apple_store.nyc_001",
    "building_type": "vendor",
    "geo_anchor": {
      "lat": 40.7637,
      "lng": -73.9722,
      "altitude": 10.0
    },
    "owner_ref": "unit.apple.admin",
    "catalog": [...],
    "status": "active",
    "props": {}
  }
]
```

**Cache:** `Cache-Control: private, max-age=60` (derived state)

---

#### POST /api/relay-physics/buildings

**Purpose:** Register new building

**Request:**
```json
{
  "building_id": "building.acme_hq.sf",
  "building_type": "partnership",
  "geo_anchor": {
    "lat": 37.7749,
    "lng": -122.4194
  },
  "owner_ref": "unit.acme.admin",
  "catalog": [],
  "props": {
    "company_name": "ACME Corp"
  }
}
```

**Response:**
```json
{
  "result": "BuildingRegistered",
  "event_id": 42,
  "building": {...}
}
```

**Error cases:**
- Building ID already exists → `400 BUILDING_ALREADY_EXISTS`
- Invalid geo_anchor → `400 INVALID_GEO_ANCHOR`
- Unauthorized → `403 AUTHORITY_DENIED`

---

### Render Endpoint

#### GET /api/relay-physics/render/world (UPDATED)

**Purpose:** Now includes building nodes

**Response:**
```json
{
  "schema_version": "relay-render-v1",
  "generated_from": {
    "timestamp": "2026-01-28T10:30:00Z"
  },
  "nodes": [
    {
      "id": "globe.world",
      "kind": "globe",
      ...
    },
    {
      "id": "building.apple_store.nyc_001",
      "kind": "building",
      "transform": {
        "position": [40.7637, -73.9722, 10.0],
        "rotation": [0, 0, 0, 1],
        "scale": [1, 1, 1]
      },
      "material": "vendor_building",
      "props": {
        "building_type": "vendor",
        "building_id": "building.apple_store.nyc_001",
        "name": "Apple Fifth Avenue",
        "open_status": "open",
        "catalog": [
          {
            "product_id": "iphone_15_pro",
            "name": "iPhone 15 Pro",
            "price": 999.00,
            "production_time_seconds": 3600,
            "delivery_time_seconds": 7200,
            "in_stock": true
          }
        ]
      }
    },
    {
      "id": "unit.alice.001",
      "kind": "unit",
      ...
    }
  ]
}
```

---

## DETERMINISM RULES

### Position Derivation

**Primary source:** `geo_anchor` (lat/lon/altitude)

**Fallback:** Deterministic hash from `building_id` (only if geo_anchor missing)

**Implementation:**
```rust
fn building_transform(building: &Building) -> Transform {
    // Primary: Use geo_anchor if available
    if let Some(geo) = &building.geo_anchor {
        Transform {
            position: [geo.lat as f32, geo.lng as f32, geo.altitude.unwrap_or(0.0) as f32],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    } else {
        // Fallback: Hash-based position (for buildings without geo)
        deterministic_position_from_id(&building.building_id.0)
    }
}
```

**Key property:** Same `geo_anchor` → Same position (deterministic)

---

### Catalog Ordering

**Rule:** Catalog items are ordered by `product_id` (alphabetical)

**Why:** Ensures consistent ordering in UI (deterministic)

**Implementation:**
```rust
fn sort_catalog(catalog: &mut Vec<CatalogItem>) {
    catalog.sort_by(|a, b| a.product_id.cmp(&b.product_id));
}
```

---

## SEEDING STRATEGY (OPTION 1)

### Seed File: `relay/fixtures/buildings_seed.json`

**Contents:** Static building definitions for demo world

**Example:**
```json
[
  {
    "building_id": "building.apple_store.nyc_001",
    "building_type": "vendor",
    "geo_anchor": {
      "lat": 40.7637,
      "lng": -73.9722,
      "altitude": 10.0
    },
    "owner_ref": "unit.apple.admin",
    "catalog": [
      {
        "product_id": "iphone_15_pro",
        "name": "iPhone 15 Pro",
        "price": 999.00,
        "production_time_seconds": 3600,
        "delivery_time_seconds": 7200,
        "in_stock": true,
        "metadata": {
          "category": "electronics",
          "sku": "MLXXX3LL/A"
        }
      }
    ],
    "status": "active",
    "props": {
      "name": "Apple Fifth Avenue",
      "address": "767 5th Ave, New York, NY 10153"
    }
  },
  {
    "building_id": "building.acme_hq.sf",
    "building_type": "partnership",
    "geo_anchor": {
      "lat": 37.7749,
      "lng": -122.4194,
      "altitude": 50.0
    },
    "owner_ref": "unit.acme.admin",
    "catalog": [],
    "status": "active",
    "props": {
      "company_name": "ACME Corp",
      "partnerships": ["supplier.A", "supplier.B"]
    }
  }
]
```

---

### Seed Loading (On First Boot)

**File:** `apps/server/src/relay_physics/building_seeder.rs`

**Implementation:**
```rust
pub fn seed_buildings_if_empty(
    building_store: &mut BuildingStore,
    event_log: &Arc<Mutex<EventLog>>,
    event_bus: &EventBus,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if buildings already exist
    if !building_store.is_empty() {
        return Ok(()); // Already seeded
    }
    
    // Load seed file
    let seed_path = "relay/fixtures/buildings_seed.json";
    let seed_data = std::fs::read_to_string(seed_path)?;
    let buildings: Vec<BuildingSeed> = serde_json::from_str(&seed_data)?;
    
    // Write each building as BUILDING_REGISTERED event
    for building_seed in buildings {
        let event = RelayEvent::BuildingRegistered {
            building: building_seed.into_building(),
        };
        
        // Persist to event log
        let mut log = event_log.lock().unwrap();
        let event_id = log.append(event.clone())?;
        
        // Emit to event bus
        event_bus.emit_with_id(event_id, event);
        
        // Apply to building store
        building_store.register(building_seed.into_building())?;
    }
    
    Ok(())
}
```

**Key property:** Seeds are written as events → Replay reconstructs same world ✅

---

## FILE STRUCTURE

### New Files

```
apps/server/src/relay_physics/
├── building_store.rs           (NEW - Building storage + operations)
├── building_seeder.rs          (NEW - Seed loader)
└── types.rs                    (UPDATED - Add Building types)

relay/fixtures/
└── buildings_seed.json         (NEW - Demo world buildings)
```

### Updated Files

```
apps/server/src/relay_physics/
├── mod.rs                      (UPDATED - Add building modules)
├── events.rs                   (UPDATED - Add BuildingRegistered event)
├── renderspec_generator.rs     (UPDATED - Add building nodes)
└── main_integration.rs         (UPDATED - Add building endpoints)
```

---

## TESTING

### Unit Tests

#### Test 1: Seed Buildings on First Boot
```rust
#[test]
fn test_seed_buildings_first_boot() {
    let mut building_store = BuildingStore::new();
    let event_log = Arc::new(Mutex::new(EventLog::open("test.jsonl").unwrap()));
    let event_bus = EventBus::new(1024);
    
    seed_buildings_if_empty(&mut building_store, &event_log, &event_bus).unwrap();
    
    let buildings = building_store.list_all();
    assert!(!buildings.is_empty());
    
    // Verify event log has BUILDING_REGISTERED events
    let events = event_log.lock().unwrap().load_all().unwrap();
    assert!(events.iter().any(|e| matches!(e.event, RelayEvent::BuildingRegistered { .. })));
}
```

---

#### Test 2: Replay Reconstructs Same Buildings
```rust
#[test]
fn test_replay_buildings() {
    // Seed buildings
    let mut building_store1 = BuildingStore::new();
    seed_buildings_if_empty(&mut building_store1, ...).unwrap();
    let buildings1 = building_store1.list_all();
    
    // Clear store and replay from event log
    let mut building_store2 = BuildingStore::new();
    let events = event_log.lock().unwrap().load_all().unwrap();
    building_store2.replay_from_events(&events);
    let buildings2 = building_store2.list_all();
    
    // Same buildings after replay
    assert_eq!(buildings1.len(), buildings2.len());
    for (b1, b2) in buildings1.iter().zip(buildings2.iter()) {
        assert_eq!(b1.building_id, b2.building_id);
        assert_eq!(b1.geo_anchor.lat, b2.geo_anchor.lat);
        assert_eq!(b1.geo_anchor.lng, b2.geo_anchor.lng);
    }
}
```

---

#### Test 3: RenderSpec Includes Buildings
```rust
#[test]
fn test_renderspec_includes_buildings() {
    let mut building_store = BuildingStore::new();
    seed_buildings_if_empty(&mut building_store, ...).unwrap();
    
    let buildings = building_store.list_all();
    let units = vec![];
    let filaments = HashMap::new();
    
    let spec = generate_world_scene(&units, &filaments, &buildings);
    
    // Check building nodes exist
    let building_nodes: Vec<_> = spec.nodes.iter()
        .filter(|n| n.kind == "building")
        .collect();
    
    assert!(!building_nodes.is_empty());
    
    // Check building has catalog in props
    let vendor_node = building_nodes.iter()
        .find(|n| n.props.get("building_type").unwrap() == "vendor")
        .unwrap();
    
    assert!(vendor_node.props.contains_key("catalog"));
}
```

---

### Manual Tests

#### Manual Test 1: List Buildings
```bash
cargo run

# In another terminal:
curl http://localhost:3002/api/relay-physics/buildings | jq .

# Expected: Array of buildings (from seed file)
```

---

#### Manual Test 2: Render World with Buildings
```bash
curl http://localhost:3002/api/relay-physics/render/world | jq '.nodes[] | select(.kind=="building")'

# Expected: Building nodes with geo_anchor, catalog, etc.
```

---

#### Manual Test 3: Register New Building
```bash
curl -X POST http://localhost:3002/api/relay-physics/buildings \
  -H "Content-Type: application/json" \
  -d '{
    "building_id": "building.test.001",
    "building_type": "vendor",
    "geo_anchor": {
      "lat": 34.0522,
      "lng": -118.2437
    },
    "owner_ref": "unit.test.admin",
    "catalog": [],
    "props": {}
  }'

# Expected: BuildingRegistered event + building in list
```

---

## IMPLEMENTATION CHECKLIST

### Step 1: Types
- [ ] Add `Building`, `BuildingId`, `BuildingType`, `GeoAnchor`, `BuildingStatus`, `CatalogItem` to `types.rs`

### Step 2: Events
- [ ] Add `RelayEvent::BuildingRegistered` to `events.rs`
- [ ] Add `RelayEvent::BuildingUpdated` to `events.rs`

### Step 3: Building Store
- [ ] Create `building_store.rs`
- [ ] Implement `register()`, `update()`, `list_all()`, `get()`, `is_empty()`
- [ ] Implement `replay_from_events()`

### Step 4: Seeder
- [ ] Create `building_seeder.rs`
- [ ] Implement `seed_buildings_if_empty()`
- [ ] Create `relay/fixtures/buildings_seed.json`

### Step 5: RenderSpec Generator
- [ ] Update `renderspec_generator.rs`
- [ ] Add `create_building_node()`
- [ ] Update `generate_world_scene()` to include buildings

### Step 6: API Endpoints
- [ ] Add `GET /buildings` handler
- [ ] Add `POST /buildings` handler
- [ ] Update `relay_router()` in `main_integration.rs`

### Step 7: Init Integration
- [ ] Update `init_relay_physics_state()` to call seeder
- [ ] Add `building_store` to `AppState`

### Step 8: Tests
- [ ] Write unit tests (3 tests)
- [ ] Run manual tests (3 tests)

---

## SUCCESS CRITERIA

✅ Buildings can be seeded from JSON file  
✅ Buildings are written to event log as `BUILDING_REGISTERED`  
✅ Replay reconstructs same buildings (deterministic)  
✅ `GET /buildings` returns all buildings  
✅ `POST /buildings` registers new building  
✅ `/render/world` includes building nodes with catalog in props  
✅ Building positions derived from `geo_anchor` (deterministic)  
✅ Tests pass (unit + manual)

---

## NEXT STEPS (AFTER PR #6)

### PR #7: Task Store
- Tasks represent build queue entries
- `TASK_CREATE`, `TASK_PROGRESS`, `TASK_COMPLETE` events
- `/tasks` endpoints

### PR #8: Shipment Store
- Shipments represent in-transit drones
- `SHIPMENT_CREATE`, `SHIPMENT_UPDATE_POSITION`, `SHIPMENT_ARRIVE` events
- Route interpolation (position calculation)
- `/shipments` endpoints

---

**Status:** READY TO IMPLEMENT  
**Reference:** `architecture@c9`  
**Estimate:** 4-6 hours

---

**Bottom line:** Buildings are the foundation. Once buildings exist, shopping/tasks/shipments become inevitable.

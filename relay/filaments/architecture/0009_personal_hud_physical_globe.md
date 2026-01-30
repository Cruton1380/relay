# COMMIT 9: Personal HUD + Physical Globe (StarCraft Model)

**Filament:** `architecture`  
**Commit Index:** 9  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** UX_ARCHITECTURE

---

## INVARIANT LOCKED (NON-NEGOTIABLE)

**The HUD is personal. The globe is shared.**

The HUD shows **my state in the world** (StarCraft player view).  
The globe shows **the world itself** (StarCraft map with buildings).

---

## PART 1: THE CORE DISTINCTION

### HUD = Personal State (My View)

**What the HUD shows:**
- **My resources** (legitimacy, commitments, escrow, time windows, disputes)
- **My active tasks** (build queue / production queue)
- **My units** (agents I control / have delegated authority to)
- **My supply/capacity** (commitment capacity, active obligations)
- **My notifications** (incoming requests, disputes, deliveries)

**What the HUD does NOT show:**
- ❌ Global analytics (total world resources)
- ❌ Other players' resources
- ❌ Aggregate statistics
- ❌ World-level dashboards

**Analogy:** StarCraft HUD
- Top-right: My minerals, my gas, my supply
- Bottom: My selected units, my build queue
- Minimap: Where I am in the world

**Key principle:** The HUD is always filtered through **viewer identity**.

---

### Globe = Shared Map (World View)

**What the globe shows:**
- **Buildings** (vendors, partnerships, civic structures, logistics hubs)
- **Points of interest** (stores, warehouses, factories, verification points)
- **Other players' units** (visible agents/SCVs)
- **Shipments in transit** (drone delivery units flying)
- **Filaments** (commitment threads between entities)

**What the globe does NOT show:**
- ❌ My personal resources (that's in the HUD)
- ❌ My build queue (that's in the HUD)
- ❌ My notifications (that's in the HUD)

**Analogy:** StarCraft map
- Buildings are structures (barracks, factories, command centers)
- Units move around (SCVs, drones, shipments)
- Fog of war (areas you can't see)

**Key principle:** The globe is the **physical world**, not a personal dashboard.

---

## PART 2: BUILDINGS ON THE GLOBE (PHYSICAL ENTITIES)

### Building Types (StarCraft Structures)

#### 1. Vendor Buildings (Stores / Warehouses / Factories)

**What they are:**
- Physical stores (Apple Store, Target, local shops)
- Warehouses (Amazon fulfillment centers)
- Factories (manufacturing plants)

**What they do:**
- Sell products (unit production)
- Ship inventory (create shipment tasks)
- Accept returns (reverse logistics)

**Visual representation:**
- Building icon on globe (scaled by size)
- Glow/pulse when active
- Click to open "production panel"

**RenderSpec node:**
```json
{
  "id": "building.apple_store.nyc_001",
  "kind": "building",
  "transform": {
    "position": [lat, lng, altitude],
    "rotation": [0, 0, 0, 1],
    "scale": [1.0, 1.0, 1.0]
  },
  "material": "vendor_building",
  "props": {
    "building_type": "vendor",
    "vendor_id": "apple",
    "name": "Apple Fifth Avenue",
    "catalog_size": 150,
    "open_status": "open"
  }
}
```

---

#### 2. Partnership Buildings (Company HQs / Hubs)

**What they are:**
- Company headquarters
- Regional hubs
- Distribution centers
- Partner offices

**What they do:**
- Source products (procurement)
- Manage partnerships (supplier relationships)
- Coordinate logistics (multi-party shipments)

**Visual representation:**
- Larger buildings (more prominent)
- Corporate branding (logos/colors)
- Partnership links (filaments to suppliers)

**RenderSpec node:**
```json
{
  "id": "building.acme_hq.sf",
  "kind": "building",
  "transform": { ... },
  "material": "partnership_building",
  "props": {
    "building_type": "partnership",
    "company_id": "acme_corp",
    "name": "ACME HQ",
    "partnerships": ["supplier.A", "supplier.B"]
  }
}
```

---

#### 3. Civic Buildings (Verification Points / Juries / Hotspots)

**What they are:**
- Verification nodes (quality control)
- Jury buildings (dispute resolution)
- Community hotspots (local coordination)

**What they do:**
- Verify commitments (quality checks)
- Resolve disputes (jury decisions)
- Host community events

**Visual representation:**
- Distinct architecture (civic style)
- Authority indicators (verification badges)
- Activity glow (when jury active)

**RenderSpec node:**
```json
{
  "id": "building.jury.001",
  "kind": "building",
  "transform": { ... },
  "material": "civic_building",
  "props": {
    "building_type": "civic",
    "civic_type": "jury",
    "name": "Dispute Resolution Center #1",
    "active_cases": 3
  }
}
```

---

#### 4. Logistics Buildings (Drone Depots / Ports)

**What they are:**
- Drone launch pads
- Shipping ports
- Distribution hubs
- Delivery stations

**What they do:**
- Launch shipments (drone takeoff)
- Accept deliveries (drone landing)
- Route optimization (logistics planning)

**Visual representation:**
- Airport-style buildings
- Takeoff/landing animations
- Active routes (polylines to destinations)

**RenderSpec node:**
```json
{
  "id": "building.drone_depot.lax",
  "kind": "building",
  "transform": { ... },
  "material": "logistics_building",
  "props": {
    "building_type": "logistics",
    "logistics_type": "drone_depot",
    "name": "LAX Drone Hub",
    "active_shipments": 42
  }
}
```

---

#### 5. Community Structures (Channels / Venues)

**What they are:**
- Physical venues (concert halls, stadiums)
- Community centers (gathering spaces)
- Event locations (conferences, meetups)

**What they do:**
- Host events (live performances)
- Facilitate gatherings (coordination)
- Stream content (shared experiences)

**Visual representation:**
- Venue-specific architecture
- Event indicators (when active)
- Capacity/attendance markers

**RenderSpec node:**
```json
{
  "id": "building.venue.madison_square_garden",
  "kind": "building",
  "transform": { ... },
  "material": "community_building",
  "props": {
    "building_type": "community",
    "community_type": "venue",
    "name": "Madison Square Garden",
    "current_event": "Concert: Artist Name",
    "capacity": 20000,
    "attendance": 18500
  }
}
```

---

## PART 3: SHOPPING AS UNIT PRODUCTION (NOT WEB CHECKOUT)

### The StarCraft Production Model

**Traditional e-commerce:**
```
1. Browse catalog (web page)
2. Add to cart (list)
3. Checkout form (payment flow)
4. Order confirmation (email)
5. Wait (no visibility)
6. Delivery (surprise arrival)
```

**Relay shopping (StarCraft model):**
```
1. Click building on globe (select structure)
2. Building panel opens (production UI)
3. Select "units" to produce (products)
4. Purchase creates task (enters build queue)
5. Task shows progression (packing → dispatched → in transit)
6. Delivery visualized (drone unit flying on globe)
7. Arrival notification (HUD alert)
```

---

### Interaction Flow (Step-by-Step)

#### Step 1: Click Building on Globe

**User action:** Click vendor building (e.g., Apple Store)

**System response:**
- Selected building highlights (glow)
- Building panel opens (bottom-left, StarCraft-style)
- Catalog loads (available "units" to produce)

---

#### Step 2: Building Panel Opens (Production UI)

**Panel contents:**
- **Building name** (e.g., "Apple Fifth Avenue")
- **Catalog grid** (products as unit cards)
- **Unit details** (name, price, production time, delivery time)
- **Build button** (like "Train Unit" in StarCraft)

**Visual style:**
- StarCraft barracks/factory UI
- Grid of unit portraits (product images)
- Stats below each unit (price = minerals, delivery = build time)

---

#### Step 3: Select Units to Produce (Add to Queue)

**User action:** Click "Build" on iPhone 15 Pro

**System response:**
- Escrow check (do I have enough resources?)
- Commitment check (do I have capacity?)
- Authority check (am I authorized to purchase?)

**If approved:**
- Create `TASK_CREATE` commit (task enters my build queue)
- Lock escrow (resources reserved)
- Start "production" (vendor packing)

**If rejected:**
- Show error (insufficient resources / capacity)
- Suggest actions (add funds, request delegation)

---

#### Step 4: Task Enters Build Queue (HUD Taskbar)

**Build queue display:**
```
┌─────────────────────────────────────────────────┐
│ BUILD QUEUE (3 / 8 capacity)                    │
├─────────────────────────────────────────────────┤
│ 1. iPhone 15 Pro  ████████░░  80% (packing)    │
│    └─ ETA: 2h 15m                              │
│                                                 │
│ 2. MacBook Air    ███░░░░░░░  30% (dispatched) │
│    └─ ETA: 4h 30m                              │
│                                                 │
│ 3. AirPods Pro    █░░░░░░░░░  10% (queued)     │
│    └─ ETA: 6h 45m                              │
└─────────────────────────────────────────────────┘
```

**Task states:**
- `queued` - Waiting for vendor to start
- `packing` - Vendor preparing shipment
- `dispatched` - Shipment created, awaiting pickup
- `in_transit` - Drone flying on globe
- `arriving` - Near destination
- `delivered` - Completed

---

#### Step 5: Shipment Visualization (Drone Unit)

**When task enters `in_transit` state:**
- Spawn drone unit on globe (at vendor building)
- Polyline route (vendor → my location)
- Drone flies along route (animated movement)
- ETA updates in real-time (HUD taskbar)

**RenderSpec node:**
```json
{
  "id": "shipment.order_12345.drone",
  "kind": "shipment",
  "geometry": {
    "route": {
      "points": [[vendor_lat, vendor_lng], [my_lat, my_lng]],
      "current_position_index": 0.3,
      "speed": 0.05
    }
  },
  "material": "drone_in_transit",
  "props": {
    "shipment_id": "order_12345",
    "state": "in_transit",
    "product": "iPhone 15 Pro",
    "eta_seconds": 7200
  }
}
```

**Layer 3 rendering:**
- Drone model flies along polyline
- Trail effect (motion blur / contrail)
- ETA label (hover tooltip)

---

#### Step 6: Arrival (Task Completes)

**When drone reaches destination:**
- `TASK_COMPLETE` commit
- Task removed from build queue
- Notification in HUD (arrival alert)
- Product added to inventory (my units)

**Animation:**
- Drone landing animation
- Glow effect on my position
- HUD notification ("iPhone 15 Pro delivered!")

---

## PART 4: DUAL-USE (PERSONAL + COMPANY)

### Same UI, Different Scale

**Personal consumer shopping:**
- Buy iPhone (1 unit)
- Track delivery (1 shipment)
- Personal resources (my escrow)

**Company procurement:**
- Order 10,000 chips (bulk)
- Multiple shipments (fleet of drones)
- Company resources (corporate escrow)

**Key insight:** The UI is the same. The scale differs.

---

### Physical Logistics Planning

**For companies:**
- Click warehouse building → See inventory
- Plan shipments → Optimize routes
- Track fleet → Multiple drones in transit

**For consumers:**
- Click store → Browse catalog
- Buy product → Single drone
- Track delivery → Watch on globe

**The globe makes logistics tangible.**

---

### Contracting (Commitments + Escrow + Delivery Proofs)

**B2B contracts:**
- Partnership building → Open contract panel
- Define commitment (delivery date, quality specs)
- Lock escrow (payment on delivery proof)
- Track shipment (drone with verification)
- Proof of delivery (commit + evidence)

**Consumer purchases:**
- Same flow, simplified
- Escrow auto-releases on delivery
- Commitment is implicit (vendor promises delivery)

---

## PART 5: DATA/RENDERING CONTRACT (LAYER 2 → 3)

### New RenderSpec Node Kinds

#### 1. Building Nodes

```json
{
  "kind": "building",
  "props": {
    "building_type": "vendor" | "partnership" | "civic" | "logistics" | "community",
    "name": "Building Name",
    ...
  }
}
```

**Layer 3 mapping:**
- `building_type: vendor` → Store icon + catalog UI
- `building_type: partnership` → Corporate HQ + partnership panel
- `building_type: civic` → Civic architecture + jury UI
- `building_type: logistics` → Airport + shipment routes
- `building_type: community` → Venue + event details

---

#### 2. Task Nodes (Build Queue Entries)

```json
{
  "kind": "task",
  "props": {
    "task_id": "task_12345",
    "task_type": "purchase" | "shipment" | "commitment",
    "state": "queued" | "packing" | "dispatched" | "in_transit" | "delivered",
    "progress": 0.8,
    "eta_seconds": 7200,
    "product": "iPhone 15 Pro",
    "vendor": "apple",
    ...
  }
}
```

**Layer 3 mapping:**
- `state: packing` → Progress bar (yellow)
- `state: in_transit` → Drone on globe
- `state: delivered` → Green checkmark

---

#### 3. Shipment Nodes (In-Transit Drones)

```json
{
  "kind": "shipment",
  "geometry": {
    "route": {
      "points": [[lat1, lng1], [lat2, lng2]],
      "current_position_index": 0.3,
      "speed": 0.05
    }
  },
  "material": "drone_in_transit",
  "props": {
    "shipment_id": "order_12345",
    "state": "in_transit",
    "product": "iPhone 15 Pro",
    "eta_seconds": 7200
  }
}
```

**Layer 3 mapping:**
- Drone model flies along polyline
- Trail/contrail effect
- ETA label on hover

---

#### 4. Unit/Product Definitions (Catalog Items)

```json
{
  "kind": "catalog_item",
  "props": {
    "product_id": "iphone_15_pro",
    "name": "iPhone 15 Pro",
    "vendor": "apple",
    "price": 999.00,
    "production_time_seconds": 3600,
    "delivery_time_seconds": 7200,
    "in_stock": true
  }
}
```

**Layer 3 mapping:**
- Product card in building panel
- Price → "Minerals: 999"
- Production time → "Build time: 1h"
- Delivery time → "Ship time: 2h"

---

### Animation Intents (Triggered by Events)

```json
{
  "animation_intents": [
    {
      "target_id": "shipment.order_12345.drone",
      "trigger_event_id": 42,
      "animation_type": "launch",
      "duration_ms": 2000,
      "params": {
        "from": "building.apple_store.nyc_001",
        "trajectory": "arc"
      }
    },
    {
      "target_id": "shipment.order_12345.drone",
      "trigger_event_id": 43,
      "animation_type": "land",
      "duration_ms": 2000,
      "params": {
        "at": "unit.my_location"
      }
    }
  ]
}
```

**Layer 3 mapping:**
- `launch` → Drone takeoff animation
- `land` → Drone landing animation
- Event-driven (deterministic, replayable)

---

## PART 6: PERSONAL "BANK" GAUGES (STARCRAFT RESOURCES)

### HUD Resource Display (Top-Right)

```
┌─────────────────────────────────────────────────┐
│ 🔵 Delegated Influence  ██████░░░░  60%        │
│ 🟨 Commitments          ███░░░░░░  3 / 8       │
│ 🟩 Escrow               ⛓ $1,240.00 locked     │
│ 🟪 Time Windows         ⏳ 2 active cooldowns   │
│ 🟥 Dispute Pressure     ⚠ 0 active disputes    │
└─────────────────────────────────────────────────┘
```

**Key principle:** These are **my** resources, not world resources.

**What governs:**
- Can I purchase? (Escrow + commitments)
- Can I propose? (Delegated influence)
- Can I act? (Time windows / cooldowns)
- Am I blocked? (Dispute pressure)

**Personal meters → Personal actions**

---

## PART 7: UI COMPONENT CHECKLIST (REACT)

### 1. MinimapPanel

**Purpose:** Show globe overview with my location + nearby buildings

**Location:** Bottom-right corner (StarCraft minimap position)

**Contents:**
- Mini globe (low-detail)
- My position (highlighted)
- Nearby buildings (icons)
- Active shipments (dots moving)

**Interactions:**
- Click to pan main globe view
- Drag to navigate

**Component:**
```typescript
<MinimapPanel
  myPosition={[lat, lng]}
  buildings={nearbyBuildings}
  shipments={activeShipments}
  onNavigate={(lat, lng) => globe.panTo(lat, lng)}
/>
```

---

### 2. SelectedBuildingPanel

**Purpose:** Show details + catalog for selected building (StarCraft unit panel)

**Location:** Bottom-left corner

**Contents:**
- Building name + type
- Catalog grid (products as cards)
- Unit stats (price, production time, delivery time)
- Build buttons

**Interactions:**
- Click product → Add to build queue
- Hover → Show details
- Close → Deselect building

**Component:**
```typescript
<SelectedBuildingPanel
  building={selectedBuilding}
  catalog={building.catalog}
  onPurchase={(product) => createTask(product)}
  onClose={() => deselectBuilding()}
/>
```

---

### 3. BuildQueueBar

**Purpose:** Show active tasks (StarCraft production queue)

**Location:** Bottom-center (above minimap)

**Contents:**
- Task list (queued, in progress, completed)
- Progress bars (packing, in transit, etc.)
- ETA timers
- Cancel buttons (if allowed)

**Interactions:**
- Click task → Focus on shipment (if in transit)
- Cancel task → Refund escrow (if early)
- Hover → Show details

**Component:**
```typescript
<BuildQueueBar
  tasks={myTasks}
  onFocusTask={(task) => globe.focusShipment(task.shipment_id)}
  onCancelTask={(task) => cancelTask(task)}
/>
```

---

### 4. ResourceGaugesRow

**Purpose:** Show personal resources (StarCraft minerals/gas/supply)

**Location:** Top-right corner

**Contents:**
- 5 gauges (legitimacy, commitments, escrow, time, disputes)
- Hover tooltips (detailed breakdown)
- Decay animations (live updates)

**Interactions:**
- Hover → Show details
- Click → Open resource detail panel

**Component:**
```typescript
<ResourceGaugesRow
  legitimacy={myGauges.legitimacy}
  commitments={myGauges.commitments}
  escrow={myGauges.escrow}
  timeWindows={myGauges.timeWindows}
  disputes={myGauges.disputes}
/>
```

---

### 5. ShipmentTracker

**Purpose:** Track individual shipment (StarCraft unit selection)

**Location:** Modal / side panel (when shipment selected)

**Contents:**
- Shipment details (product, vendor, destination)
- Route visualization (polyline on globe)
- Current position (lat/lng)
- ETA countdown
- Status updates (packing → dispatched → in transit → arriving)

**Interactions:**
- Close → Deselect shipment
- Focus → Pan globe to drone

**Component:**
```typescript
<ShipmentTracker
  shipment={selectedShipment}
  onClose={() => deselectShipment()}
  onFocus={() => globe.focusDrone(shipment.id)}
/>
```

---

### 6. UnitRoster

**Purpose:** List my controlled agents/SCVs (StarCraft unit control)

**Location:** Side panel (left)

**Contents:**
- Unit list (agents I control)
- Unit states (idle, working, moving, blocked)
- Delegation status (influence granted)
- Select multiple units (control groups)

**Interactions:**
- Click unit → Focus on globe
- Select multiple → Group control
- Right-click → Command menu

**Component:**
```typescript
<UnitRoster
  units={myUnits}
  onSelectUnit={(unit) => globe.focusUnit(unit)}
  onGroupSelect={(units) => createControlGroup(units)}
/>
```

---

### 7. TaskCommandCard

**Purpose:** Show selected task details + commands (StarCraft command card)

**Location:** Bottom-center (when task selected)

**Contents:**
- Task name + type
- Progress bar
- Commands (cancel, expedite, modify)
- Status log (timeline of events)

**Interactions:**
- Click command → Execute action
- Close → Deselect task

**Component:**
```typescript
<TaskCommandCard
  task={selectedTask}
  onCancel={() => cancelTask(task)}
  onExpedite={() => expediteTask(task)}
  onClose={() => deselectTask()}
/>
```

---

## PART 8: LOCKED INVARIANTS (PERSONAL HUD + PHYSICAL GLOBE)

1. ✅ **HUD is personal** (shows my state, not world state)
2. ✅ **HUD is viewer-scoped** (filtered by my identity)
3. ✅ **Globe is shared** (physical world map)
4. ✅ **Buildings are entities** (vendors, partnerships, civic, logistics, community)
5. ✅ **Shopping is production** (StarCraft unit creation, not web checkout)
6. ✅ **Build queue is taskbar** (active tasks with progress)
7. ✅ **Shipments are units** (drones flying on globe)
8. ✅ **Resources are gauges** (legitimacy, commitments, escrow, time, disputes)
9. ✅ **Layer 2 outputs nodes** (buildings, tasks, shipments, catalog items)
10. ✅ **Layer 3 renders UI** (StarCraft-style panels + 3D visuals)
11. ✅ **Dual-use interface** (personal consumer + company procurement)
12. ✅ **Physical logistics** (globe makes coordination tangible)

---

## PART 9: ARCHITECTURAL IMPLICATIONS

### What Changes from Previous Commits

#### From `architecture@c7` (Coordination Gauges)

**Before:** Gauges were described generically (could be personal or global)  
**After:** Gauges are explicitly **personal** (my resources, my state)

**What this means:**
- `GET /api/relay-physics/gauges` must accept `viewer_id` parameter
- Gauge values are derived from **my** filaments, **my** commitments, **my** delegations
- No "global legitimacy" gauge (only "my legitimacy in context X")

---

#### From `architecture@c5` (RenderSpec v1)

**Before:** RenderSpec had units, filaments, timeboxes  
**After:** RenderSpec also has **buildings**, **tasks**, **shipments**, **catalog items**

**What this means:**
- Add `kind: building` to RenderSpec node kinds
- Add `kind: task` for build queue entries
- Add `kind: shipment` for in-transit drones
- Add `kind: catalog_item` for product definitions

---

### New Backend Requirements

#### 1. Building Store (PR #6)

**File:** `apps/server/src/relay_physics/building_store.rs`

**Purpose:** Track physical buildings on globe

**Operations:**
- `BUILDING_CREATE` - Add building to globe
- `BUILDING_UPDATE` - Modify building state
- `CATALOG_UPDATE` - Update available products

**Data:**
- Building location (lat/lng/altitude)
- Building type (vendor, partnership, civic, logistics, community)
- Catalog (available products/services)
- Operational state (open, closed, busy)

---

#### 2. Task Store (PR #7)

**File:** `apps/server/src/relay_physics/task_store.rs`

**Purpose:** Track active tasks (build queue)

**Operations:**
- `TASK_CREATE` - Add task to queue
- `TASK_PROGRESS` - Update task state
- `TASK_COMPLETE` - Mark task finished
- `TASK_CANCEL` - Cancel task (refund)

**Data:**
- Task owner (viewer identity)
- Task type (purchase, shipment, commitment)
- Task state (queued, packing, dispatched, in_transit, delivered)
- Progress (0.0 - 1.0)
- ETA (estimated completion)

---

#### 3. Shipment Store (PR #8)

**File:** `apps/server/src/relay_physics/shipment_store.rs`

**Purpose:** Track in-transit shipments (drones)

**Operations:**
- `SHIPMENT_CREATE` - Spawn drone
- `SHIPMENT_UPDATE_POSITION` - Move drone along route
- `SHIPMENT_ARRIVE` - Complete delivery
- `SHIPMENT_CANCEL` - Abort shipment (return)

**Data:**
- Route (polyline points)
- Current position (interpolated from time)
- Speed (movement rate)
- ETA (arrival time)
- Cargo (product details)

---

### New Frontend Requirements

#### React Components (7 components)

**Files:**
```
apps/client-web/src/components/
├── MinimapPanel.tsx           (Bottom-right minimap)
├── SelectedBuildingPanel.tsx  (Bottom-left building UI)
├── BuildQueueBar.tsx          (Bottom-center taskbar)
├── ResourceGaugesRow.tsx      (Top-right gauges)
├── ShipmentTracker.tsx        (Modal for selected shipment)
├── UnitRoster.tsx             (Left panel for my units)
└── TaskCommandCard.tsx        (Bottom-center commands)
```

---

#### Three.js Renderers

**Files:**
```
apps/client-web/src/rendering/
├── BuildingRenderer.ts        (Render building nodes)
├── ShipmentRenderer.ts        (Render drone units)
├── TaskRenderer.ts            (Progress indicators on HUD)
└── RouteRenderer.ts           (Polyline routes for shipments)
```

---

## PART 10: CAUSAL REFS

- **Inputs:** `architecture@c5` (RenderSpec), `architecture@c7` (Coordination gauges)
- **Authority:** system.architect
- **Evidence:**
  - StarCraft UI design (personal HUD + shared map)
  - RTS game mechanics (production, build queue, units)
  - E-commerce logistics (shipment tracking, delivery visualization)

---

## PART 11: NEXT STEPS (IMPLEMENTATION ROADMAP)

### Phase 1: Backend - Buildings (PR #6)
**Time:** 4-6 hours  
**Deliverables:**
- `building_store.rs`
- `BUILDING_CREATE`, `BUILDING_UPDATE`, `CATALOG_UPDATE` operations
- Building node in RenderSpec

---

### Phase 2: Backend - Tasks (PR #7)
**Time:** 3-4 hours  
**Deliverables:**
- `task_store.rs`
- `TASK_CREATE`, `TASK_PROGRESS`, `TASK_COMPLETE` operations
- Task node in RenderSpec

---

### Phase 3: Backend - Shipments (PR #8)
**Time:** 4-5 hours  
**Deliverables:**
- `shipment_store.rs`
- `SHIPMENT_CREATE`, `SHIPMENT_UPDATE_POSITION`, `SHIPMENT_ARRIVE` operations
- Shipment node in RenderSpec
- Route interpolation (position calculation)

---

### Phase 4: Frontend - HUD Components
**Time:** 8-10 hours  
**Deliverables:**
- 7 React components (Minimap, BuildingPanel, BuildQueue, Gauges, ShipmentTracker, UnitRoster, CommandCard)
- SSE event listeners (live updates)
- StarCraft-style layout + interactions

---

### Phase 5: Frontend - 3D Rendering
**Time:** 6-8 hours  
**Deliverables:**
- Building renderer (icons on globe)
- Shipment renderer (drones flying)
- Route renderer (polylines)
- Animation intents (launch, land, pulse)

---

## PART 12: LOCKED DESIGN PRINCIPLES

### Principle 1: HUD = Me, Globe = World

**Bad (global dashboard):**
```
HUD shows: Total world resources, all users' tasks, aggregate stats
```

**Good (personal HUD):**
```
HUD shows: My resources, my tasks, my units, my notifications
```

---

### Principle 2: Shopping = Production (Not Checkout)

**Bad (web checkout):**
```
Add to cart → Checkout form → Wait → Surprise delivery
```

**Good (StarCraft production):**
```
Select building → Choose unit → Build starts → Progress visible → Drone arrives
```

---

### Principle 3: Buildings = Physical (Not Abstract)

**Bad (abstract entities):**
```
"Vendor" is a database entry with no location
```

**Good (physical buildings):**
```
"Apple Store Fifth Avenue" is a building on the globe at (40.7637, -73.9722)
```

---

### Principle 4: Tasks = Visible (Not Hidden)

**Bad (order tracking number):**
```
"Your order #12345 is being processed" (no visibility)
```

**Good (build queue):**
```
iPhone 15 Pro ████████░░ 80% (packing) - ETA: 2h 15m
```

---

## LOCKED INVARIANT SUMMARY

**HUD is personal. Globe is shared.**

**Shopping is production. Shipments are units.**

**Buildings are physical. Tasks are visible.**

**Layer 2 outputs data. Layer 3 renders StarCraft.**

---

**Status:** LOCKED  
**Supersedes:** None (clarifies `c5`, `c7`)  
**Next:** Implement buildings, tasks, shipments (PR #6-8) + HUD components

---

**Philosophy:**  
The globe is not a dashboard. The HUD is not a map.  
**The HUD is me. The globe is the world. StarCraft got it right 25 years ago.**

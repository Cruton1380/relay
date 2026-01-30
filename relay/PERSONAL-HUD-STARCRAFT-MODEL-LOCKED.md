# Personal HUD + Physical Globe - LOCKED ✅

**Date:** 2026-01-28  
**Status:** Architecture locked as `architecture@c9` (NON-NEGOTIABLE)  
**Reference:** `relay/filaments/architecture/0009_personal_hud_physical_globe.md`

---

## THE CORE DISTINCTION (NON-NEGOTIABLE)

### HUD = Personal (My State in the World)

**The HUD shows:**
- My resources (legitimacy, commitments, escrow, time windows, disputes)
- My active tasks (build queue / production queue)
- My units (agents I control / have delegated authority to)
- My supply/capacity (commitment capacity, active obligations)
- My notifications (incoming requests, disputes, deliveries)

**The HUD does NOT show:**
- ❌ Global analytics
- ❌ Other players' resources
- ❌ World-level dashboards

**Analogy:** StarCraft HUD (my minerals, my gas, my supply, my units)

---

### Globe = Shared (Physical World Map)

**The globe shows:**
- Buildings (vendors, partnerships, civic structures, logistics hubs)
- Points of interest (stores, warehouses, factories, verification points)
- Other players' units (visible agents/SCVs)
- Shipments in transit (drone delivery units flying)
- Filaments (commitment threads between entities)

**The globe does NOT show:**
- ❌ My personal resources (that's in the HUD)
- ❌ My build queue (that's in the HUD)
- ❌ My notifications (that's in the HUD)

**Analogy:** StarCraft map (shared world with buildings, units, fog of war)

---

## KEY ARCHITECTURAL DECISIONS

### 1. Shopping = Unit Production (Not Web Checkout)

**Traditional e-commerce:**
```
Browse → Add to cart → Checkout form → Wait (no visibility) → Surprise delivery
```

**Relay (StarCraft model):**
```
Click building → Open production panel → Select units → Build starts → Progress visible → Drone arrives
```

**Key transformation:** Shopping becomes **production**, not **checkout**.

---

### 2. Buildings = Physical Entities (Not Abstract)

**Building types:**
1. **Vendor buildings** (stores, warehouses, factories)
2. **Partnership buildings** (company HQs, regional hubs)
3. **Civic buildings** (verification points, juries, hotspots)
4. **Logistics buildings** (drone depots, ports)
5. **Community structures** (venues, channels, event locations)

**Key property:** Every building has a **physical location** on the globe.

---

### 3. Build Queue = Task Bar (StarCraft Production)

**Visual:**
```
BUILD QUEUE (3 / 8 capacity)
1. iPhone 15 Pro  ████████░░  80% (packing)    - ETA: 2h 15m
2. MacBook Air    ███░░░░░░░  30% (dispatched) - ETA: 4h 30m
3. AirPods Pro    █░░░░░░░░░  10% (queued)     - ETA: 6h 45m
```

**Task states:**
- `queued` - Waiting for vendor to start
- `packing` - Vendor preparing shipment
- `dispatched` - Shipment created, awaiting pickup
- `in_transit` - Drone flying on globe
- `arriving` - Near destination
- `delivered` - Completed

---

### 4. Shipments = Drones (Animated Units)

**What you see:**
- Drone spawns at vendor building
- Drone flies along polyline route (vendor → my location)
- Drone arrives at my position
- Landing animation + HUD notification

**Key insight:** Logistics becomes **tangible** (you can see shipments moving).

---

### 5. Dual-Use Interface (Personal + Company)

**Same UI, different scale:**

| Use Case | Personal Consumer | Company Procurement |
|----------|------------------|---------------------|
| **Purchase** | Buy iPhone (1 unit) | Order 10,000 chips (bulk) |
| **Shipments** | 1 drone | Fleet of drones |
| **Resources** | My escrow | Corporate escrow |
| **Build queue** | 3 tasks | 50 tasks |

**Key insight:** One UI language for consumer **and** enterprise.

---

## THE 7 UI COMPONENTS (REACT CHECKLIST)

### 1. MinimapPanel (Bottom-Right)
**Purpose:** Globe overview with my location + nearby buildings  
**Contents:** Mini globe, my position, nearby buildings, active shipments  
**Interactions:** Click to pan, drag to navigate

---

### 2. SelectedBuildingPanel (Bottom-Left)
**Purpose:** Building details + catalog (StarCraft production UI)  
**Contents:** Building name, catalog grid, unit stats, build buttons  
**Interactions:** Click product → Add to build queue

---

### 3. BuildQueueBar (Bottom-Center)
**Purpose:** Active tasks (StarCraft production queue)  
**Contents:** Task list, progress bars, ETA timers, cancel buttons  
**Interactions:** Click task → Focus on shipment, cancel task → Refund

---

### 4. ResourceGaugesRow (Top-Right)
**Purpose:** Personal resources (StarCraft minerals/gas/supply)  
**Contents:** 5 gauges (legitimacy, commitments, escrow, time, disputes)  
**Interactions:** Hover → Details, click → Full panel

---

### 5. ShipmentTracker (Modal)
**Purpose:** Track individual shipment  
**Contents:** Shipment details, route visualization, current position, ETA  
**Interactions:** Close → Deselect, focus → Pan globe to drone

---

### 6. UnitRoster (Left Panel)
**Purpose:** List my controlled agents/SCVs  
**Contents:** Unit list, unit states, delegation status  
**Interactions:** Click unit → Focus on globe, select multiple → Group control

---

### 7. TaskCommandCard (Bottom-Center)
**Purpose:** Selected task details + commands  
**Contents:** Task name, progress bar, commands (cancel, expedite), status log  
**Interactions:** Click command → Execute action

---

## INTERACTION FLOW (SHOPPING EXAMPLE)

### Step 1: Click Building on Globe

**User:** Clicks Apple Store building on globe  
**System:** Building highlights, panel opens (bottom-left), catalog loads

---

### Step 2: Building Panel Opens

**Contents:**
- Building name ("Apple Fifth Avenue")
- Catalog grid (products as unit cards)
- Unit stats (price, production time, delivery time)
- Build buttons

---

### Step 3: Select Unit to Produce

**User:** Clicks "Build" on iPhone 15 Pro  
**System:**
- Checks escrow (do I have $999?)
- Checks commitment capacity (do I have open slots?)
- Checks authority (am I authorized?)

**If approved:**
- Creates `TASK_CREATE` commit
- Locks escrow ($999)
- Adds task to build queue
- Starts "production" (vendor packing)

---

### Step 4: Task Enters Build Queue

**Build queue display:**
```
1. iPhone 15 Pro  ████████░░  80% (packing)    - ETA: 2h 15m
```

**Task progresses:**
- `queued` → `packing` → `dispatched` → `in_transit` → `delivered`

---

### Step 5: Shipment Spawns (Drone Unit)

**When task enters `in_transit`:**
- Drone spawns at vendor building
- Polyline route drawn (vendor → my location)
- Drone flies along route (animated)
- ETA updates in real-time

**Visual:**
- Drone model on globe
- Trail effect (motion blur)
- ETA label (hover tooltip)

---

### Step 6: Arrival (Task Completes)

**When drone reaches my location:**
- Landing animation
- Task removed from build queue
- HUD notification ("iPhone 15 Pro delivered!")
- Product added to inventory

---

## RENDERSPEC CHANGES (LAYER 2 → 3)

### New Node Kinds

#### 1. Building Nodes
```json
{
  "kind": "building",
  "props": {
    "building_type": "vendor",
    "name": "Apple Fifth Avenue",
    "catalog_size": 150,
    "open_status": "open"
  }
}
```

**Layer 3 mapping:**
- `building_type: vendor` → Store icon + catalog UI
- `building_type: partnership` → Corporate HQ + partnership panel
- `building_type: civic` → Civic architecture + jury UI

---

#### 2. Task Nodes
```json
{
  "kind": "task",
  "props": {
    "task_id": "task_12345",
    "state": "packing",
    "progress": 0.8,
    "eta_seconds": 7200,
    "product": "iPhone 15 Pro"
  }
}
```

**Layer 3 mapping:**
- `state: packing` → Progress bar (yellow)
- `state: in_transit` → Drone on globe
- `state: delivered` → Green checkmark

---

#### 3. Shipment Nodes
```json
{
  "kind": "shipment",
  "geometry": {
    "route": {
      "points": [[lat1, lng1], [lat2, lng2]],
      "current_position_index": 0.3
    }
  },
  "material": "drone_in_transit"
}
```

**Layer 3 mapping:**
- Drone model flies along polyline
- Trail/contrail effect
- ETA label on hover

---

#### 4. Catalog Item Nodes
```json
{
  "kind": "catalog_item",
  "props": {
    "product_id": "iphone_15_pro",
    "name": "iPhone 15 Pro",
    "price": 999.00,
    "production_time_seconds": 3600
  }
}
```

**Layer 3 mapping:**
- Product card in building panel
- Price → "Minerals: 999"
- Production time → "Build time: 1h"

---

## BACKEND CHANGES (NEW STORES)

### Building Store (PR #6)
**File:** `apps/server/src/relay_physics/building_store.rs`  
**Purpose:** Track physical buildings on globe  
**Operations:** `BUILDING_CREATE`, `BUILDING_UPDATE`, `CATALOG_UPDATE`

---

### Task Store (PR #7)
**File:** `apps/server/src/relay_physics/task_store.rs`  
**Purpose:** Track active tasks (build queue)  
**Operations:** `TASK_CREATE`, `TASK_PROGRESS`, `TASK_COMPLETE`, `TASK_CANCEL`

---

### Shipment Store (PR #8)
**File:** `apps/server/src/relay_physics/shipment_store.rs`  
**Purpose:** Track in-transit shipments (drones)  
**Operations:** `SHIPMENT_CREATE`, `SHIPMENT_UPDATE_POSITION`, `SHIPMENT_ARRIVE`

---

## LOCKED INVARIANTS (12 TOTAL)

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

## IMPLEMENTATION ROADMAP

### Phase 1: Backend - Buildings (PR #6)
**Time:** 4-6 hours  
**Deliverables:** Building store + RenderSpec building nodes

---

### Phase 2: Backend - Tasks (PR #7)
**Time:** 3-4 hours  
**Deliverables:** Task store + RenderSpec task nodes

---

### Phase 3: Backend - Shipments (PR #8)
**Time:** 4-5 hours  
**Deliverables:** Shipment store + RenderSpec shipment nodes + route interpolation

---

### Phase 4: Frontend - HUD Components
**Time:** 8-10 hours  
**Deliverables:** 7 React components (Minimap, BuildingPanel, BuildQueue, Gauges, ShipmentTracker, UnitRoster, CommandCard)

---

### Phase 5: Frontend - 3D Rendering
**Time:** 6-8 hours  
**Deliverables:** Building renderer, shipment renderer, route renderer, animation intents

---

## DESIGN PRINCIPLES (LOCKED)

### Principle 1: HUD = Me, Globe = World
```
HUD shows: My resources, my tasks, my units
Globe shows: Buildings, other players, shipments
```

---

### Principle 2: Shopping = Production (Not Checkout)
```
Select building → Choose unit → Build starts → Progress visible → Drone arrives
```

---

### Principle 3: Buildings = Physical (Not Abstract)
```
"Apple Store Fifth Avenue" is at (40.7637, -73.9722) on globe
```

---

### Principle 4: Tasks = Visible (Not Hidden)
```
iPhone 15 Pro ████████░░ 80% (packing) - ETA: 2h 15m
```

---

## PHILOSOPHY

**The globe is not a dashboard. The HUD is not a map.**

**The HUD is me. The globe is the world.**

**Shopping is not checkout. Shopping is production.**

**Shipments are not tracking numbers. Shipments are drones.**

**StarCraft got it right 25 years ago. Relay maps coordination to the same visual language.**

---

## QUICK REFERENCE

### When to use HUD
- Showing **my** resources
- Showing **my** tasks
- Showing **my** units
- Showing **my** notifications

### When to use Globe
- Showing **buildings** (physical entities)
- Showing **other players** (visible units)
- Showing **shipments** (drones in transit)
- Showing **world state** (shared map)

### When to use Building Panel
- Clicking building on globe
- Browsing catalog (products/services)
- Creating tasks (production)

### When to use Build Queue
- Showing **my active tasks**
- Tracking progress (packing → in transit → delivered)
- Managing queue (cancel, expedite)

---

**Status:** ✅ **LOCKED (NON-NEGOTIABLE)**  
**Reference:** `architecture@c9`  
**Next:** Implement buildings, tasks, shipments (PR #6-8) + HUD components

---

**Bottom Line:**  
The HUD is personal. The globe is shared.  
Shopping is production. Shipments are units.  
**StarCraft solved this 25 years ago. Relay uses the same pattern.**

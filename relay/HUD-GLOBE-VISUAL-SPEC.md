# HUD + Globe Visual Specification

**Reference:** `architecture@c9` - Personal HUD + Physical Globe (StarCraft Model)  
**Date:** 2026-01-28  
**Status:** LOCKED ✅

---

## FULL SCREEN LAYOUT (STARCRAFT STYLE)

```
┌──────────────────────────────────────────────────────────────────────────┐
│ 🔵 Delegated Influence  ██████░░░░  60%  (TOP-RIGHT)                    │
│ 🟨 Commitments          ███░░░░░░  3 / 8                                │
│ 🟩 Escrow               ⛓ $1,240.00 locked                              │
│ 🟪 Time Windows         ⏳ 2 active cooldowns                            │
│ 🟥 Dispute Pressure     ⚠ 0 active disputes                             │
│                                                                          │
│                                                                          │
│                       ┌──────────────────┐                              │
│                       │    3D GLOBE      │                              │
│                       │   (SHARED MAP)   │   ← UNIT ROSTER (LEFT)       │
│                       │                  │      ┌──────────────┐        │
│                       │  • Buildings     │      │ My Units:    │        │
│                       │  • Other players │      │              │        │
│                       │  • Shipments     │      │ 1. Alice     │        │
│                       │  • Filaments     │      │ 2. Bob       │        │
│                       │                  │      │ 3. Charlie   │        │
│                       └──────────────────┘      └──────────────┘        │
│                                                                          │
│  ┌────────────────────┐                           ┌──────────────────┐  │
│  │ SELECTED BUILDING  │    BUILD QUEUE (CENTER)   │    MINIMAP       │  │
│  │                    │   ┌───────────────────┐   │                  │  │
│  │ Apple Fifth Ave    │   │ 1. iPhone 15 Pro  │   │  [  Mini Globe  ]│  │
│  │ ┌──┬──┬──┬──┬──┐   │   │    ████████░░ 80% │   │   • Me (blue)   │  │
│  │ │  │  │  │  │  │   │   │    ETA: 2h 15m    │   │   • Buildings   │  │
│  │ │  │  │  │  │  │   │   ├───────────────────┤   │   • Shipments   │  │
│  │ └──┴──┴──┴──┴──┘   │   │ 2. MacBook Air    │   └──────────────────┘  │
│  │  (Catalog Grid)    │   │    ███░░░░░░░ 30% │                         │
│  │                    │   │    ETA: 4h 30m    │                         │
│  │ [ Build Selected ] │   └───────────────────┘                         │
│  └────────────────────┘                                                 │
│ (BOTTOM-LEFT)            (BOTTOM-CENTER)          (BOTTOM-RIGHT)        │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## COMPONENT POSITIONS (LOCKED)

### Top-Right: Resource Gauges Row
**Size:** 400px wide, 150px tall  
**Position:** Fixed top-right corner  
**Always visible:** Yes

---

### Center: 3D Globe (Main View)
**Size:** Full screen (minus HUD panels)  
**Position:** Center  
**Always visible:** Yes  
**Interactive:** Click buildings, click units, pan/zoom

---

### Bottom-Left: Selected Building Panel
**Size:** 600px wide, 400px tall  
**Position:** Fixed bottom-left corner  
**Visible when:** Building selected  
**Contents:** Building name, catalog grid, build buttons

---

### Bottom-Center: Build Queue Bar
**Size:** 800px wide, 200px tall  
**Position:** Fixed bottom-center  
**Always visible:** Yes (if tasks active)  
**Contents:** Task list, progress bars, ETA timers

---

### Bottom-Right: Minimap Panel
**Size:** 300px wide, 300px tall  
**Position:** Fixed bottom-right corner  
**Always visible:** Yes  
**Contents:** Mini globe, my position, nearby buildings

---

### Left: Unit Roster (Expandable)
**Size:** 250px wide, variable height  
**Position:** Fixed left side (mid-height)  
**Visible when:** Units exist  
**Contents:** My controlled units/agents

---

## INTERACTION FLOWS (VISUAL)

### Flow 1: Shopping (Click Building → Purchase → Track Shipment)

```
Step 1: CLICK BUILDING ON GLOBE
┌─────────────────────────────┐
│        3D GLOBE             │
│                             │
│    🏪 ← Click here         │
│   (Apple Store)             │
│                             │
└─────────────────────────────┘
          ↓
Step 2: BUILDING PANEL OPENS
┌─────────────────────────────┐
│ SELECTED BUILDING           │
│ Apple Fifth Avenue          │
│ ┌──┬──┬──┬──┬──┐           │
│ │📱│💻│🎧│⌚│📷│           │
│ └──┴──┴──┴──┴──┘           │
│  iPhone  MacBook  AirPods   │
│  $999    $1,299   $249      │
│                             │
│ [ Build iPhone ]            │
└─────────────────────────────┘
          ↓
Step 3: TASK ENTERS BUILD QUEUE
┌─────────────────────────────┐
│ BUILD QUEUE (1 / 8)         │
│ 1. iPhone 15 Pro            │
│    ████████░░  80% (packing)│
│    ETA: 2h 15m              │
└─────────────────────────────┘
          ↓
Step 4: SHIPMENT SPAWNS (DRONE)
┌─────────────────────────────┐
│        3D GLOBE             │
│                             │
│    🏪 ............. 🏠      │
│   (Store)  🚁  (My Home)    │
│            Drone flying →   │
└─────────────────────────────┘
          ↓
Step 5: ARRIVAL NOTIFICATION
┌─────────────────────────────┐
│ 🎉 iPhone 15 Pro delivered! │
│                             │
│ [ View Inventory ]          │
└─────────────────────────────┘
```

---

### Flow 2: Resource Check (Before Purchase)

```
USER CLICKS "BUILD IPHONE" ($999)

Step 1: CHECK ESCROW GAUGE
┌─────────────────────────────┐
│ 🟩 Escrow  ⛓ $1,240.00     │ ← Do I have $999?
└─────────────────────────────┘
          ↓ YES
Step 2: CHECK COMMITMENT GAUGE
┌─────────────────────────────┐
│ 🟨 Commitments  3 / 8       │ ← Do I have capacity?
└─────────────────────────────┘
          ↓ YES
Step 3: APPROVE PURCHASE
┌─────────────────────────────┐
│ ✅ Purchase approved        │
│ Task created: iPhone 15 Pro │
└─────────────────────────────┘
```

---

### Flow 3: Globe Exploration (Pan + Click)

```
Step 1: PAN GLOBE TO NEW AREA
┌─────────────────────────────┐
│        3D GLOBE             │
│   (drag to pan)             │
│                             │
│    🏪 🏢 🏛️                 │
│   Buildings appear          │
└─────────────────────────────┘
          ↓
Step 2: HOVER BUILDING (TOOLTIP)
┌─────────────────────────────┐
│        3D GLOBE             │
│                             │
│    🏪 ← "Apple Fifth Ave"   │
│         "150 products"      │
│         "Open"              │
└─────────────────────────────┘
          ↓
Step 3: CLICK BUILDING (OPEN PANEL)
┌─────────────────────────────┐
│ SELECTED BUILDING           │
│ Apple Fifth Avenue          │
│ (catalog loads)             │
└─────────────────────────────┘
```

---

## VISUAL STATES (COLOR CODING)

### Task States (Build Queue)

| State | Color | Progress Bar | Icon |
|-------|-------|-------------|------|
| **Queued** | Gray | ░░░░░░░░░░ | ⏸️ |
| **Packing** | Yellow | ████░░░░░░ | 📦 |
| **Dispatched** | Orange | ██████░░░░ | 🚀 |
| **In Transit** | Blue | ████████░░ | 🚁 |
| **Arriving** | Green | █████████░ | 🎯 |
| **Delivered** | Green (✓) | ██████████ | ✅ |

---

### Building States (Globe Markers)

| Building Type | Icon | Color | Glow |
|--------------|------|-------|------|
| **Vendor** | 🏪 | Blue | When selected |
| **Partnership** | 🏢 | Purple | When active |
| **Civic** | 🏛️ | Gold | When jury active |
| **Logistics** | ✈️ | Orange | When shipments active |
| **Community** | 🎭 | Pink | When event active |

---

### Shipment States (Drones)

| State | Visual | Trail | Speed |
|-------|--------|-------|-------|
| **Launching** | 🚁↗️ | White trail | Slow (takeoff) |
| **In Transit** | 🚁→ | Blue trail | Fast |
| **Arriving** | 🚁↘️ | White trail | Slow (landing) |

---

## ANIMATIONS (EVENT-DRIVEN)

### Animation 1: Drone Launch

**Trigger:** `SHIPMENT_CREATE` event  
**Duration:** 2 seconds  
**Sequence:**
1. Drone spawns at building (scale 0 → 1)
2. Drone rises vertically (0m → 50m altitude)
3. Drone rotates toward destination
4. Trail appears (white glow)

---

### Animation 2: Drone Flight

**Trigger:** Position update (every 1 second)  
**Duration:** Continuous  
**Sequence:**
1. Interpolate position along polyline
2. Update trail (fade old segments)
3. Update ETA (real-time countdown)
4. Rotate drone toward movement direction

---

### Animation 3: Drone Landing

**Trigger:** `SHIPMENT_ARRIVE` event  
**Duration:** 2 seconds  
**Sequence:**
1. Drone descends (50m → 0m altitude)
2. Landing glow (expanding circle)
3. Drone disappears (scale 1 → 0)
4. HUD notification pops (bounce animation)

---

### Animation 4: Build Queue Progress

**Trigger:** `TASK_PROGRESS` event  
**Duration:** Smooth (interpolated)  
**Sequence:**
1. Progress bar fills (smooth animation)
2. ETA counts down (real-time)
3. State changes color (queued → packing → in transit)

---

### Animation 5: Resource Gauge Decay

**Trigger:** `VOTE_DECAY` or time passage  
**Duration:** Continuous (slow drain)  
**Sequence:**
1. Legitimacy bar drains (-2% per day)
2. Color shifts (green → yellow → red)
3. Warning icon appears (when < 30%)

---

## RESPONSIVE DESIGN (DESKTOP ONLY MVP)

**Minimum screen size:** 1920x1080  
**Target screen size:** 2560x1440  
**Aspect ratio:** 16:9 or wider

**Why desktop-only MVP:**
- Globe requires precise pan/zoom (mouse)
- Building panels need space (600px+)
- Build queue needs horizontal space (800px+)
- Minimap needs fixed corner (300x300)

**Mobile UX:** Separate simplified interface (future)

---

## ACCESSIBILITY

### Keyboard Controls

| Key | Action |
|-----|--------|
| **WASD** | Pan globe |
| **Q/E** | Rotate globe |
| **Scroll** | Zoom in/out |
| **Space** | Focus on my position |
| **Tab** | Cycle selected buildings |
| **Esc** | Deselect / Close panel |
| **1-8** | Select task in build queue |

---

### Screen Reader Support

**Required ARIA labels:**
- Resource gauges (value + percentage)
- Build queue tasks (name + state + ETA)
- Buildings (name + type + open status)
- Shipments (product + ETA + current position)

---

## LOCKED VISUAL PRINCIPLES

### Principle 1: HUD is Persistent
**All HUD panels are always visible** (resource gauges, build queue, minimap)  
**Only panels that appear/disappear:** Selected building panel, shipment tracker

---

### Principle 2: Globe is Zoomable
**Zoom levels:**
- **Far:** See entire globe (buildings as dots)
- **Medium:** See continents (buildings as icons)
- **Close:** See buildings (3D models)
- **Very close:** See building details (names, status)

---

### Principle 3: Minimap Mirrors Globe
**Minimap always shows:**
- My position (blue dot)
- Nearby buildings (colored dots)
- Active shipments (moving dots)
- Current zoom/pan (white rectangle)

---

### Principle 4: Build Queue is Chronological
**Tasks appear in order:**
1. In progress (top)
2. Queued (below)
3. Recently completed (fade out after 5s)

---

## TECHNICAL SPECS (LAYER 3 RENDERING)

### Globe Rendering (Three.js)

**Library:** `react-three-fiber` + `@react-three/drei`  
**Globe model:** Sphere mesh with Earth texture  
**Buildings:** Instanced meshes (performance)  
**Shipments:** Animated sprites (drones)  
**Routes:** Line geometry (polylines)

**Performance target:** 60 FPS with 1000+ buildings

---

### HUD Rendering (React)

**Library:** `React` + `Tailwind CSS`  
**Animations:** `framer-motion` (smooth transitions)  
**Charts:** Custom SVG (progress bars)  
**Icons:** `lucide-react` (consistent iconography)

**Performance target:** <16ms render time (60 FPS)

---

### SSE Integration (Live Updates)

**Event listeners:**
- `CommitAccepted` → Update gauges
- `TaskProgress` → Update build queue
- `ShipmentUpdate` → Move drone
- `BuildingUpdate` → Refresh catalog

**Reconnect strategy:** Exponential backoff (1s, 2s, 4s, 8s, max 30s)

---

## LOCKED INVARIANTS (VISUAL SPEC)

1. ✅ **HUD panels are fixed** (StarCraft positions)
2. ✅ **Globe is center** (main view)
3. ✅ **Minimap is bottom-right** (always visible)
4. ✅ **Resource gauges are top-right** (always visible)
5. ✅ **Build queue is bottom-center** (visible when tasks active)
6. ✅ **Building panel is bottom-left** (visible when building selected)
7. ✅ **Drones fly on globe** (polyline routes)
8. ✅ **Animations are event-driven** (deterministic, replayable)
9. ✅ **Colors indicate state** (queued, packing, in transit, delivered)
10. ✅ **Desktop-only MVP** (1920x1080 minimum)

---

**Status:** ✅ **VISUAL SPEC LOCKED**  
**Reference:** `architecture@c9`  
**Next:** Implement React components + Three.js renderers

---

**Bottom Line:**  
The HUD is StarCraft. The globe is the map. Buildings are structures. Shipments are units.  
**The visual language is 25 years old. We're just mapping Relay coordination to it.**

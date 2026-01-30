# COMMIT B: Render Endpoints Contract

**Filament:** `architecture`  
**Commit Index:** 1  
**Date:** 2026-01-28  
**Author:** system.architect  
**Type:** API_CONTRACT

---

## INVARIANT LOCKED

**Layer 2 exposes 4 endpoint families: events (SSE), commits (forensic), units (state), render (scene graphs).**

---

## ENDPOINTS DEFINED

### 1. Events Stream (Real-Time Truth)

```
GET /api/relay-physics/events
```

**Purpose:** Real-time event stream with replay support

**Protocol:** Server-Sent Events (SSE)

**Headers:**
- `Last-Event-ID: N` (optional) - Reconnect from event N+1

**Query Params:**
- `?after=N` (optional) - Reconnect from event N+1

**Response Format:**
```
id: 42
event: relay_event
data: {"event_id":42,"event":{"type":"CommitAccepted",...}}

id: 43
event: relay_event
data: {"event_id":43,"event":{"type":"UnitStateChanged",...}}

event: ping
data: heartbeat
```

**Invariants:**
- Event IDs are monotonic (from event log)
- Replay is deterministic (same log → same stream)
- Lag → disconnect → force replay (truth stream completeness)

**Status:** ✅ IMPLEMENTED (PR #1.2)

**See:** `PR-1.2-SSE-REPLAY.md`

---

### 2. Commit Fetch (Forensic Inspection)

```
GET /api/relay-physics/commits/:commitRef
```

**Purpose:** Fetch single commit by ref for forensic inspection

**Path Params:**
- `:commitRef` - Format: `<filamentId>@c<commitIndex>` (e.g., `work.W123@c7`)

**Response Format:**
```json
{
  "filament_id": "work.W123",
  "commit_index": 7,
  "commit_ref": "work.W123@c7",
  "timestamp": "2026-01-28T10:30:00Z",
  "op_type": "OUTPUT_PROPOSED",
  "author_unit_ref": "unit.alice.001",
  "payload": {...},
  "causal_refs": {
    "inputs": ["work.W123@c6"],
    "authority_ref": "delegation.D001@c3",
    "evidence": ["git:abc123"]
  }
}
```

**Use Cases:**
- "Click time cube → open forensic chamber" (Layer 3 UX)
- Debugging commit rejections
- Training data annotation
- Audit trails

**Status:** ⏭️ NEXT (PR #1.3)

---

### 3. Units List/Get (SCV State)

```
GET /api/relay-physics/units
GET /api/relay-physics/units/:unitId
```

**Purpose:** Query current unit state (derived from event log)

**Response Format:**
```json
{
  "id": "unit.alice.001",
  "state": "Working",
  "attached_filament": "work.W123",
  "current_task_filament": "work.W123",
  "position": [x, y, z],
  "rank": "Engineer"
}
```

**Status:** ✅ IMPLEMENTED (PR #1.0)

---

### 4. Render Scene Graphs (Layer 2→3 Contract)

#### 4a. World Scene

```
GET /api/relay-physics/render/world
```

**Purpose:** Complete scene graph for 3D world (globe + all SCVs + all filaments)

**Response Format:** (See RenderSpec v1 - Commit C)

**Includes:**
- Globe mesh (procedural sphere or ref)
- All units (positions, states, materials)
- All filaments (segments, anchors, materials, scars)
- Animation intents (keyed by event_id)

**Status:** ⏭️ FUTURE (after PR #1.3 + RenderSpec v1)

---

#### 4b. Commit Scene (Forensic Chamber)

```
GET /api/relay-physics/render/commit/:commitRef
```

**Purpose:** Scene graph for single commit visualization (forensic chamber)

**Response Format:** (See RenderSpec v1 - Commit C)

**Includes:**
- Time cube geometry (commit as 3D timebox)
- Causal links (arrows to inputs/authority)
- Author unit (SCV position at commit time)
- Evidence badges (git refs)
- Verification state (accepted/rejected glow)

**Status:** ⏭️ FUTURE (after PR #1.3 + RenderSpec v1)

---

#### 4c. Filament Scene (Single Thread)

```
GET /api/relay-physics/render/filament/:filamentId
```

**Purpose:** Scene graph for single filament (thread visualization)

**Response Format:** (See RenderSpec v1 - Commit C)

**Includes:**
- Filament segments (commit-to-commit ropes)
- Anchor points (filament head, tail)
- Scars (rejection marks)
- Rotation parameters (for encrypted channels)
- Thickness parameters (for load/importance)

**Status:** ⏭️ FUTURE (after PR #1.3 + RenderSpec v1)

---

## ENDPOINT PHILOSOPHY

### Layer 2 Outputs Data, Not Pixels

**Example:** Layer 2 says:
```json
{
  "type": "filament_segment",
  "material": "encrypted_channel",
  "rotation_params": {"twist_rate": 0.5, "direction": "clockwise"}
}
```

**Layer 3 interprets:**
- `encrypted_channel` → twisted rope shader
- `twist_rate: 0.5` → 180° twist over segment length
- Applies visual language (color, glow, animation)

### Semantic Tags, Not Artistic Choices

**✅ Good (Layer 2):**
```json
{"material": "encrypted_channel"}
{"material": "public_filament"}
{"material": "scv_working"}
```

**❌ Bad (Layer 2 should NOT specify):**
```json
{"color": "#FF5733"}
{"glow_intensity": 0.8}
{"shader": "phong"}
```

**Why:** Layer 3 owns visual language. Layer 2 owns semantics.

---

## CACHING + PERFORMANCE

### Event Stream (SSE)
- **No caching** (real-time)
- Headers: `Cache-Control: no-cache, no-store, no-transform`

### Commit Fetch
- **Immutable** (commits never change)
- Headers: `Cache-Control: public, max-age=31536000, immutable`

### Units
- **Ephemeral** (derived state changes)
- Headers: `Cache-Control: no-cache`

### Render Scenes
- **Short-lived cache** (derived from commits + units)
- Headers: `Cache-Control: private, max-age=60`

---

## VERSIONING

All endpoints include version in path: `/api/relay-physics/...`

**If breaking change needed:**
- Add `/api/relay-physics/v2/...`
- Deprecate v1 with warning headers
- Maintain v1 for 6 months minimum

**RenderSpec schema versioning:** See Commit C

---

## CORS + SECURITY

**CORS:**
```rust
CorsLayer::new()
    .allow_origin(Any) // MVP - tighten in production
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any)
```

**Authentication (Future):**
- Layer 2 does NOT handle auth (Layer 1 responsibility)
- Use JWT tokens from Layer 1
- Validate via middleware

---

## LOCKED INVARIANTS

1. **Event stream is real-time + replayable** (SSE with Last-Event-ID)
2. **Commits are immutable** (cache forever)
3. **Render endpoints output semantic data, not pixels** (tags not RGB)
4. **All responses are JSON** (except SSE which is `text/event-stream`)

---

## CAUSAL REFS

- **Inputs:** `architecture@c0` (Layer split decision)
- **Authority:** system.architect
- **Evidence:** 
  - PR #1.2 (SSE implementation proves feasibility)
  - HTTP caching best practices (immutable commit refs)

---

**Status:** LOCKED  
**Supersedes:** None  
**Next:** PR #1.3 (implement commit fetch), then RenderSpec v1 (lock scene graph format)

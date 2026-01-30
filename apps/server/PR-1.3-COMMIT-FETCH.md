# PR #1.3: Single Commit Fetch

**Status:** 🚧 IN PROGRESS  
**Date:** 2026-01-28  
**Depends on:** PR #1.2 (SSE Replay)  
**References:** `architecture@c1` (Render Endpoints Contract)

---

## GOAL

**Enable forensic inspection: fetch any commit by reference.**

**Endpoint:** `GET /api/relay-physics/commits/:commitRef`

**Unlocks:**
- "Click time cube → open forensic chamber" (Layer 3 UX)
- Debugging commit rejections (full causal graph)
- Training data annotation
- Audit trails

---

## SCOPE (LOCKED - MINIMAL)

### What PR #1.3 DOES
1. ✅ Fetch commit by `event_id` (from master event log)
2. ✅ Fetch commit by `filament@commit_index` (from per-filament log)
3. ✅ Return **raw truth bundle** (no rendering decisions)
4. ✅ HTTP 404 for missing commits
5. ✅ Immutable caching headers (`Cache-Control: public, immutable`)

### What PR #1.3 DOES NOT DO
- ❌ Fetch by "latest" or "head" (future enhancement)
- ❌ Fetch multiple commits at once (use SSE stream for that)
- ❌ Include derived render data (that's `/render/commit/:ref`)
- ❌ Modify FilamentStore or EventLog (read-only)

---

## ENDPOINT DESIGN

### URL Format

```
GET /api/relay-physics/commits/:commitRef
```

### CommitRef Formats (2 supported)

#### Format 1: Event ID
```
GET /api/relay-physics/commits/event:42
```

**Behavior:**
- Fetch from master event log by `event_id: 42`
- Return `EventLogEntry` wrapper (includes event_id, timestamp, schema_version)
- Only returns commits (filters out `UnitStateChanged` events)

**Use Case:** "Show me what happened at event 42"

---

#### Format 2: Filament Commit Reference
```
GET /api/relay-physics/commits/work.W123@c7
```

**Behavior:**
- Parse `work.W123@c7` → `filament_id: "work.W123"`, `commit_index: 7`
- Fetch from per-filament JSONL log (`var/relay_physics/filaments/work_W123.jsonl`)
- Return `CommitEvent` (already has all commit data)

**Use Case:** "Show me commit 7 on the work.W123 filament"

---

### Response Format

#### Success (200 OK)

**For `event:id` format:**
```json
{
  "source": "event_log",
  "event_id": 42,
  "schema_version": 1,
  "timestamp": "2026-01-28T10:30:00Z",
  "commit": {
    "commit_ref": "work.W123@c7",
    "filament_id": "work.W123",
    "commit_index": 7,
    "op_type": "OUTPUT_PROPOSED",
    "timestamp": "2026-01-28T10:30:00Z",
    "author_unit_ref": "unit.alice.001",
    "payload": {...},
    "causal_refs": {
      "inputs": ["work.W123@c6"],
      "authority_ref": "delegation.D001@c3",
      "evidence": ["git:abc123"]
    }
  }
}
```

**For `filament@cN` format:**
```json
{
  "source": "filament_log",
  "commit": {
    "commit_ref": "work.W123@c7",
    "filament_id": "work.W123",
    "commit_index": 7,
    "op_type": "OUTPUT_PROPOSED",
    "timestamp": "2026-01-28T10:30:00Z",
    "author_unit_ref": "unit.alice.001",
    "payload": {...},
    "causal_refs": {
      "inputs": ["work.W123@c6"],
      "authority_ref": "delegation.D001@c3",
      "evidence": ["git:abc123"]
    }
  }
}
```

**Key Difference:**
- Event log response includes `event_id` (monotonic event stream position)
- Filament log response does NOT include `event_id` (commit may exist in log but not yet in event stream)

---

#### Not Found (404)

```json
{
  "error": "COMMIT_NOT_FOUND",
  "commit_ref": "work.W123@c99",
  "message": "Commit not found: work.W123@c99",
  "details": "Filament 'work.W123' head is at c7 (requested c99)"
}
```

---

#### Bad Request (400)

```json
{
  "error": "INVALID_COMMIT_REF",
  "commit_ref": "invalid-format",
  "message": "Invalid commit ref format",
  "details": "Expected 'event:<id>' or '<filamentId>@c<index>'"
}
```

---

### Headers

**Request:**
```
GET /api/relay-physics/commits/work.W123@c7 HTTP/1.1
Accept: application/json
```

**Response (Success):**
```
HTTP/1.1 200 OK
Content-Type: application/json
Cache-Control: public, max-age=31536000, immutable
ETag: "work.W123@c7"
```

**Why immutable:**
- Commits never change (immutable truth)
- Clients can cache forever
- CDN/proxy can cache (reduces server load)

---

## DATA STRUCTURES

### CommitBundle (Response Envelope)

```rust
// apps/server/src/relay_physics/commit_bundle.rs

use serde::{Deserialize, Serialize};
use crate::relay_physics::CommitEvent;

/// Bundle returned by GET /commits/:ref
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitBundle {
    /// Where this commit was fetched from
    pub source: CommitSource,
    
    /// Event ID (only if from event log)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<u64>,
    
    /// Schema version (only if from event log)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<u32>,
    
    /// Timestamp of event log entry (only if from event log)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    
    /// The commit data
    pub commit: CommitEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitSource {
    EventLog,
    FilamentLog,
}

/// Error response for commit not found
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitNotFoundError {
    pub error: String, // "COMMIT_NOT_FOUND"
    pub commit_ref: String,
    pub message: String,
    pub details: String,
}
```

---

## IMPLEMENTATION

### 1. CommitRef Parsing

```rust
// apps/server/src/relay_physics/commit_bundle.rs

pub enum CommitRefFormat {
    EventId(u64),                     // event:42
    FilamentCommit(String, u64),      // work.W123@c7
}

impl CommitRefFormat {
    pub fn parse(s: &str) -> Result<Self, String> {
        // Format 1: event:<id>
        if let Some(stripped) = s.strip_prefix("event:") {
            let id = stripped.parse::<u64>()
                .map_err(|_| format!("Invalid event ID: {}", stripped))?;
            return Ok(Self::EventId(id));
        }
        
        // Format 2: <filamentId>@c<index>
        let parts: Vec<&str> = s.split("@c").collect();
        if parts.len() == 2 {
            let filament_id = parts[0].to_string();
            let commit_index = parts[1].parse::<u64>()
                .map_err(|_| format!("Invalid commit index: {}", parts[1]))?;
            return Ok(Self::FilamentCommit(filament_id, commit_index));
        }
        
        Err(format!("Invalid commit ref format: expected 'event:<id>' or '<filamentId>@c<index>', got '{}'", s))
    }
}
```

---

### 2. Handler Implementation

```rust
// apps/server/src/relay_physics/main_integration.rs

async fn get_commit_handler(
    State(state): State<AppState>,
    Path(commit_ref): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;
    
    // Parse commit ref format
    let format = match CommitRefFormat::parse(&commit_ref) {
        Ok(f) => f,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_COMMIT_REF",
                    "commit_ref": commit_ref,
                    "message": "Invalid commit ref format",
                    "details": e
                })),
            ).into_response();
        }
    };
    
    match format {
        CommitRefFormat::EventId(event_id) => {
            // Fetch from master event log
            let event_log = state.event_log.lock().await;
            match event_log.load_from(event_id) {
                Ok(entries) => {
                    // Find first entry (should be exactly one if event_id exists)
                    if let Some(entry) = entries.first() {
                        // Extract commit from event (if it's CommitAccepted)
                        if let RelayEvent::CommitAccepted { commit } = &entry.event {
                            let bundle = CommitBundle {
                                source: CommitSource::EventLog,
                                event_id: Some(entry.event_id),
                                schema_version: Some(entry.schema_version),
                                timestamp: Some(entry.timestamp.clone()),
                                commit: commit.clone(),
                            };
                            
                            (
                                StatusCode::OK,
                                [(header::CACHE_CONTROL, "public, max-age=31536000, immutable")],
                                Json(bundle),
                            ).into_response()
                        } else {
                            // Event is not a commit (e.g., UnitStateChanged)
                            (
                                StatusCode::NOT_FOUND,
                                Json(CommitNotFoundError {
                                    error: "COMMIT_NOT_FOUND".to_string(),
                                    commit_ref: format!("event:{}", event_id),
                                    message: format!("Event {} is not a commit", event_id),
                                    details: format!("Event type: {:?}", entry.event),
                                }),
                            ).into_response()
                        }
                    } else {
                        // Event ID doesn't exist
                        (
                            StatusCode::NOT_FOUND,
                            Json(CommitNotFoundError {
                                error: "COMMIT_NOT_FOUND".to_string(),
                                commit_ref: format!("event:{}", event_id),
                                message: format!("Event {} not found", event_id),
                                details: "Event log does not contain this event_id".to_string(),
                            }),
                        ).into_response()
                    }
                }
                Err(e) => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "EVENT_LOG_READ_ERROR",
                            "message": e.to_string()
                        })),
                    ).into_response()
                }
            }
        }
        
        CommitRefFormat::FilamentCommit(filament_id, commit_index) => {
            // Fetch from per-filament log
            let filament_store = state.filament_store.lock().await;
            let filament_id_obj = FilamentId::new(filament_id.clone());
            
            match filament_store.list_commits(&filament_id_obj) {
                Ok(commits) => {
                    // Find commit by index
                    if let Some(commit) = commits.iter().find(|c| c.commit_index == commit_index) {
                        let bundle = CommitBundle {
                            source: CommitSource::FilamentLog,
                            event_id: None,
                            schema_version: None,
                            timestamp: None,
                            commit: commit.clone(),
                        };
                        
                        (
                            StatusCode::OK,
                            [(header::CACHE_CONTROL, "public, max-age=31536000, immutable")],
                            Json(bundle),
                        ).into_response()
                    } else {
                        // Commit index doesn't exist
                        let head = filament_store.get_head_index(&filament_id_obj).unwrap_or(0);
                        (
                            StatusCode::NOT_FOUND,
                            Json(CommitNotFoundError {
                                error: "COMMIT_NOT_FOUND".to_string(),
                                commit_ref: format!("{}@c{}", filament_id, commit_index),
                                message: format!("Commit not found: {}@c{}", filament_id, commit_index),
                                details: format!("Filament '{}' head is at c{} (requested c{})", filament_id, head, commit_index),
                            }),
                        ).into_response()
                    }
                }
                Err(e) => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "FILAMENT_LOG_READ_ERROR",
                            "message": e.to_string()
                        })),
                    ).into_response()
                }
            }
        }
    }
}
```

---

### 3. Router Update

```rust
// apps/server/src/relay_physics/main_integration.rs

pub fn relay_router(state: AppState) -> Router {
    Router::new()
        .route("/api/relay-physics/events", get(events_sse_handler))
        .route(
            "/api/relay-physics/filaments/:id/commits",
            post(append_commit_handler).get(get_commits_handler),
        )
        .route(
            "/api/relay-physics/units",
            post(create_unit_handler).get(list_units_handler),
        )
        // PR #1.3: Single commit fetch
        .route(
            "/api/relay-physics/commits/:commit_ref",
            get(get_commit_handler),
        )
        .with_state(state)
}
```

---

## TESTS

### Test 1: Fetch by Event ID (Success)

```rust
#[tokio::test]
async fn test_fetch_commit_by_event_id() {
    // Setup: Create event log with commit event
    let mut event_log = EventLog::open("test_commit_fetch_event.jsonl").unwrap();
    let commit = CommitEvent::new(
        FilamentId::new("work.W123"),
        7,
        "OUTPUT_PROPOSED".to_string(),
        UnitId::new("unit.alice.001"),
        serde_json::json!({"output": "result"}),
        CausalRefs::default(),
    );
    let event = RelayEvent::CommitAccepted { commit: commit.clone() };
    let event_id = event_log.append(event).unwrap();
    
    // Test: Fetch by event:<id>
    let response = get_commit_handler(
        State(app_state),
        Path(format!("event:{}", event_id)),
    ).await;
    
    // Assert: 200 OK with commit bundle
    assert_eq!(response.status(), StatusCode::OK);
    let bundle: CommitBundle = serde_json::from_slice(&response.body()).unwrap();
    assert_eq!(bundle.source, CommitSource::EventLog);
    assert_eq!(bundle.event_id, Some(event_id));
    assert_eq!(bundle.commit.filament_id.as_str(), "work.W123");
    assert_eq!(bundle.commit.commit_index, 7);
}
```

---

### Test 2: Fetch by Filament@CommitIndex (Success)

```rust
#[tokio::test]
async fn test_fetch_commit_by_filament_ref() {
    // Setup: Create filament with commit
    let mut filament_store = FilamentStore::new("test_commit_fetch_filament").unwrap();
    let commit_data = CommitData {
        op_type: "TASK_ASSIGN".to_string(),
        author_unit_ref: UnitId::new("unit.manager.001"),
        payload: serde_json::json!({"task": "build"}),
        causal_refs: CausalRefs::default(),
    };
    let commit = filament_store.append_commit(
        FilamentId::new("work.W123"),
        commit_data,
        &verifier,
        &authority_store,
    ).unwrap();
    
    // Test: Fetch by work.W123@c1
    let response = get_commit_handler(
        State(app_state),
        Path("work.W123@c1".to_string()),
    ).await;
    
    // Assert: 200 OK with commit bundle
    assert_eq!(response.status(), StatusCode::OK);
    let bundle: CommitBundle = serde_json::from_slice(&response.body()).unwrap();
    assert_eq!(bundle.source, CommitSource::FilamentLog);
    assert_eq!(bundle.event_id, None); // No event_id from filament log
    assert_eq!(bundle.commit.commit_index, 1);
}
```

---

### Test 3: 404 for Missing Event ID

```rust
#[tokio::test]
async fn test_fetch_commit_event_not_found() {
    let response = get_commit_handler(
        State(app_state),
        Path("event:999999".to_string()),
    ).await;
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let error: CommitNotFoundError = serde_json::from_slice(&response.body()).unwrap();
    assert_eq!(error.error, "COMMIT_NOT_FOUND");
    assert_eq!(error.commit_ref, "event:999999");
}
```

---

### Test 4: 404 for Missing Filament Commit

```rust
#[tokio::test]
async fn test_fetch_commit_filament_not_found() {
    let response = get_commit_handler(
        State(app_state),
        Path("work.W123@c999".to_string()),
    ).await;
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let error: CommitNotFoundError = serde_json::from_slice(&response.body()).unwrap();
    assert!(error.details.contains("head is at"));
}
```

---

### Test 5: 400 for Invalid Format

```rust
#[tokio::test]
async fn test_fetch_commit_invalid_format() {
    let response = get_commit_handler(
        State(app_state),
        Path("invalid-format".to_string()),
    ).await;
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error: serde_json::Value = serde_json::from_slice(&response.body()).unwrap();
    assert_eq!(error["error"], "INVALID_COMMIT_REF");
}
```

---

## FILES TO CREATE/MODIFY

### NEW FILES
```
apps/server/src/relay_physics/commit_bundle.rs  (~150 lines)
  - CommitBundle struct
  - CommitSource enum
  - CommitRefFormat parsing
  - CommitNotFoundError struct
```

### MODIFIED FILES
```
apps/server/src/relay_physics/mod.rs             (+1 line - add commit_bundle module)
apps/server/src/relay_physics/main_integration.rs (+80 lines - add get_commit_handler)
apps/server/src/relay_physics/filament_store.rs  (+5 lines - expose get_head_index as pub)
```

### TEST FILES
```
apps/server/src/relay_physics/commit_fetch_tests.rs (NEW, ~200 lines - 5 tests)
```

---

## INVARIANTS (LOCKED)

1. **Commits are immutable** → Cache forever (`Cache-Control: immutable`)
2. **Event log is source of truth** → Event ID always maps to same commit
3. **Filament logs are deterministic** → Same filament state → same commit at index N
4. **No rendering decisions** → Bundle contains only raw truth (no colors, materials, etc.)
5. **404 is not an error** → Missing commit returns structured error (not panic)

---

## WHAT THIS UNLOCKS

### ✅ For Layer 3 (Frontend)
- "Click time cube → fetch commit → render forensic chamber"
- Display causal graph (inputs, authority, evidence)
- Show commit details (op_type, payload, timestamp)

### ✅ For Debugging
- `curl /api/relay-physics/commits/work.W123@c7` → inspect any commit
- See full causal chain (inputs refs)
- Understand why commit was accepted/rejected

### ✅ For Training Pipeline
- Fetch commits by event_id for annotation
- Export commit bundles as training data
- Label op_types for LLM fine-tuning

### ✅ For Future PRs
- `/render/commit/:ref` will use this internally
- RenderSpec v1 can reference commit structure
- Forensic chamber UI can build on this API

---

## NEXT STEPS (AFTER PR #1.3)

1. ✅ **Lock RenderSpec v1** - Use CommitBundle structure to inform schema
2. ⏭️ **Implement `/render/commit/:ref`** - Derived scene graph for forensic chamber
3. ⏭️ **Implement `/render/world`** - Full scene graph (globe + SCVs + filaments)
4. ⏭️ **Layer 3 frontend** - React + Three.js rendering

---

**Status:** 🚧 READY TO IMPLEMENT  
**Estimated Effort:** ~4 hours  
**Philosophy:** Raw truth first, rendering later. Commits are immutable. The log is authority.

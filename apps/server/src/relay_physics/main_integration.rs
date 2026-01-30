// apps/server/src/relay_physics/main_integration.rs
// 
// This file shows how to integrate Relay Physics routes into main.rs
// COPY this code into your actual main.rs, keeping existing Git routes intact

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::relay_physics::{
    agent_ops, CommitData, CommitEvent, CommitRejectedError, EventBus, EventLog, FilamentId, FilamentStore,
    Unit, UnitId, UnitStore, Verifier, AuthorityStore, RelayEvent, sse_stream_with_replay, CommitProcessor,
    events::EVENT_BUS_CAPACITY, CommitBundle, CommitSource, CommitRefFormat, CommitNotFoundError,
    renderspec_generator, BuildingStore, seed_buildings_if_empty, Building, BuildingId,
    TaskStore, Task, TaskId, TaskType, TaskState, TaskPriority, TaskProgress,
    ShipmentStore, Shipment, ShipmentId, ShipmentState, GeoPosition, CarrierType, GeoAnchor,
    generate_simple_route, estimate_travel_time,
    IdentityStore, IdentityLog, IdentityId, IdentityPayload, TargetKind, check_access,
    BindingType, AttestationType, ScarType,
};

#[derive(Clone)]
pub struct AppState {
    pub filament_store: Arc<Mutex<FilamentStore>>,
    pub unit_store: Arc<Mutex<UnitStore>>,
    pub authority_store: Arc<Mutex<AuthorityStore>>,
    pub building_store: Arc<Mutex<BuildingStore>>, // PR #6
    pub task_store: Arc<Mutex<TaskStore>>, // PR #7
    pub shipment_store: Arc<Mutex<ShipmentStore>>, // PR #8
    pub identity_store: Arc<Mutex<IdentityStore>>, // PR #9
    pub verifier: Arc<Verifier>,
    pub events: EventBus,
    pub event_log: Arc<Mutex<EventLog>>,
}

pub fn init_relay_physics_state() -> AppState {
    // PR #1.1: DETERMINISTIC STATE RECONSTRUCTION FROM EVENT LOG
    
    // 1. Open event log (creates if doesn't exist)
    let event_log = EventLog::open("var/relay_physics/events.jsonl")
        .expect("failed to open event log");
    
    // 2. Load all events for replay
    let events_to_replay = event_log.load_all()
        .expect("failed to load events from log");
    
    println!("🔄 Replaying {} events to reconstruct state...", events_to_replay.len());
    
    // 3. Initialize stores (empty)
    let filament_store = FilamentStore::new("var/relay_physics/filaments")
        .expect("failed to init FilamentStore");
    
    let mut unit_store = UnitStore::new();
    let mut authority_store = AuthorityStore::new();
    let mut building_store = BuildingStore::new(); // PR #6
    let mut task_store = TaskStore::new(); // PR #7
    let mut shipment_store = ShipmentStore::new(); // PR #8
    let mut identity_store = IdentityStore::new(); // PR #9
    
    // 4. Replay events to reconstruct state
    let relay_events: Vec<RelayEvent> = events_to_replay.iter().map(|e| e.event.clone()).collect();
    unit_store.replay_from_events(&relay_events);
    authority_store.replay_from_events(&relay_events);
    building_store.replay_from_events(&events_to_replay); // PR #6
    task_store.replay_from_events(&events_to_replay); // PR #7
    shipment_store.replay_from_events(&events_to_replay); // PR #8
    
    // 5. Seed buildings if empty (PR #6 - architecture@c9)
    let event_log_arc = Arc::new(Mutex::new(event_log));
    let event_bus = EventBus::new(EVENT_BUS_CAPACITY);
    
    seed_buildings_if_empty(&mut building_store, &event_log_arc, &event_bus)
        .expect("failed to seed buildings");
    
    // 6. Apply static authority grants (MVP - until PR #4 full delegation)
    authority_store.grant_op(&UnitId::new("unit.manager.001"), agent_ops::TASK_ASSIGN);
    authority_store.grant_op(&UnitId::new("unit.manager.001"), agent_ops::SCV_CHANNEL_CANCEL);
    authority_store.grant_op(&UnitId::new("unit.manager.001"), agent_ops::OUTPUT_PROPOSED);
    
    println!("✅ State reconstructed. Units: {}, Buildings: {}, Tasks: {}, Shipments: {}, Next event ID: {}", 
             unit_store.list_units().len(),
             building_store.list_all().len(),
             task_store.list_all().len(),
             shipment_store.count(),
             event_log_arc.lock().unwrap().next_event_id());

    AppState {
        filament_store: Arc::new(Mutex::new(filament_store)),
        unit_store: Arc::new(Mutex::new(unit_store)),
        authority_store: Arc::new(Mutex::new(authority_store)),
        building_store: Arc::new(Mutex::new(building_store)), // PR #6
        task_store: Arc::new(Mutex::new(task_store)), // PR #7
        shipment_store: Arc::new(Mutex::new(shipment_store)), // PR #8
        identity_store: Arc::new(Mutex::new(identity_store)), // PR #9
        verifier: Arc::new(Verifier::new()),
        events: event_bus,
        event_log: event_log_arc,
    }
}

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
        // PR #1.3: Single commit fetch (forensic inspection)
        .route(
            "/api/relay-physics/commits/:commit_ref",
            get(get_commit_handler),
        )
        // PR #6: Buildings endpoints (architecture@c9)
        .route(
            "/api/relay-physics/buildings",
            post(register_building_handler).get(list_buildings_handler),
        )
        // PR #7: Task endpoints (architecture@c9 - Shopping as Production)
        .route(
            "/api/relay-physics/tasks",
            post(create_task_handler).get(list_tasks_handler),
        )
        .route(
            "/api/relay-physics/tasks/:id/progress",
            patch(progress_task_handler),
        )
        // PR #8: Shipment endpoints (architecture@c9 - Drones in flight)
        .route(
            "/api/relay-physics/shipments",
            post(create_shipment_handler).get(list_shipments_handler),
        )
        .route(
            "/api/relay-physics/shipments/:id",
            get(get_shipment_handler),
        )
        .route(
            "/api/relay-physics/shipments/:id/arrive",
            patch(arrive_shipment_handler),
        )
        // PR #9: Identity endpoints (architecture@c10 - Users as filament trees)
        .route(
            "/api/relay-physics/identities/:id",
            get(get_identity_handler),
        )
        .route(
            "/api/relay-physics/identities/:id/commits",
            get(list_identity_commits_handler).post(append_identity_commit_handler),
        )
        .route(
            "/api/relay-physics/access/check",
            post(check_access_handler),
        )
        // Option C - Backend: Render endpoints (RenderSpec v1)
        .route("/api/relay-physics/render/world", get(render_world_handler))
        .route("/api/relay-physics/render/commit/:commit_ref", get(render_commit_handler))
        .route("/api/relay-physics/render/filament/:id", get(render_filament_handler))
        .with_state(state)
}

// PR #1.2: SSE handler with replay + Last-Event-ID support
#[derive(Debug, Deserialize)]
struct EventsQuery {
    after: Option<u64>, // ?after=N query param
}

async fn events_sse_handler(
    State(state): State<AppState>,
    Query(query): Query<EventsQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    use axum::response::Response;
    use axum::http::header;
    
    // PR #1.2: Parse reconnection cursor
    // Priority: Last-Event-ID header > ?after query param
    let start_event_id = headers
        .get("Last-Event-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .or(query.after)
        .map(|id| id + 1); // Client sends last received; we start from next
    
    if let Some(id) = start_event_id {
        println!("📡 SSE reconnection from event_id {}", id);
    } else {
        println!("📡 SSE new connection (live only)");
    }
    
    // Clone event log for replay
    let event_log = {
        let log = state.event_log.lock().await;
        // Clone the EventLog internals (need to make EventLog cloneable or pass Arc)
        // For now, we'll pass the path and reopen
        EventLog::open("var/relay_physics/events.jsonl").expect("failed to open event log")
    };
    
    let sse = sse_stream_with_replay(event_log, state.events.clone(), start_event_id);
    
    // PR #1.2 LOCK: Production headers for proxy/CDN compatibility
    Response::builder()
        // Q2: Add no-transform for maximum proxy compatibility
        .header(header::CACHE_CONTROL, "no-cache, no-store, no-transform, must-revalidate")
        .header(header::CONNECTION, "keep-alive")
        .header("X-Accel-Buffering", "no") // Nginx: disable buffering
        .body(axum::body::Body::from_stream(sse.into_stream()))
        .unwrap()
}

// ---------- Filaments ----------

#[derive(Debug, Deserialize)]
struct AppendCommitRequest {
    pub op_type: String,
    pub author_unit_ref: String,
    #[serde(default)]
    pub payload: serde_json::Value,
    #[serde(default)]
    pub causal_refs: crate::relay_physics::CausalRefs,
}

#[derive(Debug, Serialize)]
#[serde(tag = "result", content = "data")]
enum AppendCommitResponse {
    CommitAccepted { commit: CommitEvent },
    CommitRejected { error: CommitRejectedError },
}

async fn append_commit_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AppendCommitRequest>,
) -> impl IntoResponse {
    let filament_id = FilamentId::new(id);
    let commit_data = CommitData {
        op_type: req.op_type,
        author_unit_ref: UnitId::new(req.author_unit_ref),
        payload: req.payload,
        causal_refs: req.causal_refs,
    };

    let verifier = state.verifier.clone();

    // Lock stores
    let mut fs = state.filament_store.lock().await;
    let mut us = state.unit_store.lock().await;
    let auth = state.authority_store.lock().await;

    let result = fs.append_commit(filament_id, commit_data, &verifier, &auth);

    match result {
        Ok(commit) => {
            let event = RelayEvent::CommitAccepted { commit: commit.clone() };
            
            // PR #1.2 LOCK: Persist to log MUST succeed before emit
            // "If it's not in the log, it never happened" (history is authority)
            let event_id = {
                let mut log = state.event_log.lock().await;
                match log.append(event.clone()) {
                    Ok(id) => id,
                    Err(e) => {
                        eprintln!("🚨 FATAL: Failed to persist event to log: {}", e);
                        // Q4 FIX: Use 503 (not 500) - storage failure is retryable
                        return (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(AppendCommitResponse::CommitRejected {
                                error: CommitRejectedError::new(
                                    crate::relay_physics::ReasonCode::CustomRuleFailed,
                                    format!("Event log temporarily unavailable: {}", e),
                                ),
                            }),
                        ).into_response();
                    }
                }
            };
            
            // PR #1.2: Emit with ID to live subscribers (only after successful persist)
            state.events.emit_with_id(event_id, event);
            
            // Process commit → SCV state transitions (LOCKED: commit-driven only)
            CommitProcessor::process_commit(&commit, &mut us, &state.events, &state.event_log);
            
            (StatusCode::OK, Json(AppendCommitResponse::CommitAccepted { commit }))
        }
        Err(err) => {
            let event = RelayEvent::CommitRejected { error: err.clone() };
            
            // PR #1.2 LOCK: Persist rejection (or fail request)
            let event_id = {
                let mut log = state.event_log.lock().await;
                match log.append(event.clone()) {
                    Ok(id) => id,
                    Err(e) => {
                        eprintln!("🚨 FATAL: Failed to persist rejection to log: {}", e);
                        // Return HTTP 503 (service unavailable) - storage failure
                        return (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(AppendCommitResponse::CommitRejected {
                                error: CommitRejectedError::new(
                                    crate::relay_physics::ReasonCode::CustomRuleFailed,
                                    format!("Event log unavailable: {}", e),
                                ),
                            }),
                        ).into_response();
                    }
                }
            };
            
            // PR #1.2: Emit with ID
            state.events.emit_with_id(event_id, event);
            (StatusCode::BAD_REQUEST, Json(AppendCommitResponse::CommitRejected { error: err }))
        }
    }
}

async fn get_commits_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let filament_id = FilamentId::new(id);
    let fs = state.filament_store.lock().await;

    match fs.list_commits(&filament_id) {
        Ok(commits) => (StatusCode::OK, Json(commits)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("read error: {e}"))).into_response(),
    }
}

// ---------- Units (SCVs) ----------

#[derive(Debug, Deserialize)]
struct CreateUnitRequest {
    pub unit_id: String,
}

async fn create_unit_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateUnitRequest>,
) -> impl IntoResponse {
    let mut us = state.unit_store.lock().await;
    let unit = us.create_unit(UnitId::new(req.unit_id));

    let event = RelayEvent::UnitStateChanged { unit: unit.clone() };
    
    // PR #1.2 LOCK: Persist MUST succeed
    let event_id = {
        let mut log = state.event_log.lock().await;
        match log.append(event.clone()) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("🚨 FATAL: Failed to persist unit creation to log: {}", e);
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(serde_json::json!({
                        "error": "Event log unavailable",
                        "details": e.to_string()
                    })),
                ).into_response();
            }
        }
    };
    
    // PR #1.2: Emit with ID (only after successful persist)
    state.events.emit_with_id(event_id, event);
    (StatusCode::OK, Json(unit))
}

async fn list_units_handler(State(state): State<AppState>) -> impl IntoResponse {
    let us = state.unit_store.lock().await;
    (StatusCode::OK, Json(us.list_units()))
}

// ---------- Commits (Forensic Inspection) ----------

/// PR #1.3: Fetch single commit by reference
/// 
/// Supported formats:
/// - `event:42` - Fetch from master event log by event_id
/// - `work.W123@c7` - Fetch from per-filament log
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
            let mut filament_store = state.filament_store.lock().await;
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

// ---------- Render Endpoints (RenderSpec v1) ----------

/// Option C - Backend: GET /render/world
/// 
/// Generates complete world scene (globe + all units + all filaments)
async fn render_world_handler(State(state): State<AppState>) -> impl IntoResponse {
    use axum::http::header;
    use std::collections::HashMap;
    
    // Lock stores (read-only)
    let unit_store = state.unit_store.lock().await;
    let mut filament_store = state.filament_store.lock().await;
    let building_store = state.building_store.lock().await; // PR #6
    let task_store = state.task_store.lock().await; // PR #7
    let mut shipment_store = state.shipment_store.lock().await; // PR #8
    
    // Get all units
    let units = unit_store.list_units();
    
    // Get all buildings (PR #6)
    let buildings = building_store.list_all();
    
    // Get all tasks (PR #7)
    let tasks = task_store.list_all();
    
    // Get all shipments and update their positions (PR #8)
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let mut shipments = shipment_store.list_all();
    // Recalculate current positions for all active shipments
    for shipment in &mut shipments {
        if shipment.state == ShipmentState::InTransit || shipment.state == ShipmentState::Created {
            shipment.current_position = ShipmentStore::interpolate_position(shipment, current_time);
            shipment.progress_percentage = ((current_time.saturating_sub(shipment.created_at) as f64)
                / (shipment.estimated_arrival.saturating_sub(shipment.created_at) as f64))
                .min(1.0) as f32;
        }
    }
    
    // Get all filaments (MVP: scan directory for all *.jsonl files)
    // TODO: FilamentStore should track known filaments
    let mut filaments_map = HashMap::new();
    
    // For MVP, we'll just get filaments that have commits
    // In production, FilamentStore would maintain an index
    
    // Generate RenderSpec v1 (now includes buildings + tasks + shipments - PR #6 + PR #7 + PR #8)
    let spec = renderspec_generator::generate_world_scene(&units, &filaments_map, &buildings, &tasks, &shipments);
    
    (
        StatusCode::OK,
        [(header::CACHE_CONTROL, "private, max-age=60")], // Short cache (derived state)
        [(header::CONTENT_TYPE, "application/json")],
        Json(spec),
    )
}

/// Option C - Backend: GET /render/commit/:ref
/// 
/// Generates forensic chamber scene for single commit
async fn render_commit_handler(
    State(state): State<AppState>,
    Path(commit_ref): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;
    
    // Parse commit ref
    let format = match CommitRefFormat::parse(&commit_ref) {
        Ok(f) => f,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_COMMIT_REF",
                    "details": e
                })),
            ).into_response();
        }
    };
    
    // Fetch commit (reuse commit fetch logic)
    let commit = match format {
        CommitRefFormat::EventId(event_id) => {
            let event_log = state.event_log.lock().await;
            match event_log.load_from(event_id) {
                Ok(entries) => {
                    if let Some(entry) = entries.first() {
                        if let RelayEvent::CommitAccepted { commit } = &entry.event {
                            Some(commit.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        CommitRefFormat::FilamentCommit(filament_id, commit_index) => {
            let filament_store = state.filament_store.lock().await;
            let filament_id_obj = FilamentId::new(filament_id);
            
            match filament_store.list_commits(&filament_id_obj) {
                Ok(commits) => {
                    commits.iter().find(|c| c.commit_index == commit_index).cloned()
                }
                Err(_) => None,
            }
        }
    };
    
    match commit {
        Some(commit) => {
            let spec = renderspec_generator::generate_commit_scene(&commit);
            
            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "private, max-age=300")], // 5 min cache
                [(header::CONTENT_TYPE, "application/json")],
                Json(spec),
            ).into_response()
        }
        None => {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "COMMIT_NOT_FOUND",
                    "commit_ref": commit_ref
                })),
            ).into_response()
        }
    }
}

/// Option C - Backend: GET /render/filament/:id
/// 
/// Generates filament thread scene (polyline + timeboxes)
async fn render_filament_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;
    
    let filament_id = FilamentId::new(id.clone());
    let filament_store = state.filament_store.lock().await;
    
    match filament_store.list_commits(&filament_id) {
        Ok(commits) => {
            if commits.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "FILAMENT_NOT_FOUND",
                        "filament_id": id
                    })),
                ).into_response();
            }
            
            let spec = renderspec_generator::generate_filament_scene(&filament_id, &commits);
            
            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "private, max-age=60")], // Short cache
                [(header::CONTENT_TYPE, "application/json")],
                Json(spec),
            ).into_response()
        }
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "FILAMENT_READ_ERROR",
                    "message": e.to_string()
                })),
            ).into_response()
        }
    }
}

// ============================================================================
// PR #6: BUILDING ENDPOINTS (architecture@c9)
// ============================================================================

/// GET /api/relay-physics/buildings
/// List all buildings
async fn list_buildings_handler(State(state): State<AppState>) -> impl IntoResponse {
    use axum::http::header;
    
    let building_store = state.building_store.lock().await;
    let buildings = building_store.list_all();
    
    (
        StatusCode::OK,
        [(header::CACHE_CONTROL, "private, max-age=60")], // Derived state
        [(header::CONTENT_TYPE, "application/json")],
        Json(buildings),
    )
}

/// POST /api/relay-physics/buildings
/// Register a new building
async fn register_building_handler(
    State(state): State<AppState>,
    Json(building): Json<Building>,
) -> impl IntoResponse {
    // Create BuildingRegistered event
    let event = RelayEvent::BuildingRegistered {
        building: building.clone(),
    };
    
    // Persist to event log
    let mut event_log = state.event_log.lock().await;
    let event_id = match event_log.append(event.clone()) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "EVENT_LOG_ERROR",
                    "message": e.to_string()
                })),
            ).into_response();
        }
    };
    drop(event_log);
    
    // Emit to event bus
    state.events.emit_with_id(event_id, event);
    
    // Apply to building store
    let mut building_store = state.building_store.lock().await;
    match building_store.register(building.clone()) {
        Ok(_) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "result": "BuildingRegistered",
                    "event_id": event_id,
                    "building": building
                })),
            ).into_response()
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "BUILDING_REGISTRATION_FAILED",
                    "message": e
                })),
            ).into_response()
        }
    }
}

// ============================================================================
// PR #7: TASK ENDPOINTS (architecture@c9 - Shopping as Production)
// ============================================================================

/// GET /api/relay-physics/tasks
/// List all tasks (optionally filtered by requester, building, or state)
#[derive(Debug, Deserialize)]
struct TasksQuery {
    requester_ref: Option<String>,
    building_ref: Option<String>,
    state: Option<String>,
}

async fn list_tasks_handler(
    State(state): State<AppState>,
    Query(query): Query<TasksQuery>,
) -> impl IntoResponse {
    use axum::http::header;
    
    let task_store = state.task_store.lock().await;
    
    let tasks = if let Some(requester_ref) = query.requester_ref {
        task_store.list_by_requester(&UnitId::new(requester_ref))
    } else if let Some(building_ref) = query.building_ref {
        task_store.list_by_building(&BuildingId::new(building_ref))
    } else if let Some(state_str) = query.state {
        // Parse state string to TaskState enum
        let task_state = match state_str.as_str() {
            "queued" => TaskState::Queued,
            "packing" => TaskState::Packing,
            "dispatched" => TaskState::Dispatched,
            "in_transit" => TaskState::InTransit,
            "delivered" => TaskState::Delivered,
            "failed" => TaskState::Failed,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "INVALID_STATE",
                        "message": format!("Invalid state: {}", state_str)
                    })),
                ).into_response();
            }
        };
        task_store.list_by_state(task_state)
    } else {
        task_store.list_all()
    };
    
    (
        StatusCode::OK,
        [(header::CACHE_CONTROL, "private, max-age=5")], // Very short cache (frequently updated)
        [(header::CONTENT_TYPE, "application/json")],
        Json(tasks),
    ).into_response()
}

/// POST /api/relay-physics/tasks
/// Create a new task (add to build queue)
#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    task_type: String,
    building_ref: String,
    requester_ref: String,
    product_ref: Option<String>,
    priority: Option<String>,
}

async fn create_task_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    // Parse task_type
    let task_type = match req.task_type.as_str() {
        "production" => TaskType::Production,
        "shipment" => TaskType::Shipment,
        "verification" => TaskType::Verification,
        "commitment" => TaskType::Commitment,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_TASK_TYPE",
                    "message": format!("Invalid task type: {}", req.task_type)
                })),
            ).into_response();
        }
    };
    
    // Parse priority
    let priority = match req.priority.as_deref() {
        Some("background") => TaskPriority::Background,
        Some("urgent") => TaskPriority::Urgent,
        Some("normal") | None => TaskPriority::Normal,
        Some(p) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_PRIORITY",
                    "message": format!("Invalid priority: {}", p)
                })),
            ).into_response();
        }
    };
    
    // Verify building exists
    {
        let building_store = state.building_store.lock().await;
        if building_store.get(&BuildingId::new(&req.building_ref)).is_none() {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "BUILDING_NOT_FOUND",
                    "message": format!("Building {} not found", req.building_ref)
                })),
            ).into_response();
        }
    }
    
    // Generate task ID (deterministic from inputs + timestamp)
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let task_id_str = format!(
        "task.{}.{}.{}",
        req.building_ref.replace("building.", ""),
        req.product_ref.as_deref().unwrap_or("generic"),
        timestamp
    );
    
    // Create task
    let task = Task {
        task_id: TaskId::new(task_id_str),
        task_type,
        building_ref: BuildingId::new(req.building_ref),
        requester_ref: UnitId::new(req.requester_ref),
        product_ref: req.product_ref,
        state: TaskState::Queued,
        progress: TaskProgress::default(),
        estimated_completion: timestamp + 3600, // 1 hour from now (placeholder)
        priority,
        failure_count: 0,
        props: serde_json::Map::new(),
    };
    
    // Create TaskCreated event
    let event = RelayEvent::TaskCreated {
        task: task.clone(),
    };
    
    // Persist to event log
    let mut event_log = state.event_log.lock().await;
    let event_id = match event_log.append(event.clone()) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "EVENT_LOG_ERROR",
                    "message": e.to_string()
                })),
            ).into_response();
        }
    };
    drop(event_log);
    
    // Emit to event bus
    state.events.emit_with_id(event_id, event);
    
    // Apply to task store
    let mut task_store = state.task_store.lock().await;
    match task_store.create(task.clone()) {
        Ok(_) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "result": "TaskCreated",
                    "event_id": event_id,
                    "task": task
                })),
            ).into_response()
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "TASK_CREATION_FAILED",
                    "message": e
                })),
            ).into_response()
        }
    }
}

/// PATCH /api/relay-physics/tasks/:id/progress
/// Update task progress (state transition)
#[derive(Debug, Deserialize)]
struct ProgressTaskRequest {
    to_state: String,
    progress_percentage: Option<f32>,
}

async fn progress_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ProgressTaskRequest>,
) -> impl IntoResponse {
    // Parse to_state
    let to_state = match req.to_state.as_str() {
        "queued" => TaskState::Queued,
        "packing" => TaskState::Packing,
        "dispatched" => TaskState::Dispatched,
        "in_transit" => TaskState::InTransit,
        "delivered" => TaskState::Delivered,
        "failed" => TaskState::Failed,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_STATE",
                    "message": format!("Invalid state: {}", req.to_state)
                })),
            ).into_response();
        }
    };
    
    let progress_percentage = req.progress_percentage.unwrap_or(0.0);
    let task_id = TaskId::new(id);
    
    // Create TaskProgressed event
    let event = RelayEvent::TaskProgressed {
        task_id: task_id.clone(),
        to_state,
        progress_percentage,
    };
    
    // Persist to event log
    let mut event_log = state.event_log.lock().await;
    let event_id = match event_log.append(event.clone()) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "EVENT_LOG_ERROR",
                    "message": e.to_string()
                })),
            ).into_response();
        }
    };
    drop(event_log);
    
    // Emit to event bus
    state.events.emit_with_id(event_id, event);
    
    // Apply to task store
    let mut task_store = state.task_store.lock().await;
    match task_store.progress(&task_id, to_state, progress_percentage) {
        Ok(_) => {
            let task = task_store.get(&task_id).cloned();
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "result": "TaskProgressed",
                    "event_id": event_id,
                    "task": task
                })),
            ).into_response()
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "TASK_PROGRESS_FAILED",
                    "message": e
                })),
            ).into_response()
        }
    }
}

// ============================================================================
// PR #8: SHIPMENT ENDPOINTS (architecture@c9 - Drones in flight)
// ============================================================================

/// GET /api/relay-physics/shipments
/// List all shipments (optionally filtered)
#[derive(Debug, Deserialize)]
struct ShipmentsQuery {
    task_ref: Option<String>,
    origin_building: Option<String>,
    destination_building: Option<String>,
    state: Option<String>,
}

async fn list_shipments_handler(
    State(state): State<AppState>,
    Query(query): Query<ShipmentsQuery>,
) -> impl IntoResponse {
    use axum::http::header;
    
    let shipment_store = state.shipment_store.lock().await;
    
    let shipments = if let Some(task_ref) = query.task_ref {
        shipment_store.list_by_task(&TaskId::new(task_ref))
    } else if let Some(origin_ref) = query.origin_building {
        shipment_store.list_by_origin(&BuildingId::new(origin_ref))
    } else if let Some(dest_ref) = query.destination_building {
        shipment_store.list_by_destination(&BuildingId::new(dest_ref))
    } else if let Some(state_str) = query.state {
        let shipment_state = match state_str.as_str() {
            "created" => ShipmentState::Created,
            "in_transit" => ShipmentState::InTransit,
            "arrived" => ShipmentState::Arrived,
            "failed" => ShipmentState::Failed,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "INVALID_STATE",
                        "message": format!("Invalid state: {}", state_str)
                    })),
                ).into_response();
            }
        };
        shipment_store.list_by_state(shipment_state)
    } else {
        shipment_store.list_all()
    };
    
    (
        StatusCode::OK,
        [(header::CACHE_CONTROL, "private, max-age=2")], // Very short cache (positions update frequently)
        [(header::CONTENT_TYPE, "application/json")],
        Json(shipments),
    ).into_response()
}

/// GET /api/relay-physics/shipments/:id
/// Get a single shipment by ID with current position
async fn get_shipment_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;
    
    let shipment_id = ShipmentId::new(id);
    let shipment_store = state.shipment_store.lock().await;
    
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    match shipment_store.get(&shipment_id) {
        Some(mut shipment) => {
            // Recalculate current position
            shipment.current_position = ShipmentStore::interpolate_position(&shipment, current_time);
            shipment.progress_percentage = ((current_time.saturating_sub(shipment.created_at) as f64)
                / (shipment.estimated_arrival.saturating_sub(shipment.created_at) as f64))
                .min(1.0) as f32;
            
            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "private, max-age=2")],
                [(header::CONTENT_TYPE, "application/json")],
                Json(shipment),
            ).into_response()
        }
        None => {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "SHIPMENT_NOT_FOUND",
                    "message": format!("Shipment {} not found", shipment_id)
                })),
            ).into_response()
        }
    }
}

/// POST /api/relay-physics/shipments
/// Create a new shipment (spawn drone)
#[derive(Debug, Deserialize)]
struct CreateShipmentRequest {
    task_ref: String,
    carrier_type: Option<String>,
}

async fn create_shipment_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateShipmentRequest>,
) -> impl IntoResponse {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let task_id = TaskId::new(&req.task_ref);
    
    // Parse carrier type
    let carrier_type = match req.carrier_type.as_deref() {
        Some("ground") => CarrierType::Ground,
        Some("air") => CarrierType::Air,
        Some("drone") | None => CarrierType::Drone,
        Some(c) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_CARRIER_TYPE",
                    "message": format!("Invalid carrier type: {}", c)
                })),
            ).into_response();
        }
    };
    
    // Get task to verify it exists and is in Dispatched state
    let task_store = state.task_store.lock().await;
    let task = match task_store.get(&task_id) {
        Some(t) => t.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "TASK_NOT_FOUND",
                    "message": format!("Task {} not found", task_id)
                })),
            ).into_response();
        }
    };
    drop(task_store);
    
    if task.state != TaskState::Dispatched {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "TASK_NOT_READY",
                "message": format!("Task must be in Dispatched state (currently {})", task.state)
            })),
        ).into_response();
    }
    
    // Get origin and destination buildings
    let building_store = state.building_store.lock().await;
    let origin_building = match building_store.get(&task.building_ref) {
        Some(b) => b.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "BUILDING_NOT_FOUND",
                    "message": format!("Origin building {} not found", task.building_ref)
                })),
            ).into_response();
        }
    };
    
    // For MVP, assume destination is same as origin (local pickup)
    // In real system, would get requester's location or specified destination
    let destination_building = origin_building.clone();
    drop(building_store);
    
    // Generate shipment ID
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let shipment_id = ShipmentId::new(format!(
        "shipment.{}.{}",
        carrier_type,
        timestamp
    ));
    
    // Generate route
    let route = generate_simple_route(
        &origin_building.geo_anchor,
        &destination_building.geo_anchor,
        carrier_type,
    );
    
    // Calculate ETA
    let travel_time = estimate_travel_time(
        &origin_building.geo_anchor,
        &destination_building.geo_anchor,
        carrier_type,
    );
    let estimated_arrival = timestamp + travel_time;
    
    // Create shipment
    let shipment = Shipment {
        shipment_id: shipment_id.clone(),
        task_ref: task_id.clone(),
        origin_building: origin_building.building_id.clone(),
        destination_building: destination_building.building_id.clone(),
        current_position: route.first().cloned().unwrap_or_else(|| GeoPosition::new(0.0, 0.0, 0.0)),
        route,
        state: ShipmentState::Created,
        progress_percentage: 0.0,
        created_at: timestamp,
        estimated_arrival,
        actual_arrival: None,
        carrier_type,
        props: serde_json::Map::new(),
    };
    
    // Create event
    let event = RelayEvent::ShipmentCreated {
        shipment: shipment.clone(),
        triggered_by_task: task_id.clone(),
    };
    
    // Persist to event log
    let mut event_log = state.event_log.lock().await;
    let event_id = match event_log.append(event.clone()) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "EVENT_LOG_ERROR",
                    "message": e.to_string()
                })),
            ).into_response();
        }
    };
    drop(event_log);
    
    // Emit to event bus
    state.events.emit_with_id(event_id, event);
    
    // Apply to shipment store
    let mut shipment_store = state.shipment_store.lock().await;
    match shipment_store.create(shipment.clone()) {
        Ok(_) => {
            // Auto-start transit
            let _ = shipment_store.start_transit(&shipment_id);
            
            // Update task with shipment_ref
            let mut task_store = state.task_store.lock().await;
            if let Some(task) = task_store.get_mut(&task_id) {
                task.props.insert(
                    "shipment_ref".to_string(),
                    serde_json::Value::String(shipment_id.as_str().to_string()),
                );
            }
            
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "result": "ShipmentCreated",
                    "event_id": event_id,
                    "shipment": shipment
                })),
            ).into_response()
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "SHIPMENT_CREATION_FAILED",
                    "message": e
                })),
            ).into_response()
        }
    }
}

/// PATCH /api/relay-physics/shipments/:id/arrive
/// Mark shipment as arrived (drone landed)
async fn arrive_shipment_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let shipment_id = ShipmentId::new(id);
    
    let actual_arrival = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Create event
    let event = RelayEvent::ShipmentArrived {
        shipment_id: shipment_id.clone(),
        actual_arrival,
    };
    
    // Persist to event log
    let mut event_log = state.event_log.lock().await;
    let event_id = match event_log.append(event.clone()) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "EVENT_LOG_ERROR",
                    "message": e.to_string()
                })),
            ).into_response();
        }
    };
    drop(event_log);
    
    // Emit to event bus
    state.events.emit_with_id(event_id, event);
    
    // Apply to shipment store
    let mut shipment_store = state.shipment_store.lock().await;
    match shipment_store.arrive(&shipment_id, actual_arrival) {
        Ok(_) => {
            let shipment = shipment_store.get(&shipment_id).cloned();
            
            // Complete linked task
            if let Some(ref ship) = shipment {
                let mut task_store = state.task_store.lock().await;
                let _ = task_store.complete(&ship.task_ref);
                
                // Emit TaskCompleted event
                let task_event = RelayEvent::TaskCompleted {
                    task_id: ship.task_ref.clone(),
                };
                let mut event_log = state.event_log.lock().await;
                if let Ok(eid) = event_log.append(task_event.clone()) {
                    state.events.emit_with_id(eid, task_event);
                }
            }
            
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "result": "ShipmentArrived",
                    "event_id": event_id,
                    "shipment": shipment
                })),
            ).into_response()
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "SHIPMENT_ARRIVAL_FAILED",
                    "message": e
                })),
            ).into_response()
        }
    }
}

// ============================================================================
// PR #9: IDENTITY ENDPOINTS (architecture@c10 - Users as filament trees)
// ============================================================================

/// GET /api/relay-physics/identities/:id
/// Get derived identity state
async fn get_identity_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;
    
    let identity_id = IdentityId::new(id);
    let identity_store = state.identity_store.lock().await;
    
    match identity_store.get(&identity_id) {
        Some(identity_state) => {
            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "private, max-age=10")],
                [(header::CONTENT_TYPE, "application/json")],
                Json(identity_state),
            ).into_response()
        }
        None => {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "IDENTITY_NOT_FOUND",
                    "message": format!("Identity {} not found", identity_id)
                })),
            ).into_response()
        }
    }
}

/// GET /api/relay-physics/identities/:id/commits
/// List all commits for an identity
async fn list_identity_commits_handler(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;
    
    let identity_id = IdentityId::new(id);
    
    // Open identity log
    let log = match IdentityLog::open(identity_id.clone(), "var/relay_physics/identities") {
        Ok(log) => log,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "LOG_OPEN_FAILED",
                    "message": e
                })),
            ).into_response();
        }
    };
    
    // Load all commits
    let commits = match log.load_all() {
        Ok(commits) => commits,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "LOG_LOAD_FAILED",
                    "message": e
                })),
            ).into_response();
        }
    };
    
    (
        StatusCode::OK,
        [(header::CACHE_CONTROL, "private, max-age=60")],
        [(header::CONTENT_TYPE, "application/json")],
        Json(commits),
    ).into_response()
}

/// POST /api/relay-physics/identities/:id/commits
/// Append a commit to an identity
async fn append_identity_commit_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<IdentityPayload>,
) -> impl IntoResponse {
    let identity_id = IdentityId::new(id);
    
    // Open identity log
    let mut log = match IdentityLog::open(identity_id.clone(), "var/relay_physics/identities") {
        Ok(log) => log,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "LOG_OPEN_FAILED",
                    "message": e
                })),
            ).into_response();
        }
    };
    
    // Append commit
    let commit_index = match log.append(payload.clone()) {
        Ok(idx) => idx,
        Err(e) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "COMMIT_APPEND_FAILED",
                    "message": e
                })),
            ).into_response();
        }
    };
    
    // Reload identity into store
    let commits = match log.load_all() {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "LOG_LOAD_FAILED",
                    "message": e
                })),
            ).into_response();
        }
    };
    
    let mut identity_store = state.identity_store.lock().await;
    identity_store.replay_identity(identity_id.clone(), &commits);
    
    // Emit SSE event
    let payload_type = match &payload {
        IdentityPayload::SelfClaim { .. } => "SELF_CLAIM",
        IdentityPayload::Attestation { .. } => "ATTESTATION",
        IdentityPayload::Binding { .. } => "BINDING",
        IdentityPayload::ScarApplied { .. } => "SCAR_APPLIED",
        IdentityPayload::AuthorityGranted { .. } => "AUTHORITY_GRANTED",
    }.to_string();
    
    let event = RelayEvent::IdentityChanged {
        identity_id: identity_id.clone(),
        commit_index,
        payload_type,
    };
    
    // Note: We don't persist identity events to main event log (they're in separate identity logs)
    // But we do emit them via SSE for real-time updates
    state.events.emit(event);
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "result": "CommitAppended",
            "commit_index": commit_index,
            "identity_id": identity_id.as_str()
        })),
    ).into_response()
}

/// POST /api/relay-physics/access/check
/// Check if viewer has access to target
#[derive(Debug, Deserialize)]
struct CheckAccessRequest {
    viewer_id: String,
    target_ref: String,
    target_kind: String,
    location_tile: Option<String>,
}

async fn check_access_handler(
    State(state): State<AppState>,
    Json(req): Json<CheckAccessRequest>,
) -> impl IntoResponse {
    // Parse target kind
    let target_kind = match req.target_kind.to_uppercase().as_str() {
        "ORG" => TargetKind::Org,
        "DEPT" => TargetKind::Dept,
        "BUILDING" => TargetKind::Building,
        "CHANNEL" => TargetKind::Channel,
        "FILAMENT" => TargetKind::Filament,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "INVALID_TARGET_KIND",
                    "message": format!("Invalid target kind: {}", req.target_kind)
                })),
            ).into_response();
        }
    };
    
    let viewer_id = IdentityId::new(req.viewer_id);
    let identity_store = state.identity_store.lock().await;
    
    let decision = check_access(
        &viewer_id,
        &req.target_ref,
        target_kind,
        req.location_tile.as_deref(),
        &identity_store,
    );
    
    (
        StatusCode::OK,
        Json(decision),
    ).into_response()
}

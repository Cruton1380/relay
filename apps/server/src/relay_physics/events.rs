// apps/server/src/relay_physics/events.rs
//
// PR #1.2: SSE with replay support + Last-Event-ID

/// PR #1.2 LOCK: Event bus capacity (Q1)
/// 
/// This defines the "max tolerated lag window" before forcing client reconnection.
/// 
/// Trade-offs:
/// - Larger buffer: slow clients stay connected longer (more memory, more lag tolerance)
/// - Smaller buffer: clients disconnect faster (less memory, stricter real-time guarantee)
/// 
/// Current policy: 1024 events (~1-2 seconds under high load, ~10-60 minutes under normal load)
/// 
/// Production tuning guide:
/// - Low load (< 1 event/sec): 256 events (~4 min lag tolerance)
/// - Medium load (1-10 events/sec): 1024 events (~2 min lag tolerance)  
/// - High load (10-100 events/sec): 4096 events (~40 sec lag tolerance)
/// - Burst load (100+ events/sec): 8192 events (~80 sec lag tolerance)
pub const EVENT_BUS_CAPACITY: usize = 1024;

use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;

use crate::relay_physics::{
    CommitEvent, CommitRejectedError, EventLog, EventLogEntry, Unit, Building, BuildingId,
    Task, TaskId, TaskState, Shipment, ShipmentId, GeoPosition,
    IdentityId, IdentityPayload,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RelayEvent {
    CommitAccepted { commit: CommitEvent },
    CommitRejected { error: CommitRejectedError },
    UnitStateChanged { unit: Unit },
    
    // PR #6: Building events (architecture@c9)
    BuildingRegistered { building: Building },
    BuildingUpdated { building_id: BuildingId, building: Building },
    
    // PR #7: Task events (architecture@c9 - Shopping as Production)
    TaskCreated { task: Task },
    TaskProgressed { task_id: TaskId, to_state: TaskState, progress_percentage: f32 },
    TaskCompleted { task_id: TaskId },
    TaskFailed { task_id: TaskId, reason: String },
    
    // PR #8: Shipment events (architecture@c9 - Drones in flight)
    ShipmentCreated { shipment: Shipment, triggered_by_task: TaskId },
    ShipmentPositionUpdated { shipment_id: ShipmentId, new_position: GeoPosition, progress_percentage: f32 },
    ShipmentArrived { shipment_id: ShipmentId, actual_arrival: u64 },
    ShipmentFailed { shipment_id: ShipmentId, reason: String },
    
    // PR #9: Identity events (architecture@c10 - Users as filament trees)
    IdentityChanged { identity_id: IdentityId, commit_index: u64, payload_type: String },
}

#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<(u64, RelayEvent)>, // (event_id, event)
}

impl EventBus {
    pub fn new(buffer: usize) -> Self {
        let (tx, _) = broadcast::channel(buffer);
        Self { tx }
    }

    /// Emit event with ID (called after persisting to log)
    pub fn emit_with_id(&self, event_id: u64, evt: RelayEvent) {
        let _ = self.tx.send((event_id, evt));
    }

    /// Legacy emit (for backward compat - assigns dummy ID)
    pub fn emit(&self, evt: RelayEvent) {
        let _ = self.tx.send((u64::MAX, evt));
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(u64, RelayEvent)> {
        self.tx.subscribe()
    }
}

/// SSE stream with replay support (PR #1.2)
/// 
/// Behavior:
/// - If `start_event_id` provided → replay from that ID, then switch to live
/// - Else → stream from "now" (live only)
pub fn sse_stream_with_replay(
    event_log: EventLog,
    bus: EventBus,
    start_event_id: Option<u64>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        let mut last_sent_id: Option<u64> = None;
        
        // Phase 1: Replay from log (if requested)
        if let Some(start_id) = start_event_id {
            match event_log.load_from(start_id) {
                Ok(entries) => {
                    for entry in entries {
                        let event_id = entry.event_id;
                        
                        // Send full envelope (auditable, replay-identical)
                        let json = serde_json::to_string(&entry)
                            .unwrap_or_else(|_| format!(r#"{{"error":"SERDE_ERROR","event_id":{}}}"#, event_id));
                        
                        yield Ok(Event::default()
                            .id(event_id.to_string())
                            .event("relay_event")
                            .data(json));
                        
                        last_sent_id = Some(event_id);
                    }
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to load events for replay: {}", e);
                }
            }
        }
        
        // Phase 2: Subscribe to live events
        let mut rx = bus.subscribe();
        
        loop {
            match rx.recv().await {
                Ok((event_id, evt)) => {
                    // Defensive: skip if we already sent this ID during replay
                    if let Some(last_id) = last_sent_id {
                        if event_id <= last_id {
                            continue; // Skip duplicates
                        }
                    }
                    
                    // Wrap in envelope format (consistent with replay)
                    let envelope = EventLogEntry::new(event_id, evt);
                    let json = serde_json::to_string(&envelope)
                        .unwrap_or_else(|_| format!(r#"{{"error":"SERDE_ERROR","event_id":{}}}"#, event_id));
                    
                    yield Ok(Event::default()
                        .id(event_id.to_string())
                        .event("relay_event")
                        .data(json));
                    
                    last_sent_id = Some(event_id);
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    eprintln!("🚨 SSE client lagged {} events - closing connection to force replay", n);
                    
                    // PR #1.2 LOCK: Lag breaks truth stream completeness
                    // Close connection → client reconnects with Last-Event-ID → replays from log
                    // This enforces: "truth stream must be complete"
                    
                    // Q3: Send error event before closing (BEST-EFFORT, NOT GUARANTEED)
                    // Due to async timing + TCP buffering, this event may not arrive.
                    // Recovery path is: client detects disconnect → EventSource auto-reconnects
                    //                  → Last-Event-ID header → replay from log ✅
                    yield Ok(Event::default()
                        .event("relay_error")
                        .data(format!(r#"{{"error":"STREAM_LAGGED","lagged":{},"message":"Connection closing. Reconnect to replay missed events."}}"#, n)));
                    
                    // Break loop → closes SSE connection
                    break;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"), // Heartbeat event name
    )
}

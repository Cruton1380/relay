// apps/server/src/relay_physics/commit_processor.rs
//
// SCV Commit-to-State Bridge
// This processes accepted commits and applies state transitions to UnitStore
//
// PR #1.1: Now also persists UnitStateChanged events to event log

use crate::relay_physics::{agent_ops, CommitEvent, EventBus, EventLog, RelayEvent, UnitStore, FilamentId};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CommitProcessor;

impl CommitProcessor {
    /// Process an accepted commit and apply state transitions
    /// 
    /// NOTE: This is called synchronously after commit is accepted
    /// Event persistence happens here (PR #1.1)
    pub fn process_commit(
        commit: &CommitEvent,
        unit_store: &mut UnitStore,
        event_bus: &EventBus,
        event_log: &Arc<Mutex<EventLog>>,
    ) {
        // Helper to emit and persist state change (PR #1.1 + #1.2)
        let emit_and_persist = |unit: crate::relay_physics::Unit| {
            let event = RelayEvent::UnitStateChanged { unit };
            
            // PR #1.2 LOCK: Persist MUST succeed before emit
            if let Ok(mut log) = event_log.try_lock() {
                match log.append(event.clone()) {
                    Ok(event_id) => {
                        // Only emit if persist succeeded
                        event_bus.emit_with_id(event_id, event);
                    }
                    Err(e) => {
                        eprintln!("🚨 WARNING: Failed to persist unit state change: {}", e);
                        // Don't emit if persist failed (maintains truth stream invariant)
                    }
                }
            } else {
                eprintln!("⚠️ WARNING: Event log locked, skipping state change persistence");
                // Don't emit if we can't persist (maintains truth stream invariant)
            }
        };
        
        match commit.op_type.as_str() {
            agent_ops::TASK_ASSIGN => {
                // Extract target_unit from payload
                if let Some(target_unit) = commit.payload.get("targetUnit").and_then(|v| v.as_str()) {
                    if let Some(filament_id) = commit.payload.get("filamentId").and_then(|v| v.as_str()) {
                        let unit_id = crate::relay_physics::UnitId::new(target_unit);
                        let work_filament = FilamentId::new(filament_id);
                        
                        if let Some(updated_unit) = unit_store.apply_task_assign(&unit_id, work_filament) {
                            emit_and_persist(updated_unit);
                        }
                    }
                }
            }
            
            agent_ops::UNIT_ATTACH => {
                if let Some(unit_id) = commit.payload.get("unitId").and_then(|v| v.as_str()) {
                    if let Some(filament_id) = commit.payload.get("filamentId").and_then(|v| v.as_str()) {
                        let uid = crate::relay_physics::UnitId::new(unit_id);
                        let fid = FilamentId::new(filament_id);
                        
                        if let Some(updated_unit) = unit_store.apply_attach(&uid, fid) {
                            emit_and_persist(updated_unit);
                        }
                    }
                }
            }
            
            agent_ops::UNIT_DETACH => {
                if let Some(unit_id) = commit.payload.get("unitId").and_then(|v| v.as_str()) {
                    let uid = crate::relay_physics::UnitId::new(unit_id);
                    
                    if let Some(updated_unit) = unit_store.apply_detach(&uid) {
                        emit_and_persist(updated_unit);
                    }
                }
            }
            
            agent_ops::OUTPUT_ACCEPTED => {
                // Extract unitId from proposalRef or payload
                if let Some(unit_id) = extract_unit_from_commit(commit) {
                    if let Some(updated_unit) = unit_store.apply_output_accepted(&unit_id) {
                        emit_and_persist(updated_unit);
                    }
                }
            }
            
            agent_ops::OUTPUT_REJECTED => {
                if let Some(unit_id) = extract_unit_from_commit(commit) {
                    if let Some(updated_unit) = unit_store.apply_output_rejected(&unit_id) {
                        emit_and_persist(updated_unit);
                    }
                }
            }
            
            _ => {
                // Unknown op_type or no state transition needed
            }
        }
    }
}

fn extract_unit_from_commit(commit: &CommitEvent) -> Option<crate::relay_physics::UnitId> {
    // MVP: try to extract from payload.unitId or from author_unit_ref
    if let Some(unit_id) = commit.payload.get("unitId").and_then(|v| v.as_str()) {
        return Some(crate::relay_physics::UnitId::new(unit_id));
    }
    
    // Fallback: use author as the unit (common for OUTPUT_ACCEPTED/REJECTED)
    Some(commit.author_unit_ref.clone())
}

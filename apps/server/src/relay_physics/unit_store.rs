// apps/server/src/relay_physics/unit_store.rs
use crate::relay_physics::{FilamentId, UnitId, UnitState, RelayEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub state: UnitState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_filament: Option<FilamentId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_task_filament: Option<FilamentId>,
}

#[derive(Debug)]
pub struct UnitStore {
    units: HashMap<UnitId, Unit>,
}

impl UnitStore {
    pub fn new() -> Self {
        Self { units: HashMap::new() }
    }

    pub fn create_unit(&mut self, id: UnitId) -> Unit {
        let unit = Unit {
            id: id.clone(),
            state: UnitState::Idle,
            attached_filament: None,
            current_task_filament: None,
        };
        self.units.insert(id.clone(), unit.clone());
        unit
    }

    pub fn list_units(&self) -> Vec<Unit> {
        self.units.values().cloned().collect()
    }

    pub fn get_unit(&self, id: &UnitId) -> Option<Unit> {
        self.units.get(id).cloned()
    }

    pub fn apply_task_assign(&mut self, unit_id: &UnitId, work_filament: FilamentId) -> Option<Unit> {
        let u = self.units.get_mut(unit_id)?;
        u.current_task_filament = Some(work_filament);
        // TASK_ASSIGN => Idle -> Moving (LOCKED)
        if u.state == UnitState::Idle {
            u.state = UnitState::Moving;
        }
        Some(u.clone())
    }

    pub fn apply_attach(&mut self, unit_id: &UnitId, filament: FilamentId) -> Option<Unit> {
        let u = self.units.get_mut(unit_id)?;
        u.attached_filament = Some(filament);
        // UNIT_ATTACH => Moving -> Working (LOCKED)
        if u.state == UnitState::Moving {
            u.state = UnitState::Working;
        }
        Some(u.clone())
    }

    pub fn apply_detach(&mut self, unit_id: &UnitId) -> Option<Unit> {
        let u = self.units.get_mut(unit_id)?;
        u.attached_filament = None;
        // UNIT_DETACH => Working -> Idle (LOCKED)
        if u.state == UnitState::Working {
            u.state = UnitState::Idle;
        }
        Some(u.clone())
    }

    pub fn apply_output_rejected(&mut self, unit_id: &UnitId) -> Option<Unit> {
        let u = self.units.get_mut(unit_id)?;
        // OUTPUT_REJECTED => Working -> Blocked (LOCKED)
        if u.state == UnitState::Working {
            u.state = UnitState::Blocked;
        }
        Some(u.clone())
    }

    pub fn apply_output_accepted(&mut self, unit_id: &UnitId) -> Option<Unit> {
        let u = self.units.get_mut(unit_id)?;
        // OUTPUT_ACCEPTED => remains Working (LOCKED)
        if u.state == UnitState::Working {
            // no-op
        }
        Some(u.clone())
    }

    pub fn apply_awaiting_authority(&mut self, unit_id: &UnitId) -> Option<Unit> {
        let u = self.units.get_mut(unit_id)?;
        u.state = UnitState::AwaitingAuthority;
        Some(u.clone())
    }
    
    /// Replay events to reconstruct state (PR #1.1)
    pub fn replay_from_events(&mut self, events: &[RelayEvent]) {
        for event in events {
            match event {
                RelayEvent::UnitStateChanged { unit } => {
                    // Directly restore unit state from event
                    self.units.insert(unit.id.clone(), unit.clone());
                }
                _ => {
                    // Ignore other event types (CommitAccepted, CommitRejected don't change unit state)
                }
            }
        }
    }
}

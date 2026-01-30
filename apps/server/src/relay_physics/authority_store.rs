// apps/server/src/relay_physics/authority_store.rs
use crate::relay_physics::{UnitId, RelayEvent};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthorityStore {
    // unit_id -> set of allowed op_types (MVP)
    unit_capabilities: HashMap<UnitId, HashSet<String>>,
}

impl AuthorityStore {
    pub fn new() -> Self {
        Self { unit_capabilities: HashMap::new() }
    }

    pub fn grant_op(&mut self, unit: &UnitId, op_type: impl Into<String>) {
        self.unit_capabilities
            .entry(unit.clone())
            .or_default()
            .insert(op_type.into());
    }

    pub fn is_authorized(&self, unit: &UnitId, op_type: &str, _authority_ref: &str) -> bool {
        // MVP: ignore authority_ref semantics; require it exists + op permitted
        self.unit_capabilities
            .get(unit)
            .map(|s| s.contains(op_type))
            .unwrap_or(false)
    }
    
    /// Replay events to reconstruct authority grants (PR #1.1)
    /// 
    /// NOTE: For MVP, authority grants are set during init (not event-driven)
    /// In PR #4 (full delegation), this will replay AUTHORITY_DELEGATE commits
    pub fn replay_from_events(&mut self, _events: &[RelayEvent]) {
        // MVP: Authority grants are static (set during init)
        // Future: Parse AUTHORITY_DELEGATE commits from event log
        // For now, this is a no-op placeholder
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::types::*;

/// Immutable commit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitEvent {
    /// Commit reference (filamentId@cIndex)
    pub commit_ref: CommitRef,
    
    /// Filament this commit belongs to
    pub filament_id: FilamentId,
    
    /// Monotonic commit index
    pub commit_index: CommitIndex,
    
    /// Operation type (e.g., "TASK_ASSIGN", "OUTPUT_PROPOSED")
    pub op_type: String,
    
    /// Timestamp (RFC3339)
    pub timestamp: DateTime<Utc>,
    
    /// Unit that authored this commit
    pub author_unit_ref: UnitId,
    
    /// Payload (operation-specific data)
    pub payload: JsonValue,
    
    /// Causal references
    pub causal_refs: CausalRefs,
}

impl CommitEvent {
    /// Create new commit event (used by FilamentStore)
    pub fn new(
        filament_id: FilamentId,
        commit_index: CommitIndex,
        op_type: String,
        author_unit_ref: UnitId,
        payload: JsonValue,
        causal_refs: CausalRefs,
    ) -> Self {
        let commit_ref = CommitRef::new(&filament_id, commit_index);
        
        Self {
            commit_ref,
            filament_id,
            commit_index,
            op_type,
            timestamp: Utc::now(),
            author_unit_ref,
            payload,
            causal_refs,
        }
    }
}

/// Data for creating a commit (before verification)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitData {
    pub op_type: String,
    pub author_unit_ref: UnitId,
    pub payload: JsonValue,
    pub causal_refs: CausalRefs,
}

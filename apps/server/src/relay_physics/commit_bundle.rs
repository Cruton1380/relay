// apps/server/src/relay_physics/commit_bundle.rs
//
// PR #1.3: Commit fetch - raw truth bundles (no rendering)

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
    
    /// The commit data (raw truth)
    pub commit: CommitEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Parsed commit reference format
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitRefFormat {
    EventId(u64),                     // event:42
    FilamentCommit(String, u64),      // work.W123@c7
}

impl CommitRefFormat {
    /// Parse commit ref from string
    /// 
    /// Supported formats:
    /// - `event:42` → EventId(42)
    /// - `work.W123@c7` → FilamentCommit("work.W123", 7)
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
        
        Err(format!(
            "Invalid commit ref format: expected 'event:<id>' or '<filamentId>@c<index>', got '{}'", 
            s
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_event_id() {
        assert_eq!(
            CommitRefFormat::parse("event:42").unwrap(),
            CommitRefFormat::EventId(42)
        );
    }

    #[test]
    fn test_parse_filament_commit() {
        assert_eq!(
            CommitRefFormat::parse("work.W123@c7").unwrap(),
            CommitRefFormat::FilamentCommit("work.W123".to_string(), 7)
        );
    }

    #[test]
    fn test_parse_invalid() {
        assert!(CommitRefFormat::parse("invalid-format").is_err());
        assert!(CommitRefFormat::parse("event:abc").is_err());
        assert!(CommitRefFormat::parse("work.W123@abc").is_err());
    }
}

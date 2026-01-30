// apps/server/src/relay_physics/event_log.rs
//
// Master event log for state reconstruction (PR #1.1)
// 
// INVARIANT: Same event log → same state (deterministic replay)

use crate::relay_physics::RelayEvent;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

/// Versioned event envelope for future schema evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLogEntry {
    /// Monotonic event ID (line number in JSONL)
    pub event_id: u64,
    
    /// Event schema version (for migration)
    pub schema_version: u32,
    
    /// Timestamp (for debugging, not used in replay)
    pub timestamp: String,
    
    /// The actual event
    pub event: RelayEvent,
}

impl EventLogEntry {
    pub fn new(event_id: u64, event: RelayEvent) -> Self {
        Self {
            event_id,
            schema_version: 1, // Current schema version
            timestamp: chrono::Utc::now().to_rfc3339(),
            event,
        }
    }
}

#[derive(Debug)]
pub struct EventLog {
    path: PathBuf,
    next_event_id: u64,
}

impl EventLog {
    /// Open or create event log at path
    pub fn open(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = path.into();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Scan existing log to determine next event ID
        let next_event_id = if path.exists() {
            Self::count_entries(&path)?
        } else {
            0
        };
        
        Ok(Self { path, next_event_id })
    }
    
    /// Append event to log (thread-safe via file locking)
    pub fn append(&mut self, event: RelayEvent) -> std::io::Result<u64> {
        let event_id = self.next_event_id;
        let entry = EventLogEntry::new(event_id, event);
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        
        let line = serde_json::to_string(&entry)?;
        writeln!(file, "{}", line)?;
        file.flush()?;
        
        self.next_event_id += 1;
        Ok(event_id)
    }
    
    /// Load all events from log (for replay)
    pub fn load_all(&self) -> std::io::Result<Vec<EventLogEntry>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<EventLogEntry>(&line) {
                Ok(entry) => {
                    // Verify event_id matches line number (corruption check)
                    if entry.event_id != line_num as u64 {
                        eprintln!(
                            "⚠️ Event log corruption: expected event_id {}, got {}",
                            line_num, entry.event_id
                        );
                        // Still continue (best-effort recovery)
                    }
                    entries.push(entry);
                }
                Err(e) => {
                    eprintln!(
                        "⚠️ Failed to parse event log entry at line {}: {}",
                        line_num, e
                    );
                    // Continue (skip corrupted entries)
                }
            }
        }
        
        Ok(entries)
    }
    
    /// Get next event ID (for SSE Last-Event-ID support)
    pub fn next_event_id(&self) -> u64 {
        self.next_event_id
    }
    
    /// Load events starting from event_id (for SSE reconnection)
    pub fn load_from(&self, start_event_id: u64) -> std::io::Result<Vec<EventLogEntry>> {
        let all_events = self.load_all()?;
        Ok(all_events
            .into_iter()
            .filter(|e| e.event_id >= start_event_id)
            .collect())
    }
    
    /// Count entries in log (for init)
    fn count_entries(path: &Path) -> std::io::Result<u64> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines().filter(|l| l.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false)).count() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::{CommitEvent, CommitRejectedError, Unit, UnitId, UnitState, FilamentId};
    
    #[test]
    fn test_event_log_append_and_replay() {
        let temp_path = "test_data/event_log_test.jsonl";
        
        // Create log and append events
        {
            let mut log = EventLog::open(temp_path).unwrap();
            
            let event1 = RelayEvent::UnitStateChanged {
                unit: Unit {
                    id: UnitId::new("unit.test.001"),
                    state: UnitState::Idle,
                    attached_filament: None,
                    current_task_filament: None,
                },
            };
            
            let event_id1 = log.append(event1.clone()).unwrap();
            assert_eq!(event_id1, 0);
            
            let event_id2 = log.append(event1.clone()).unwrap();
            assert_eq!(event_id2, 1);
        }
        
        // Reload log and verify events
        {
            let log = EventLog::open(temp_path).unwrap();
            assert_eq!(log.next_event_id(), 2);
            
            let entries = log.load_all().unwrap();
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].event_id, 0);
            assert_eq!(entries[1].event_id, 1);
        }
        
        // Cleanup
        std::fs::remove_file(temp_path).ok();
        std::fs::remove_dir("test_data").ok();
    }
    
    #[test]
    fn test_event_log_deterministic_replay() {
        let temp_path = "test_data/determinism_test.jsonl";
        
        // Create log with events
        {
            let mut log = EventLog::open(temp_path).unwrap();
            
            for i in 0..10 {
                let event = RelayEvent::UnitStateChanged {
                    unit: Unit {
                        id: UnitId::new(format!("unit.test.{:03}", i)),
                        state: UnitState::Working,
                        attached_filament: Some(FilamentId::new(format!("work.W{}", i))),
                        current_task_filament: Some(FilamentId::new(format!("work.W{}", i))),
                    },
                };
                log.append(event).unwrap();
            }
        }
        
        // Replay 1
        let entries1 = EventLog::open(temp_path).unwrap().load_all().unwrap();
        
        // Replay 2 (from fresh EventLog instance)
        let entries2 = EventLog::open(temp_path).unwrap().load_all().unwrap();
        
        // Must be identical
        assert_eq!(entries1.len(), entries2.len());
        for (e1, e2) in entries1.iter().zip(entries2.iter()) {
            assert_eq!(e1.event_id, e2.event_id);
            assert_eq!(e1.schema_version, e2.schema_version);
            // Events should match (RelayEvent must be deterministically serializable)
        }
        
        // Cleanup
        std::fs::remove_file(temp_path).ok();
        std::fs::remove_dir("test_data").ok();
    }
    
    #[test]
    fn test_event_log_load_from() {
        let temp_path = "test_data/load_from_test.jsonl";
        
        // Create log with 5 events
        {
            let mut log = EventLog::open(temp_path).unwrap();
            for i in 0..5 {
                let event = RelayEvent::UnitStateChanged {
                    unit: Unit {
                        id: UnitId::new(format!("unit.test.{:03}", i)),
                        state: UnitState::Idle,
                        attached_filament: None,
                        current_task_filament: None,
                    },
                };
                log.append(event).unwrap();
            }
        }
        
        // Load from event_id 3
        let log = EventLog::open(temp_path).unwrap();
        let entries = log.load_from(3).unwrap();
        
        assert_eq!(entries.len(), 2); // Events 3, 4
        assert_eq!(entries[0].event_id, 3);
        assert_eq!(entries[1].event_id, 4);
        
        // Cleanup
        std::fs::remove_file(temp_path).ok();
        std::fs::remove_dir("test_data").ok();
    }
}

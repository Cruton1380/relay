// apps/server/src/relay_physics/identity_log.rs
// PR #9: Identity commit log (JSONL, append-only per identity)
//
// Each identity has its own filament file: var/relay_physics/identities/<id>.jsonl
// No deletes, no rewrites, deterministic replay

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::relay_physics::{IdentityId, IdentityCommit, IdentityPayload};

/// Identity log (per-identity JSONL file)
pub struct IdentityLog {
    identity_id: IdentityId,
    file_path: PathBuf,
    next_commit_index: u64,
}

impl IdentityLog {
    /// Open or create identity log for given identity
    pub fn open(identity_id: IdentityId, base_dir: impl AsRef<Path>) -> Result<Self, String> {
        let base_path = base_dir.as_ref();
        
        // Create identities directory if it doesn't exist
        fs::create_dir_all(base_path)
            .map_err(|e| format!("Failed to create identities dir: {}", e))?;
        
        // Sanitize identity ID for filename (replace slashes, etc.)
        let safe_filename = identity_id.as_str().replace(['/', '\\', ':'], "_");
        let file_path = base_path.join(format!("{}.jsonl", safe_filename));
        
        // If file exists, count lines to get next commit index
        let next_commit_index = if file_path.exists() {
            let file = File::open(&file_path)
                .map_err(|e| format!("Failed to open identity log: {}", e))?;
            let reader = BufReader::new(file);
            reader.lines().count() as u64
        } else {
            0
        };
        
        Ok(Self {
            identity_id,
            file_path,
            next_commit_index,
        })
    }
    
    /// Append a commit to the log
    pub fn append(&mut self, payload: IdentityPayload) -> Result<u64, String> {
        let commit_index = self.next_commit_index;
        
        let commit = IdentityCommit {
            schema_version: 1,
            identity_id: self.identity_id.clone(),
            commit_index,
            timestamp: chrono::Utc::now().to_rfc3339(),
            payload,
            authority_ref: None,
        };
        
        // Serialize to JSON
        let json_line = serde_json::to_string(&commit)
            .map_err(|e| format!("Failed to serialize commit: {}", e))?;
        
        // Append to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| format!("Failed to open identity log for append: {}", e))?;
        
        writeln!(file, "{}", json_line)
            .map_err(|e| format!("Failed to write commit: {}", e))?;
        
        file.sync_all()
            .map_err(|e| format!("Failed to sync identity log: {}", e))?;
        
        self.next_commit_index += 1;
        
        Ok(commit_index)
    }
    
    /// Load all commits from the log
    pub fn load_all(&self) -> Result<Vec<IdentityCommit>, String> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }
        
        let file = File::open(&self.file_path)
            .map_err(|e| format!("Failed to open identity log: {}", e))?;
        
        let reader = BufReader::new(file);
        let mut commits = Vec::new();
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| format!("Failed to read line {}: {}", line_num, e))?;
            
            let commit: IdentityCommit = serde_json::from_str(&line)
                .map_err(|e| format!("Failed to parse commit at line {}: {}", line_num, e))?;
            
            commits.push(commit);
        }
        
        Ok(commits)
    }
    
    /// Get the next commit index (current count)
    pub fn next_commit_index(&self) -> u64 {
        self.next_commit_index
    }
    
    /// Check if log exists and has commits
    pub fn exists(&self) -> bool {
        self.file_path.exists() && self.next_commit_index > 0
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::BindingType;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_and_append() {
        let temp_dir = TempDir::new().unwrap();
        let identity_id = IdentityId::new("id.human.test");
        
        let mut log = IdentityLog::open(identity_id.clone(), temp_dir.path()).unwrap();
        
        assert_eq!(log.next_commit_index(), 0);
        
        let payload = IdentityPayload::SelfClaim {
            claim_key: "name".to_string(),
            claim_value: "Alice".to_string(),
            scope: "global".to_string(),
        };
        
        let commit_index = log.append(payload).unwrap();
        assert_eq!(commit_index, 0);
        assert_eq!(log.next_commit_index(), 1);
    }
    
    #[test]
    fn test_load_all() {
        let temp_dir = TempDir::new().unwrap();
        let identity_id = IdentityId::new("id.human.test");
        
        let mut log = IdentityLog::open(identity_id.clone(), temp_dir.path()).unwrap();
        
        // Append 3 commits
        log.append(IdentityPayload::SelfClaim {
            claim_key: "name".to_string(),
            claim_value: "Alice".to_string(),
            scope: "global".to_string(),
        }).unwrap();
        
        log.append(IdentityPayload::Binding {
            binding_type: BindingType::OrgMember,
            target_ref: "org.acme".to_string(),
            scope: "global".to_string(),
            expires_event_id: None,
        }).unwrap();
        
        log.append(IdentityPayload::SelfClaim {
            claim_key: "role".to_string(),
            claim_value: "Engineer".to_string(),
            scope: "org.acme".to_string(),
        }).unwrap();
        
        // Load all
        let commits = log.load_all().unwrap();
        assert_eq!(commits.len(), 3);
        assert_eq!(commits[0].commit_index, 0);
        assert_eq!(commits[1].commit_index, 1);
        assert_eq!(commits[2].commit_index, 2);
    }
    
    #[test]
    fn test_persistence_across_open() {
        let temp_dir = TempDir::new().unwrap();
        let identity_id = IdentityId::new("id.human.test");
        
        // First open: append commit
        {
            let mut log = IdentityLog::open(identity_id.clone(), temp_dir.path()).unwrap();
            log.append(IdentityPayload::SelfClaim {
                claim_key: "name".to_string(),
                claim_value: "Alice".to_string(),
                scope: "global".to_string(),
            }).unwrap();
        }
        
        // Second open: should resume at index 1
        {
            let log = IdentityLog::open(identity_id.clone(), temp_dir.path()).unwrap();
            assert_eq!(log.next_commit_index(), 1);
            
            let commits = log.load_all().unwrap();
            assert_eq!(commits.len(), 1);
        }
    }
    
    #[test]
    fn test_monotonic_commit_index() {
        let temp_dir = TempDir::new().unwrap();
        let identity_id = IdentityId::new("id.human.test");
        
        let mut log = IdentityLog::open(identity_id.clone(), temp_dir.path()).unwrap();
        
        let idx0 = log.append(IdentityPayload::SelfClaim {
            claim_key: "a".to_string(),
            claim_value: "1".to_string(),
            scope: "global".to_string(),
        }).unwrap();
        
        let idx1 = log.append(IdentityPayload::SelfClaim {
            claim_key: "b".to_string(),
            claim_value: "2".to_string(),
            scope: "global".to_string(),
        }).unwrap();
        
        let idx2 = log.append(IdentityPayload::SelfClaim {
            claim_key: "c".to_string(),
            claim_value: "3".to_string(),
            scope: "global".to_string(),
        }).unwrap();
        
        assert_eq!(idx0, 0);
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
    }
}

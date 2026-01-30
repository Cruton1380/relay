// apps/server/src/relay_physics/filament_store.rs
use crate::relay_physics::{
    CommitData, CommitEvent, CommitIndex, CommitRef, FilamentId, Verifier, AuthorityStore,
    CommitRejectedError,
};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct FilamentStore {
    base_dir: PathBuf,
    // Cache: head index per filament (derived deterministically by reading file on first access)
    head_index_cache: HashMap<FilamentId, CommitIndex>,
}

impl FilamentStore {
    pub fn new(base_dir: impl Into<PathBuf>) -> std::io::Result<Self> {
        let base_dir = base_dir.into();
        fs::create_dir_all(&base_dir)?;
        Ok(Self {
            base_dir,
            head_index_cache: HashMap::new(),
        })
    }

    fn filament_path(&self, filament_id: &FilamentId) -> PathBuf {
        // Safe-ish filename mapping. You may want stricter sanitization later.
        // For MVP: replace '/' and '\' so it can't escape base_dir.
        let name = filament_id
            .as_str()
            .replace('/', "_")
            .replace('\\', "_")
            .replace("..", "_");
        self.base_dir.join(format!("{name}.jsonl"))
    }

    fn ensure_head_loaded(&mut self, filament_id: &FilamentId) -> std::io::Result<()> {
        if self.head_index_cache.contains_key(filament_id) {
            return Ok(());
        }
        let path = self.filament_path(filament_id);
        if !path.exists() {
            self.head_index_cache.insert(filament_id.clone(), 0);
            return Ok(());
        }

        // Read last commit_index deterministically (scan file once).
        // MVP approach: read all lines; upgrade later to tail-read.
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut head: CommitIndex = 0;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let evt: CommitEvent = serde_json::from_str(&line)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            head = head.max(evt.commit_index);
        }

        self.head_index_cache.insert(filament_id.clone(), head);
        Ok(())
    }

    pub fn get_head_index(&mut self, filament_id: &FilamentId) -> std::io::Result<CommitIndex> {
        self.ensure_head_loaded(filament_id)?;
        Ok(*self.head_index_cache.get(filament_id).unwrap_or(&0))
    }

    pub fn list_commits(&self, filament_id: &FilamentId) -> std::io::Result<Vec<CommitEvent>> {
        let path = self.filament_path(filament_id);
        if !path.exists() {
            return Ok(vec![]);
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut out = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let evt: CommitEvent = serde_json::from_str(&line)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            out.push(evt);
        }
        Ok(out)
    }

    pub fn commit_ref_exists(&self, commit_ref: &CommitRef) -> bool {
        // MVP: existence check by parsing "<filament>@c<index>" and verifying index <= head.
        // This is deterministic, and avoids scanning all files.
        // If parsing fails, treat as missing.
        let s = commit_ref.as_str();
        let Some((fid, idx)) = parse_commit_ref(s) else {
            return false;
        };

        let filament_id = FilamentId::new(fid);
        let path = self.filament_path(&filament_id);
        if !path.exists() {
            return false;
        }

        // Deterministic but slightly expensive: scan to determine head.
        // OK for MVP. Later: store head index in a small sidecar.
        match self.list_commits(&filament_id) {
            Ok(commits) => commits.iter().any(|c| c.commit_index == idx),
            Err(_) => false,
        }
    }

    /// Append commit: VERIFY BEFORE APPEND (LOCKED INVARIANT)
    pub fn append_commit(
        &mut self,
        filament_id: FilamentId,
        commit_data: CommitData,
        verifier: &Verifier,
        authority_store: &AuthorityStore,
    ) -> Result<CommitEvent, CommitRejectedError> {
        // Pre-append verification
        verifier.verify(&filament_id, &commit_data, self, authority_store)?;

        // Deterministic commitIndex allocation per filament
        self.ensure_head_loaded(&filament_id)
            .map_err(|e| CommitRejectedError::new(crate::relay_physics::ReasonCode::CustomRuleFailed, e.to_string()))?;

        let next_index = self
            .head_index_cache
            .get(&filament_id)
            .copied()
            .unwrap_or(0)
            + 1;

        let evt = CommitEvent::new(
            filament_id.clone(),
            next_index,
            commit_data.op_type,
            commit_data.author_unit_ref,
            commit_data.payload,
            commit_data.causal_refs,
        );

        // Append JSONL
        let path = self.filament_path(&filament_id);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                CommitRejectedError::new(crate::relay_physics::ReasonCode::CustomRuleFailed, e.to_string())
            })?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| CommitRejectedError::new(crate::relay_physics::ReasonCode::CustomRuleFailed, e.to_string()))?;

        let line = serde_json::to_string(&evt).map_err(|e| {
            CommitRejectedError::new(crate::relay_physics::ReasonCode::CustomRuleFailed, e.to_string())
        })?;

        writeln!(file, "{line}").map_err(|e| {
            CommitRejectedError::new(crate::relay_physics::ReasonCode::CustomRuleFailed, e.to_string())
        })?;
        file.flush().ok(); // best effort for MVP

        // Update cache
        self.head_index_cache.insert(filament_id, next_index);

        Ok(evt)
    }
}

fn parse_commit_ref(s: &str) -> Option<(String, CommitIndex)> {
    // "<filamentId>@c<index>"
    let parts: Vec<&str> = s.split("@c").collect();
    if parts.len() != 2 {
        return None;
    }
    let fid = parts[0].to_string();
    let idx: CommitIndex = parts[1].parse().ok()?;
    Some((fid, idx))
}

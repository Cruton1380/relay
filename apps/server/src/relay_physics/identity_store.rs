// apps/server/src/relay_physics/identity_store.rs
// PR #9: Identity Store (in-memory state + deterministic replay)
//
// Maintains derived IdentityState for all known identities
// Replay from commits is deterministic and idempotent

use std::collections::HashMap;
use crate::relay_physics::{
    IdentityId, IdentityCommit, IdentityPayload, IdentityState,
    Attestation, Binding, Scar, AuthorityGrant,
    AttestationType, BindingType, ScarType,
};

/// Identity store (in-memory cache of derived states)
pub struct IdentityStore {
    states: HashMap<IdentityId, IdentityState>,
}

impl IdentityStore {
    /// Create a new empty identity store
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }
    
    /// Get identity state (if known)
    pub fn get(&self, identity_id: &IdentityId) -> Option<&IdentityState> {
        self.states.get(identity_id)
    }
    
    /// Get mutable identity state (if known)
    pub fn get_mut(&mut self, identity_id: &IdentityId) -> Option<&mut IdentityState> {
        self.states.get_mut(identity_id)
    }
    
    /// Ensure identity exists (create if needed)
    pub fn ensure(&mut self, identity_id: IdentityId) -> &mut IdentityState {
        self.states.entry(identity_id.clone())
            .or_insert_with(|| IdentityState::new(identity_id))
    }
    
    /// Apply a single commit to identity state
    pub fn apply_commit(&mut self, commit: &IdentityCommit) {
        let state = self.ensure(commit.identity_id.clone());
        state.commit_count = commit.commit_index + 1;
        
        match &commit.payload {
            IdentityPayload::SelfClaim { claim_key, claim_value, .. } => {
                state.claims.insert(claim_key.clone(), claim_value.clone());
            }
            
            IdentityPayload::Attestation {
                attester_id,
                attestation_type,
                target_ref,
                scope,
                expires_event_id,
            } => {
                state.attestations.push(Attestation {
                    commit_index: commit.commit_index,
                    attester_id: attester_id.clone(),
                    attestation_type: *attestation_type,
                    target_ref: target_ref.clone(),
                    scope: scope.clone(),
                    expires_event_id: *expires_event_id,
                });
            }
            
            IdentityPayload::Binding {
                binding_type,
                target_ref,
                scope,
                expires_event_id,
            } => {
                state.bindings.push(Binding {
                    commit_index: commit.commit_index,
                    binding_type: *binding_type,
                    target_ref: target_ref.clone(),
                    scope: scope.clone(),
                    expires_event_id: *expires_event_id,
                });
            }
            
            IdentityPayload::ScarApplied {
                scar_type,
                scope,
                reason,
                severity,
            } => {
                state.scars.push(Scar {
                    commit_index: commit.commit_index,
                    scar_type: *scar_type,
                    scope: scope.clone(),
                    reason: reason.clone(),
                    severity: *severity,
                });
            }
            
            IdentityPayload::AuthorityGranted {
                grant_type,
                target_ref,
                scope,
                duration_events,
                amount,
            } => {
                state.authority_grants.push(AuthorityGrant {
                    commit_index: commit.commit_index,
                    grant_type: grant_type.clone(),
                    target_ref: target_ref.clone(),
                    scope: scope.clone(),
                    duration_events: *duration_events,
                    amount: *amount,
                });
            }
        }
    }
    
    /// Replay a sequence of commits (deterministic)
    pub fn replay(&mut self, commits: &[IdentityCommit]) {
        for commit in commits {
            self.apply_commit(commit);
        }
    }
    
    /// Replay commits for a specific identity (creates state if needed)
    pub fn replay_identity(&mut self, identity_id: IdentityId, commits: &[IdentityCommit]) -> &IdentityState {
        // Clear existing state for this identity
        self.states.remove(&identity_id);
        
        // Replay all commits
        self.replay(commits);
        
        // Return the derived state
        self.get(&identity_id).expect("State should exist after replay")
    }
    
    /// List all known identities
    pub fn list_all(&self) -> Vec<IdentityId> {
        self.states.keys().cloned().collect()
    }
    
    /// Count known identities
    pub fn count(&self) -> usize {
        self.states.len()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_commit(
        identity_id: IdentityId,
        commit_index: u64,
        payload: IdentityPayload,
    ) -> IdentityCommit {
        IdentityCommit {
            schema_version: 1,
            identity_id,
            commit_index,
            timestamp: "2026-01-29T00:00:00Z".to_string(),
            payload,
            authority_ref: None,
        }
    }
    
    #[test]
    fn test_apply_self_claim() {
        let mut store = IdentityStore::new();
        let identity_id = IdentityId::new("id.human.alice");
        
        let commit = create_test_commit(
            identity_id.clone(),
            0,
            IdentityPayload::SelfClaim {
                claim_key: "name".to_string(),
                claim_value: "Alice".to_string(),
                scope: "global".to_string(),
            },
        );
        
        store.apply_commit(&commit);
        
        let state = store.get(&identity_id).unwrap();
        assert_eq!(state.commit_count, 1);
        assert_eq!(state.claims.get("name").unwrap(), "Alice");
    }
    
    #[test]
    fn test_apply_binding() {
        let mut store = IdentityStore::new();
        let identity_id = IdentityId::new("id.human.alice");
        
        let commit = create_test_commit(
            identity_id.clone(),
            0,
            IdentityPayload::Binding {
                binding_type: BindingType::OrgMember,
                target_ref: "org.acme".to_string(),
                scope: "global".to_string(),
                expires_event_id: None,
            },
        );
        
        store.apply_commit(&commit);
        
        let state = store.get(&identity_id).unwrap();
        assert_eq!(state.bindings.len(), 1);
        assert_eq!(state.bindings[0].target_ref, "org.acme");
        assert_eq!(state.bindings[0].binding_type, BindingType::OrgMember);
    }
    
    #[test]
    fn test_apply_scar() {
        let mut store = IdentityStore::new();
        let identity_id = IdentityId::new("id.human.alice");
        
        let commit = create_test_commit(
            identity_id.clone(),
            0,
            IdentityPayload::ScarApplied {
                scar_type: ScarType::AccessRestrict,
                scope: "dept.rd".to_string(),
                reason: "REQUIRES_ESCORT".to_string(),
                severity: 2,
            },
        );
        
        store.apply_commit(&commit);
        
        let state = store.get(&identity_id).unwrap();
        assert_eq!(state.scars.len(), 1);
        assert_eq!(state.scars[0].scope, "dept.rd");
        assert_eq!(state.scars[0].severity, 2);
    }
    
    #[test]
    fn test_deterministic_replay() {
        let identity_id = IdentityId::new("id.human.alice");
        
        let commits = vec![
            create_test_commit(
                identity_id.clone(),
                0,
                IdentityPayload::SelfClaim {
                    claim_key: "name".to_string(),
                    claim_value: "Alice".to_string(),
                    scope: "global".to_string(),
                },
            ),
            create_test_commit(
                identity_id.clone(),
                1,
                IdentityPayload::Binding {
                    binding_type: BindingType::OrgMember,
                    target_ref: "org.acme".to_string(),
                    scope: "global".to_string(),
                    expires_event_id: None,
                },
            ),
            create_test_commit(
                identity_id.clone(),
                2,
                IdentityPayload::SelfClaim {
                    claim_key: "role".to_string(),
                    claim_value: "Engineer".to_string(),
                    scope: "org.acme".to_string(),
                },
            ),
        ];
        
        // Replay twice
        let mut store1 = IdentityStore::new();
        store1.replay(&commits);
        
        let mut store2 = IdentityStore::new();
        store2.replay(&commits);
        
        // States should be identical
        let state1 = store1.get(&identity_id).unwrap();
        let state2 = store2.get(&identity_id).unwrap();
        
        assert_eq!(state1.commit_count, state2.commit_count);
        assert_eq!(state1.claims.len(), state2.claims.len());
        assert_eq!(state1.bindings.len(), state2.bindings.len());
        assert_eq!(state1.claims.get("name"), state2.claims.get("name"));
        assert_eq!(state1.claims.get("role"), state2.claims.get("role"));
    }
    
    #[test]
    fn test_replay_identity() {
        let identity_id = IdentityId::new("id.human.alice");
        
        let commits = vec![
            create_test_commit(
                identity_id.clone(),
                0,
                IdentityPayload::Binding {
                    binding_type: BindingType::DeptMember,
                    target_ref: "dept.finance".to_string(),
                    scope: "org.acme".to_string(),
                    expires_event_id: None,
                },
            ),
        ];
        
        let mut store = IdentityStore::new();
        let state = store.replay_identity(identity_id.clone(), &commits);
        
        assert_eq!(state.commit_count, 1);
        assert_eq!(state.bindings.len(), 1);
        assert_eq!(state.bindings[0].target_ref, "dept.finance");
    }
}

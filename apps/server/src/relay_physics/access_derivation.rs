// apps/server/src/relay_physics/access_derivation.rs
// PR #9: Access derivation engine (explainable allow/deny)
//
// Access is derived from identity intersections, not permissions
// Every decision includes an explanation path (commit refs that granted/blocked)

use crate::relay_physics::{
    IdentityId, IdentityStore, TargetKind, AccessDecision, AccessPathStep,
    BindingType, ScarType,
};

/// Check if viewer has access to target
/// 
/// Returns AccessDecision with:
/// - allow: true/false
/// - reason: human-readable explanation
/// - path: commits that granted access (if allow)
/// - blocked_by: commit that blocked access (if deny)
pub fn check_access(
    viewer_id: &IdentityId,
    target_ref: &str,
    target_kind: TargetKind,
    _location_tile: Option<&str>, // Future use (PR #12)
    identity_store: &IdentityStore,
) -> AccessDecision {
    // Get viewer's identity state
    let viewer_state = match identity_store.get(viewer_id) {
        Some(state) => state,
        None => return AccessDecision::deny(
            format!("Identity {} not found", viewer_id),
            None,
        ),
    };
    
    // Step 1: Check for blocking scars
    for scar in &viewer_state.scars {
        if scar_blocks_access(&scar.scope, target_ref, target_kind) {
            let commit_ref = format!("{}@c{}", viewer_id, scar.commit_index);
            return AccessDecision::deny(
                format!("Access blocked by scar: {} ({})", scar.scar_type, scar.reason),
                Some(commit_ref),
            );
        }
    }
    
    // Step 2: Check for matching bindings
    let mut path = Vec::new();
    
    for binding in &viewer_state.bindings {
        if binding_grants_access(&binding.target_ref, target_ref, target_kind) {
            let commit_ref = format!("{}@c{}", viewer_id, binding.commit_index);
            let effect = format!(
                "{}:{}->{}",
                binding.binding_type,
                binding.binding_type,
                binding.target_ref
            );
            
            path.push(AccessPathStep {
                commit_ref,
                effect,
            });
            
            return AccessDecision::allow(
                format!("Access granted via {} binding to {}", binding.binding_type, binding.target_ref),
                path,
            );
        }
    }
    
    // Step 3: Check for matching attestations
    for attestation in &viewer_state.attestations {
        if attestation_grants_access(&attestation.target_ref, target_ref, target_kind) {
            let commit_ref = format!("{}@c{}", viewer_id, attestation.commit_index);
            let effect = format!(
                "ATTESTATION:{}->{}",
                attestation.attestation_type,
                attestation.target_ref
            );
            
            path.push(AccessPathStep {
                commit_ref,
                effect,
            });
            
            return AccessDecision::allow(
                format!("Access granted via {} attestation to {}", attestation.attestation_type, attestation.target_ref),
                path,
            );
        }
    }
    
    // No match found
    AccessDecision::deny(
        format!("No binding or attestation grants access to {}", target_ref),
        None,
    )
}

/// Check if scar blocks access to target
fn scar_blocks_access(scar_scope: &str, target_ref: &str, target_kind: TargetKind) -> bool {
    // Exact match
    if scar_scope == target_ref {
        return true;
    }
    
    // Parent scope match (e.g., scar on "dept.rd" blocks "dept.rd.lab1")
    if target_ref.starts_with(&format!("{}.", scar_scope)) {
        return true;
    }
    
    // Kind-specific blocking
    match target_kind {
        TargetKind::Dept | TargetKind::Org => {
            // Scar on parent org blocks dept
            if let Some(org_prefix) = scar_scope.strip_prefix("org.") {
                if target_ref.starts_with(&format!("dept.{}", org_prefix)) {
                    return true;
                }
            }
        }
        _ => {}
    }
    
    false
}

/// Check if binding grants access to target
fn binding_grants_access(binding_target: &str, target_ref: &str, target_kind: TargetKind) -> bool {
    // Exact match
    if binding_target == target_ref {
        return true;
    }
    
    // Parent scope match (e.g., org binding grants dept access)
    match target_kind {
        TargetKind::Dept => {
            // Binding to org grants dept access
            if let Some(dept_org) = extract_org_from_dept(target_ref) {
                if binding_target == dept_org {
                    return true;
                }
            }
        }
        TargetKind::Building => {
            // Binding to org/dept might grant building access (org-owned buildings)
            // For now, require exact match or parent scope
            if target_ref.starts_with(&format!("{}.", binding_target)) {
                return true;
            }
        }
        _ => {}
    }
    
    false
}

/// Check if attestation grants access to target
fn attestation_grants_access(attestation_target: &str, target_ref: &str, _target_kind: TargetKind) -> bool {
    // Exact match
    if attestation_target == target_ref {
        return true;
    }
    
    // For now, attestations are scope-specific (no parent inheritance)
    false
}

/// Extract org from dept ref (e.g., "dept.finance" -> "org.acme" if known)
/// For v1, just return None (requires org registry)
fn extract_org_from_dept(_dept_ref: &str) -> Option<String> {
    // TODO: Implement org lookup
    // For now, assume dept refs include org (e.g., "dept.acme.finance")
    None
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::{
        IdentityCommit, IdentityPayload, BindingType, ScarType,
        AttestationType,
    };
    
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
    fn test_access_granted_via_binding() {
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
        store.replay(&commits);
        
        let decision = check_access(
            &identity_id,
            "dept.finance",
            TargetKind::Dept,
            None,
            &store,
        );
        
        assert!(decision.allow);
        assert_eq!(decision.path.len(), 1);
        assert!(decision.path[0].commit_ref.contains("@c0"));
    }
    
    #[test]
    fn test_access_denied_no_binding() {
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
        store.replay(&commits);
        
        let decision = check_access(
            &identity_id,
            "dept.rd",
            TargetKind::Dept,
            None,
            &store,
        );
        
        assert!(!decision.allow);
        assert_eq!(decision.path.len(), 0);
        assert!(decision.blocked_by.is_none());
    }
    
    #[test]
    fn test_access_blocked_by_scar() {
        let identity_id = IdentityId::new("id.human.alice");
        
        let commits = vec![
            create_test_commit(
                identity_id.clone(),
                0,
                IdentityPayload::Binding {
                    binding_type: BindingType::DeptMember,
                    target_ref: "dept.rd".to_string(),
                    scope: "org.acme".to_string(),
                    expires_event_id: None,
                },
            ),
            create_test_commit(
                identity_id.clone(),
                1,
                IdentityPayload::ScarApplied {
                    scar_type: ScarType::AccessRestrict,
                    scope: "dept.rd".to_string(),
                    reason: "REQUIRES_ESCORT".to_string(),
                    severity: 2,
                },
            ),
        ];
        
        let mut store = IdentityStore::new();
        store.replay(&commits);
        
        let decision = check_access(
            &identity_id,
            "dept.rd",
            TargetKind::Dept,
            None,
            &store,
        );
        
        assert!(!decision.allow);
        assert!(decision.blocked_by.is_some());
        assert!(decision.blocked_by.unwrap().contains("@c1"));
    }
    
    #[test]
    fn test_explain_allow_path() {
        let identity_id = IdentityId::new("id.human.alice");
        
        let commits = vec![
            create_test_commit(
                identity_id.clone(),
                0,
                IdentityPayload::Attestation {
                    attester_id: IdentityId::new("id.human.manager"),
                    attestation_type: AttestationType::DeptMember,
                    target_ref: "dept.finance".to_string(),
                    scope: "org.acme".to_string(),
                    expires_event_id: None,
                },
            ),
        ];
        
        let mut store = IdentityStore::new();
        store.replay(&commits);
        
        let decision = check_access(
            &identity_id,
            "dept.finance",
            TargetKind::Dept,
            None,
            &store,
        );
        
        assert!(decision.allow);
        assert_eq!(decision.path.len(), 1);
        assert!(decision.path[0].effect.contains("ATTESTATION"));
        assert!(decision.path[0].effect.contains("DEPT_MEMBER"));
    }
}

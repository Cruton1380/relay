// apps/server/src/relay_physics/tests.rs
#![cfg(test)]

use super::*;
use serde_json::json;

#[test]
fn test_commit_index_monotonic() {
    let mut store = FilamentStore::new("test_data/filaments").unwrap();
    let verifier = Verifier::new();
    let auth_store = AuthorityStore::new();
    
    let fid = FilamentId::new("work.TEST");
    let unit = UnitId::new("unit.test.001");
    
    let commit1 = store.append_commit(
        fid.clone(),
        CommitData {
            op_type: "TEST_OP".to_string(),
            author_unit_ref: unit.clone(),
            payload: json!({}),
            causal_refs: CausalRefs::default(),
        },
        &verifier,
        &auth_store,
    ).unwrap();
    
    assert_eq!(commit1.commit_index, 1);
    
    let commit2 = store.append_commit(
        fid.clone(),
        CommitData {
            op_type: "TEST_OP".to_string(),
            author_unit_ref: unit,
            payload: json!({}),
            causal_refs: CausalRefs::default(),
        },
        &verifier,
        &auth_store,
    ).unwrap();
    
    assert_eq!(commit2.commit_index, 2);
    
    // Cleanup
    std::fs::remove_dir_all("test_data").ok();
}

#[test]
fn test_schema_invalid_rejects() {
    let mut store = FilamentStore::new("test_data/filaments").unwrap();
    let verifier = Verifier::new();
    let auth_store = AuthorityStore::new();
    
    let result = store.append_commit(
        FilamentId::new("work.TEST"),
        CommitData {
            op_type: "".to_string(), // ❌ Empty op_type
            author_unit_ref: UnitId::new("unit.test.001"),
            payload: json!({}),
            causal_refs: CausalRefs::default(),
        },
        &verifier,
        &auth_store,
    );
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.reason_code, ReasonCode::SchemaInvalid));
    
    std::fs::remove_dir_all("test_data").ok();
}

#[test]
fn test_ref_invalid_rejects() {
    let mut store = FilamentStore::new("test_data/filaments").unwrap();
    let verifier = Verifier::new();
    let auth_store = AuthorityStore::new();
    
    let result = store.append_commit(
        FilamentId::new("work.TEST"),
        CommitData {
            op_type: "TEST_OP".to_string(),
            author_unit_ref: UnitId::new("unit.test.001"),
            payload: json!({}),
            causal_refs: CausalRefs {
                inputs: vec![CommitRef(String::from("work.NONEXISTENT@c999"))], // ❌ Invalid
                evidence: vec![],
                authority_ref: None,
            },
        },
        &verifier,
        &auth_store,
    );
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.reason_code, ReasonCode::RefInvalid));
    assert!(err.details.contains("work.NONEXISTENT@c999"));
    
    std::fs::remove_dir_all("test_data").ok();
}

#[test]
fn test_authority_missing_rejects() {
    let mut store = FilamentStore::new("test_data/filaments").unwrap();
    let verifier = Verifier::new();
    let auth_store = AuthorityStore::new();
    
    // TASK_ASSIGN requires authority
    let result = store.append_commit(
        FilamentId::new("work.TEST"),
        CommitData {
            op_type: agent_ops::TASK_ASSIGN.to_string(),
            author_unit_ref: UnitId::new("unit.test.001"),
            payload: json!({}),
            causal_refs: CausalRefs {
                inputs: vec![],
                evidence: vec![],
                authority_ref: None, // ❌ Missing
            },
        },
        &verifier,
        &auth_store,
    );
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.reason_code, ReasonCode::AuthorityMissing));
    
    std::fs::remove_dir_all("test_data").ok();
}

#[test]
fn test_output_proposed_accepted_when_valid() {
    let mut store = FilamentStore::new("test_data/filaments").unwrap();
    let verifier = Verifier::new();
    let mut auth_store = AuthorityStore::new();
    
    let unit = UnitId::new("unit.test.001");
    
    // Grant authority
    auth_store.grant_op(&unit, agent_ops::OUTPUT_PROPOSED);
    
    let result = store.append_commit(
        FilamentId::new("work.TEST"),
        CommitData {
            op_type: agent_ops::OUTPUT_PROPOSED.to_string(),
            author_unit_ref: unit,
            payload: json!({"operation": "test"}),
            causal_refs: CausalRefs {
                inputs: vec![],
                evidence: vec!["evidence.E1".to_string()], // Required
                authority_ref: Some("auth.test".to_string()),
            },
        },
        &verifier,
        &auth_store,
    );
    
    assert!(result.is_ok());
    let commit = result.unwrap();
    assert_eq!(commit.commit_index, 1);
    assert_eq!(commit.op_type, agent_ops::OUTPUT_PROPOSED);
    
    std::fs::remove_dir_all("test_data").ok();
}

#[test]
fn test_output_rejected_transitions_to_blocked() {
    let mut unit_store = UnitStore::new();
    let unit_id = UnitId::new("unit.test.001");
    
    // Create unit in Working state
    unit_store.create_unit(unit_id.clone());
    unit_store.apply_task_assign(&unit_id, FilamentId::new("work.TEST"));
    unit_store.apply_attach(&unit_id, FilamentId::new("work.TEST"));
    
    assert_eq!(unit_store.get_unit(&unit_id).unwrap().state, UnitState::Working);
    
    // Apply OUTPUT_REJECTED
    unit_store.apply_output_rejected(&unit_id);
    
    assert_eq!(unit_store.get_unit(&unit_id).unwrap().state, UnitState::Blocked);
}

#[test]
fn test_evidence_missing_rejects() {
    let mut store = FilamentStore::new("test_data/filaments").unwrap();
    let verifier = Verifier::new();
    let mut auth_store = AuthorityStore::new();
    
    let unit = UnitId::new("unit.test.001");
    auth_store.grant_op(&unit, agent_ops::OUTPUT_PROPOSED);
    
    let result = store.append_commit(
        FilamentId::new("work.TEST"),
        CommitData {
            op_type: agent_ops::OUTPUT_PROPOSED.to_string(),
            author_unit_ref: unit,
            payload: json!({}),
            causal_refs: CausalRefs {
                inputs: vec![],
                evidence: vec![], // ❌ Empty (required for OUTPUT_PROPOSED)
                authority_ref: Some("auth.test".to_string()),
            },
        },
        &verifier,
        &auth_store,
    );
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.reason_code, ReasonCode::EvidenceMissing));
    assert!(err.suggested_fix.is_some());
    
    std::fs::remove_dir_all("test_data").ok();
}

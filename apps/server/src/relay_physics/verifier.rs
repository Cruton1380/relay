// apps/server/src/relay_physics/verifier.rs
use crate::relay_physics::{
    agent_ops::requires_authority,
    AuthorityStore, CommitData, CommitRejectedError, FilamentId, FilamentStore, ReasonCode,
};

#[derive(Debug, Default)]
pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    /// 4-stage pipeline: schema → refs → authority → evidence
    pub fn verify(
        &self,
        filament_id: &FilamentId,
        commit_data: &CommitData,
        filament_store: &FilamentStore,
        authority_store: &AuthorityStore,
    ) -> Result<(), CommitRejectedError> {
        self.verify_schema(filament_id, commit_data)?;
        self.verify_refs(commit_data, filament_store)?;
        self.verify_authority(commit_data, authority_store)?;
        self.verify_evidence(commit_data)?;
        Ok(())
    }

    fn verify_schema(
        &self,
        _filament_id: &FilamentId,
        commit_data: &CommitData,
    ) -> Result<(), CommitRejectedError> {
        if commit_data.op_type.trim().is_empty() {
            return Err(CommitRejectedError::new(
                ReasonCode::SchemaInvalid,
                "op_type must be non-empty",
            ));
        }
        // MVP: ensure payload exists (always true since serde_json::Value)
        Ok(())
    }

    fn verify_refs(
        &self,
        commit_data: &CommitData,
        filament_store: &FilamentStore,
    ) -> Result<(), CommitRejectedError> {
        // inputs must exist if provided
        for cref in &commit_data.causal_refs.inputs {
            if !filament_store.commit_ref_exists(cref) {
                return Err(
                    CommitRejectedError::new(ReasonCode::RefInvalid, format!("Missing input ref: {}", cref.as_str()))
                        .with_fix("Provide a valid inputs[] CommitRef that exists on-ledger"),
                );
            }
        }
        Ok(())
    }

    fn verify_authority(
        &self,
        commit_data: &CommitData,
        authority_store: &AuthorityStore,
    ) -> Result<(), CommitRejectedError> {
        if requires_authority(&commit_data.op_type) {
            let Some(auth_ref) = &commit_data.causal_refs.authority_ref else {
                return Err(
                    CommitRejectedError::new(ReasonCode::AuthorityMissing, "authority_ref is required for this op_type")
                        .with_fix("Attach causal_refs.authority_ref with delegation path proof (MVP: capability grant)"),
                );
            };

            // MVP capability policy: check (unit, op_type) allowed
            if !authority_store.is_authorized(&commit_data.author_unit_ref, &commit_data.op_type, auth_ref) {
                return Err(
                    CommitRejectedError::new(
                        ReasonCode::AuthorityDenied,
                        format!("Unit not authorized for op_type={} with authority_ref={}", commit_data.op_type, auth_ref),
                    )
                    .with_fix("Request delegation for required capability or attach the correct authority_ref"),
                );
            }
        }
        Ok(())
    }

    fn verify_evidence(&self, commit_data: &CommitData) -> Result<(), CommitRejectedError> {
        // MVP policy:
        // - OUTPUT_PROPOSED must include evidence refs (later: per-op policies)
        if commit_data.op_type == crate::relay_physics::agent_ops::OUTPUT_PROPOSED {
            if commit_data.causal_refs.evidence.is_empty() {
                return Err(
                    CommitRejectedError::new(ReasonCode::EvidenceMissing, "OUTPUT_PROPOSED requires evidence refs")
                        .with_fix("Attach causal_refs.evidence[] (MVP: strings; later: structured git refs)"),
                );
            }
        }
        Ok(())
    }
}

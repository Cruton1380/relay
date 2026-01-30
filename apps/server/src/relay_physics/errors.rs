use serde::{Deserialize, Serialize};
use std::fmt;

/// Reason codes for commit rejection (LOCKED)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReasonCode {
    SchemaInvalid,
    RefInvalid,
    AuthorityMissing,
    AuthorityDenied,
    EvidenceMissing,
    EvidenceInvalid,
    ConflictDetected,
    ScopeViolation,
    TimeboxExceeded,
    CustomRuleFailed,
}

impl fmt::Display for ReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReasonCode::SchemaInvalid => write!(f, "SCHEMA_INVALID"),
            ReasonCode::RefInvalid => write!(f, "REF_INVALID"),
            ReasonCode::AuthorityMissing => write!(f, "AUTHORITY_MISSING"),
            ReasonCode::AuthorityDenied => write!(f, "AUTHORITY_DENIED"),
            ReasonCode::EvidenceMissing => write!(f, "EVIDENCE_MISSING"),
            ReasonCode::EvidenceInvalid => write!(f, "EVIDENCE_INVALID"),
            ReasonCode::ConflictDetected => write!(f, "CONFLICT_DETECTED"),
            ReasonCode::ScopeViolation => write!(f, "SCOPE_VIOLATION"),
            ReasonCode::TimeboxExceeded => write!(f, "TIMEBOX_EXCEEDED"),
            ReasonCode::CustomRuleFailed => write!(f, "CUSTOM_RULE_FAILED"),
        }
    }
}

/// Commit rejection error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitRejectedError {
    pub reason_code: ReasonCode,
    pub details: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fix: Option<String>,
}

impl CommitRejectedError {
    pub fn new(reason_code: ReasonCode, details: impl Into<String>) -> Self {
        Self {
            reason_code,
            details: details.into(),
            suggested_fix: None,
        }
    }
    
    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.suggested_fix = Some(fix.into());
        self
    }
}

impl fmt::Display for CommitRejectedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.reason_code, self.details)
    }
}

impl std::error::Error for CommitRejectedError {}

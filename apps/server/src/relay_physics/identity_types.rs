// apps/server/src/relay_physics/identity_types.rs
// PR #9: Identity Filaments v1
//
// Identity = filament tree of all actions, attestations, bindings, and scars
// Access = derived from identity intersections, not permissions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identity identifier (e.g., "id.human.alice", "id.ai.agent.forecast_03")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityId(pub String);

impl IdentityId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IdentityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identity commit (append-only log entry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCommit {
    pub schema_version: u32,
    pub identity_id: IdentityId,
    pub commit_index: u64,
    pub timestamp: String, // ISO 8601
    pub payload: IdentityPayload,
    pub authority_ref: Option<String>, // Who authorized this (future use)
}

/// Identity commit payload types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IdentityPayload {
    #[serde(rename = "SELF_CLAIM")]
    SelfClaim {
        claim_key: String,
        claim_value: String,
        scope: String,
    },
    
    #[serde(rename = "ATTESTATION")]
    Attestation {
        attester_id: IdentityId,
        attestation_type: AttestationType,
        target_ref: String,
        scope: String,
        expires_event_id: Option<u64>,
    },
    
    #[serde(rename = "BINDING")]
    Binding {
        binding_type: BindingType,
        target_ref: String,
        scope: String,
        expires_event_id: Option<u64>,
    },
    
    #[serde(rename = "SCAR_APPLIED")]
    ScarApplied {
        scar_type: ScarType,
        scope: String,
        reason: String,
        severity: u32,
    },
    
    #[serde(rename = "AUTHORITY_GRANTED")]
    AuthorityGranted {
        grant_type: String,
        target_ref: String,
        scope: String,
        duration_events: Option<u64>,
        amount: f64,
    },
}

/// Attestation types (external validation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttestationType {
    DeptMember,
    OrgMember,
    SkillVerified,
    Trusted,
}

impl std::fmt::Display for AttestationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttestationType::DeptMember => write!(f, "DEPT_MEMBER"),
            AttestationType::OrgMember => write!(f, "ORG_MEMBER"),
            AttestationType::SkillVerified => write!(f, "SKILL_VERIFIED"),
            AttestationType::Trusted => write!(f, "TRUSTED"),
        }
    }
}

/// Binding types (structural links)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BindingType {
    BuildingStaff,
    BuildingOwner,
    OrgMember,
    DeptMember,
    ChannelMember,
}

impl std::fmt::Display for BindingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BindingType::BuildingStaff => write!(f, "BUILDING_STAFF"),
            BindingType::BuildingOwner => write!(f, "BUILDING_OWNER"),
            BindingType::OrgMember => write!(f, "ORG_MEMBER"),
            BindingType::DeptMember => write!(f, "DEPT_MEMBER"),
            BindingType::ChannelMember => write!(f, "CHANNEL_MEMBER"),
        }
    }
}

/// Scar types (access restrictions)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScarType {
    AccessRestrict,
    RequiresEscort,
    Untrusted,
    RateLimited,
}

impl std::fmt::Display for ScarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScarType::AccessRestrict => write!(f, "ACCESS_RESTRICT"),
            ScarType::RequiresEscort => write!(f, "REQUIRES_ESCORT"),
            ScarType::Untrusted => write!(f, "UNTRUSTED"),
            ScarType::RateLimited => write!(f, "RATE_LIMITED"),
        }
    }
}

/// Derived identity state (replayed from commits)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityState {
    pub identity_id: IdentityId,
    pub commit_count: u64,
    pub claims: HashMap<String, String>,
    pub attestations: Vec<Attestation>,
    pub bindings: Vec<Binding>,
    pub scars: Vec<Scar>,
    pub authority_grants: Vec<AuthorityGrant>,
}

impl IdentityState {
    pub fn new(identity_id: IdentityId) -> Self {
        Self {
            identity_id,
            commit_count: 0,
            claims: HashMap::new(),
            attestations: Vec::new(),
            bindings: Vec::new(),
            scars: Vec::new(),
            authority_grants: Vec::new(),
        }
    }
}

/// Attestation (active)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub commit_index: u64,
    pub attester_id: IdentityId,
    pub attestation_type: AttestationType,
    pub target_ref: String,
    pub scope: String,
    pub expires_event_id: Option<u64>,
}

/// Binding (active)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub commit_index: u64,
    pub binding_type: BindingType,
    pub target_ref: String,
    pub scope: String,
    pub expires_event_id: Option<u64>,
}

/// Scar (permanent record)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scar {
    pub commit_index: u64,
    pub scar_type: ScarType,
    pub scope: String,
    pub reason: String,
    pub severity: u32,
}

/// Authority grant (temporary)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityGrant {
    pub commit_index: u64,
    pub grant_type: String,
    pub target_ref: String,
    pub scope: String,
    pub duration_events: Option<u64>,
    pub amount: f64,
}

/// Access decision (output of access check)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessDecision {
    pub allow: bool,
    pub reason: String,
    pub path: Vec<AccessPathStep>,
    pub blocked_by: Option<String>, // Commit ref that blocked access
}

impl AccessDecision {
    pub fn allow(reason: String, path: Vec<AccessPathStep>) -> Self {
        Self {
            allow: true,
            reason,
            path,
            blocked_by: None,
        }
    }
    
    pub fn deny(reason: String, blocked_by: Option<String>) -> Self {
        Self {
            allow: false,
            reason,
            path: Vec::new(),
            blocked_by,
        }
    }
}

/// Access path step (explanation of why access was granted)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPathStep {
    pub commit_ref: String, // e.g., "id.human.alice@c3"
    pub effect: String,     // e.g., "ATTESTATION:DEPT_MEMBER->dept.finance"
}

/// Target kind (for access checks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TargetKind {
    Org,
    Dept,
    Building,
    Channel,
    Filament,
}

impl std::fmt::Display for TargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetKind::Org => write!(f, "ORG"),
            TargetKind::Dept => write!(f, "DEPT"),
            TargetKind::Building => write!(f, "BUILDING"),
            TargetKind::Channel => write!(f, "CHANNEL"),
            TargetKind::Filament => write!(f, "FILAMENT"),
        }
    }
}

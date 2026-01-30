//! Relay Physics Layer - Coordination Substrate
//! 
//! This module implements:
//! - Append-only commit logs per filament
//! - Deterministic commitIndex allocation
//! - Pre-append verification pipeline
//! - SCV unit state machine (commit-driven only)
//! - Authority delegation chains
//! - Real-time event stream (SSE)

pub mod types;
pub mod commit;
pub mod filament_store;
pub mod unit_store;
pub mod authority_store;
pub mod verifier;
pub mod errors;
pub mod agent_ops;
pub mod events;
pub mod event_log;
pub mod commit_processor;
pub mod commit_bundle;
pub mod main_integration;

// PR #5: RenderSpec v1
pub mod renderspec;
pub mod renderspec_generator;
pub mod renderspec_validator;
pub mod renderspec_invariant_gates;

// PR #6: Buildings
pub mod building_store;
pub mod building_seeder;

// PR #7: Tasks
pub mod task_store;

// PR #8: Shipments
pub mod shipment_store;
pub mod route_generator;

// PR #9: Identity Filaments
pub mod identity_types;
pub mod identity_log;
pub mod identity_store;
pub mod access_derivation;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod sse_tests;

// Re-exports
pub use types::*;
pub use commit::*;
pub use filament_store::FilamentStore;
pub use unit_store::UnitStore;
pub use authority_store::AuthorityStore;
pub use verifier::Verifier;
pub use errors::*;
pub use agent_ops::*;
pub use events::*;
pub use event_log::{EventLog, EventLogEntry};
pub use commit_processor::CommitProcessor;
pub use commit_bundle::{CommitBundle, CommitSource, CommitRefFormat, CommitNotFoundError};
pub use renderspec::*;
pub use renderspec_generator::*;
pub use renderspec_validator::{validate_renderspec, ValidationResult};
pub use renderspec_invariant_gates::{check_invariant_gates, InvariantGateResult, GateViolation};
pub use building_store::BuildingStore;
pub use building_seeder::seed_buildings_if_empty;
pub use task_store::TaskStore;
pub use shipment_store::ShipmentStore;
pub use route_generator::*;
pub use identity_types::*;
pub use identity_log::IdentityLog;
pub use identity_store::IdentityStore;
pub use access_derivation::check_access;

//! Agent operation constants (LOCKED)

pub const TASK_ASSIGN: &str = "TASK_ASSIGN";
pub const UNIT_ATTACH: &str = "UNIT_ATTACH";
pub const UNIT_DETACH: &str = "UNIT_DETACH";
pub const PROMPT_STEP_RUN: &str = "PROMPT_STEP_RUN";
pub const OUTPUT_PROPOSED: &str = "OUTPUT_PROPOSED";
pub const OUTPUT_ACCEPTED: &str = "OUTPUT_ACCEPTED";
pub const OUTPUT_REJECTED: &str = "OUTPUT_REJECTED";
pub const SCV_CHANNEL_CANCEL: &str = "SCV_CHANNEL_CANCEL";

/// Check if operation requires authority
pub fn requires_authority(op_type: &str) -> bool {
    matches!(
        op_type,
        TASK_ASSIGN | OUTPUT_PROPOSED | SCV_CHANNEL_CANCEL
    )
}

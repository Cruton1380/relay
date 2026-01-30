// RenderSpec Invariant Gates
// Hard-fail checks to enforce physics laws in rendering

use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct GateViolation {
    pub gate_id: String,
    pub description: String,
    pub node_id: Option<String>,
    pub suggestion: String,
}

#[derive(Debug, Clone)]
pub struct InvariantGateResult {
    pub passed: bool,
    pub violations: Vec<GateViolation>,
}

impl InvariantGateResult {
    pub fn new() -> Self {
        Self {
            passed: true,
            violations: Vec::new(),
        }
    }

    pub fn add_violation(&mut self, violation: GateViolation) {
        self.passed = false;
        self.violations.push(violation);
    }

    pub fn merge(&mut self, other: InvariantGateResult) {
        if !other.passed {
            self.passed = false;
            self.violations.extend(other.violations);
        }
    }
}

/// Run all invariant gates on a RenderSpec
pub fn check_invariant_gates(spec: &Value) -> InvariantGateResult {
    let mut result = InvariantGateResult::new();

    // Gate 1: No randomness in output
    result.merge(gate_no_randomness(spec));

    // Gate 2: No current time affecting geometry
    result.merge(gate_no_time_dependent_geometry(spec));

    // Gate 3: Materials are semantic tags (not RGB)
    result.merge(gate_semantic_materials(spec));

    // Gate 4: Filaments have stable timebox IDs
    result.merge(gate_filament_stable_timeboxes(spec));

    result
}

/// Gate 1: No Randomness
/// Checks for patterns that suggest non-deterministic generation
fn gate_no_randomness(spec: &Value) -> InvariantGateResult {
    let mut result = InvariantGateResult::new();

    // Check node IDs for random patterns (UUIDs, random suffixes)
    if let Some(nodes) = spec.get("nodes").and_then(|n| n.as_array()) {
        for node in nodes {
            if let Some(id) = node.get("id").and_then(|i| i.as_str()) {
                // Check for UUID pattern (8-4-4-4-12 hex digits)
                if is_uuid_like(id) {
                    result.add_violation(GateViolation {
                        gate_id: "gate_no_randomness".to_string(),
                        description: format!(
                            "Node ID appears to be a random UUID: '{}'", id
                        ),
                        node_id: Some(id.to_string()),
                        suggestion: "Use deterministic hash-based IDs instead of UUIDs".to_string(),
                    });
                }

                // Check for "_random_" or similar patterns
                if id.contains("_random_") || id.contains("_rand_") {
                    result.add_violation(GateViolation {
                        gate_id: "gate_no_randomness".to_string(),
                        description: format!(
                            "Node ID contains random marker: '{}'", id
                        ),
                        node_id: Some(id.to_string()),
                        suggestion: "Remove random suffixes from IDs".to_string(),
                    });
                }
            }
        }
    }

    result
}

/// Gate 2: No Time-Dependent Geometry
/// Checks for patterns suggesting geometry changes based on current time
fn gate_no_time_dependent_geometry(spec: &Value) -> InvariantGateResult {
    let mut result = InvariantGateResult::new();

    // Check if generated_from.timestamp matches any geometry values
    // (This is a heuristic - catching obvious Date.now() usage)
    if let Some(timestamp) = spec.get("generated_from")
        .and_then(|gf| gf.get("timestamp"))
        .and_then(|ts| ts.as_u64())
    {
        if let Some(nodes) = spec.get("nodes").and_then(|n| n.as_array()) {
            for node in nodes {
                // Check transform positions for timestamp values
                if let Some(position) = node.get("transform")
                    .and_then(|t| t.get("position"))
                    .and_then(|p| p.as_array())
                {
                    for coord in position {
                        if let Some(val) = coord.as_f64() {
                            // Check if coordinate suspiciously close to timestamp
                            let ts_f64 = timestamp as f64;
                            if (val - ts_f64).abs() < 1000.0 && val > 1000000000000.0 {
                                result.add_violation(GateViolation {
                                    gate_id: "gate_no_time_dependent_geometry".to_string(),
                                    description: format!(
                                        "Position coordinate appears to be derived from timestamp: {}",
                                        val
                                    ),
                                    node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                                    suggestion: "Use stable coordinates, not Date.now() or timestamps".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

/// Gate 3: Semantic Materials Only (No RGB)
/// Materials must be semantic tags like "unit_active", not "#FF5733"
fn gate_semantic_materials(spec: &Value) -> InvariantGateResult {
    let mut result = InvariantGateResult::new();

    if let Some(nodes) = spec.get("nodes").and_then(|n| n.as_array()) {
        for node in nodes {
            if let Some(material) = node.get("material").and_then(|m| m.as_str()) {
                // Check for RGB hex patterns
                if material.starts_with("#") {
                    result.add_violation(GateViolation {
                        gate_id: "gate_semantic_materials".to_string(),
                        description: format!(
                            "Material is RGB hex code instead of semantic tag: '{}'",
                            material
                        ),
                        node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                        suggestion: "Use semantic material tags (e.g., 'unit_active', 'building_vendor')".to_string(),
                    });
                }

                // Check for rgb() patterns
                if material.starts_with("rgb(") || material.starts_with("rgba(") {
                    result.add_violation(GateViolation {
                        gate_id: "gate_semantic_materials".to_string(),
                        description: format!(
                            "Material is RGB function instead of semantic tag: '{}'",
                            material
                        ),
                        node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                        suggestion: "Use semantic material tags (e.g., 'filament_causal', 'task_in_progress')".to_string(),
                    });
                }

                // Check for numeric RGB tuples (less common but possible)
                if material.starts_with("[") && material.contains(",") {
                    result.add_violation(GateViolation {
                        gate_id: "gate_semantic_materials".to_string(),
                        description: format!(
                            "Material appears to be numeric RGB tuple: '{}'",
                            material
                        ),
                        node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                        suggestion: "Use semantic material tags".to_string(),
                    });
                }
            }
        }
    }

    result
}

/// Gate 4: Filaments Must Have Stable Timebox IDs
/// Filament nodes must reference deterministic timebox IDs
fn gate_filament_stable_timeboxes(spec: &Value) -> InvariantGateResult {
    let mut result = InvariantGateResult::new();

    // Collect all timebox IDs
    let mut timebox_ids: HashSet<String> = HashSet::new();
    if let Some(nodes) = spec.get("nodes").and_then(|n| n.as_array()) {
        for node in nodes {
            if node.get("kind").and_then(|k| k.as_str()) == Some("timebox") {
                if let Some(id) = node.get("id").and_then(|i| i.as_str()) {
                    timebox_ids.insert(id.to_string());
                }

                // Check timebox has commit_ref in props
                if !node.get("props")
                    .and_then(|p| p.get("commit_ref"))
                    .and_then(|cr| cr.as_str())
                    .is_some()
                {
                    result.add_violation(GateViolation {
                        gate_id: "gate_filament_stable_timeboxes".to_string(),
                        description: "Timebox missing commit_ref in props".to_string(),
                        node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                        suggestion: "Add commit_ref to timebox props (e.g., 'work.W123@c42')".to_string(),
                    });
                }

                // Check commit_ref format
                if let Some(commit_ref) = node.get("props")
                    .and_then(|p| p.get("commit_ref"))
                    .and_then(|cr| cr.as_str())
                {
                    if !commit_ref.contains("@c") {
                        result.add_violation(GateViolation {
                            gate_id: "gate_filament_stable_timeboxes".to_string(),
                            description: format!(
                                "Timebox commit_ref does not follow format 'filament@cN': '{}'",
                                commit_ref
                            ),
                            node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                            suggestion: "Use format 'filament_id@cN' (e.g., 'work.W123@c42')".to_string(),
                        });
                    }
                }
            }
        }

        // Check filament nodes reference their timeboxes
        for node in nodes {
            if node.get("kind").and_then(|k| k.as_str()) == Some("filament") {
                if let Some(filament_id) = node.get("props")
                    .and_then(|p| p.get("filament_id"))
                    .and_then(|fid| fid.as_str())
                {
                    // Check if there are corresponding timebox nodes
                    let has_timeboxes = timebox_ids.iter().any(|tb_id| {
                        tb_id.starts_with(&format!("timebox.{}", filament_id))
                    });

                    if !has_timeboxes {
                        result.add_violation(GateViolation {
                            gate_id: "gate_filament_stable_timeboxes".to_string(),
                            description: format!(
                                "Filament '{}' has no corresponding timebox nodes",
                                filament_id
                            ),
                            node_id: node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                            suggestion: "Emit timebox nodes for filament commits".to_string(),
                        });
                    }
                }
            }
        }
    }

    result
}

/// Helper: Check if string looks like a UUID
fn is_uuid_like(s: &str) -> bool {
    // UUID format: 8-4-4-4-12 hex digits
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    
    if parts[0].len() != 8 || parts[1].len() != 4 || 
       parts[2].len() != 4 || parts[3].len() != 4 || 
       parts[4].len() != 12 {
        return false;
    }

    // Check all parts are hex
    parts.iter().all(|part| {
        part.chars().all(|c| c.is_ascii_hexdigit())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_gate_no_randomness_pass() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "unit.alice.123",
                    "kind": "unit",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "unit_active",
                    "props": {}
                }
            ]
        });

        let result = gate_no_randomness(&spec);
        assert!(result.passed, "Violations: {:?}", result.violations);
    }

    #[test]
    fn test_gate_no_randomness_fail_uuid() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "kind": "unit",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "unit_active",
                    "props": {}
                }
            ]
        });

        let result = gate_no_randomness(&spec);
        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.description.contains("UUID")));
    }

    #[test]
    fn test_gate_semantic_materials_pass() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "unit1",
                    "kind": "unit",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "unit_active",
                    "props": {}
                }
            ]
        });

        let result = gate_semantic_materials(&spec);
        assert!(result.passed);
    }

    #[test]
    fn test_gate_semantic_materials_fail_hex() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "unit1",
                    "kind": "unit",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "#FF5733",
                    "props": {}
                }
            ]
        });

        let result = gate_semantic_materials(&spec);
        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.description.contains("RGB hex")));
    }

    #[test]
    fn test_gate_semantic_materials_fail_rgb_function() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "unit1",
                    "kind": "unit",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "rgb(255, 87, 51)",
                    "props": {}
                }
            ]
        });

        let result = gate_semantic_materials(&spec);
        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.description.contains("RGB function")));
    }

    #[test]
    fn test_gate_filament_timeboxes_pass() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/filament/work.W123",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "filament.work.W123",
                    "kind": "filament",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "filament_causal",
                    "props": {
                        "filament_id": "work.W123"
                    }
                },
                {
                    "id": "timebox.work.W123.c42",
                    "kind": "timebox",
                    "transform": {
                        "position": [1.0, 2.0, 3.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [0.5, 0.5, 0.5]
                    },
                    "material": "timebox_commit",
                    "props": {
                        "commit_ref": "work.W123@c42",
                        "commit_index": 42
                    }
                }
            ]
        });

        let result = gate_filament_stable_timeboxes(&spec);
        assert!(result.passed, "Violations: {:?}", result.violations);
    }

    #[test]
    fn test_gate_filament_timeboxes_fail_missing_commit_ref() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/filament/work.W123",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "timebox.work.W123.c42",
                    "kind": "timebox",
                    "transform": {
                        "position": [1.0, 2.0, 3.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [0.5, 0.5, 0.5]
                    },
                    "material": "timebox_commit",
                    "props": {
                        "commit_index": 42
                        // Missing commit_ref
                    }
                }
            ]
        });

        let result = gate_filament_stable_timeboxes(&spec);
        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.description.contains("missing commit_ref")));
    }

    #[test]
    fn test_all_gates_on_valid_spec() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "globe",
                    "kind": "globe",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "globe_base",
                    "props": {}
                }
            ]
        });

        let result = check_invariant_gates(&spec);
        assert!(result.passed, "Violations: {:?}", result.violations);
        assert_eq!(result.violations.len(), 0);
    }
}

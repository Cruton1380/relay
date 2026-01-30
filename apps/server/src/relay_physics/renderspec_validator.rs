// RenderSpec v1 Validator
// Enforces invariants for Layer 2 → Layer 3 contract

use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.valid = false;
        self.errors.push(error);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        if !other.valid {
            self.valid = false;
            self.errors.extend(other.errors);
        }
    }
}

/// Validate RenderSpec v1 JSON against locked specification
pub fn validate_renderspec(spec: &Value) -> ValidationResult {
    let mut result = ValidationResult::new();

    // 1. Check schema_version
    if let Some(version) = spec.get("schema_version") {
        if version.as_str() != Some("relay-render-v1") {
            result.add_error(
                "Invalid schema_version (must be 'relay-render-v1')".to_string()
            );
        }
    } else {
        result.add_error("Missing schema_version".to_string());
    }

    // 2. Check required top-level fields
    if !spec.get("generated_from").is_some() {
        result.add_error("Missing generated_from".to_string());
    }

    if !spec.get("nodes").is_some() {
        result.add_error("Missing nodes".to_string());
    } else if !spec["nodes"].is_array() {
        result.add_error("nodes must be array".to_string());
    }

    // 3. Validate generated_from
    if let Some(generated_from) = spec.get("generated_from") {
        result.merge(validate_generated_from(generated_from));
    }

    // 4. Validate nodes
    if let Some(nodes) = spec.get("nodes").and_then(|n| n.as_array()) {
        for (index, node) in nodes.iter().enumerate() {
            result.merge(validate_node(node, index));
        }
    }

    // 5. Check for unknown top-level keys
    let allowed_keys: HashSet<&str> = 
        ["schema_version", "generated_from", "nodes", "metadata"]
        .iter()
        .copied()
        .collect();

    if let Some(obj) = spec.as_object() {
        for key in obj.keys() {
            if !allowed_keys.contains(key.as_str()) {
                result.add_error(format!("Unknown top-level key: {}", key));
            }
        }
    }

    result
}

fn validate_generated_from(generated_from: &Value) -> ValidationResult {
    let mut result = ValidationResult::new();

    if !generated_from.get("endpoint").and_then(|e| e.as_str()).is_some() {
        result.add_error("Missing generated_from.endpoint".to_string());
    }

    if !generated_from.get("timestamp").and_then(|t| t.as_u64()).is_some() {
        result.add_error("Invalid generated_from.timestamp (must be number)".to_string());
    }

    result
}

fn validate_node(node: &Value, index: usize) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check required node fields
    if !node.get("id").and_then(|id| id.as_str()).is_some() {
        result.add_error(format!("Node {}: missing id", index));
    }

    if !node.get("kind").and_then(|k| k.as_str()).is_some() {
        result.add_error(format!("Node {}: missing kind", index));
    } else {
        // Validate kind is one of known types
        let valid_kinds = [
            "globe", "unit", "filament", "timebox", 
            "building", "task", "shipment"
        ];
        if let Some(kind) = node.get("kind").and_then(|k| k.as_str()) {
            if !valid_kinds.contains(&kind) {
                result.add_error(format!(
                    "Node {}: unknown kind '{}' (must be one of: {})",
                    index, kind, valid_kinds.join(", ")
                ));
            }
        }
    }

    if !node.get("transform").is_some() {
        result.add_error(format!("Node {}: missing transform", index));
    } else {
        result.merge(validate_transform(node.get("transform").unwrap(), index));
    }

    if !node.get("material").and_then(|m| m.as_str()).is_some() {
        result.add_error(format!("Node {}: missing material", index));
    } else {
        // Validate material is not RGB
        if let Some(material) = node.get("material").and_then(|m| m.as_str()) {
            if material.starts_with("#") || material.starts_with("rgb") {
                result.add_error(format!(
                    "Node {}: material must be semantic tag, not RGB (got '{}')",
                    index, material
                ));
            }
        }
    }

    if !node.get("props").is_some() {
        result.add_error(format!("Node {}: missing props", index));
    } else if !node["props"].is_object() {
        result.add_error(format!("Node {}: props must be object", index));
    }

    result
}

fn validate_transform(transform: &Value, node_index: usize) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Validate position
    if let Some(pos) = transform.get("position").and_then(|p| p.as_array()) {
        if pos.len() != 3 {
            result.add_error(format!(
                "Node {}: transform.position must have exactly 3 elements",
                node_index
            ));
        }
        if !pos.iter().all(|v| v.is_number()) {
            result.add_error(format!(
                "Node {}: transform.position must contain only numbers",
                node_index
            ));
        }
    } else {
        result.add_error(format!("Node {}: invalid transform.position", node_index));
    }

    // Validate rotation
    if let Some(rot) = transform.get("rotation").and_then(|r| r.as_array()) {
        if rot.len() != 3 {
            result.add_error(format!(
                "Node {}: transform.rotation must have exactly 3 elements",
                node_index
            ));
        }
        if !rot.iter().all(|v| v.is_number()) {
            result.add_error(format!(
                "Node {}: transform.rotation must contain only numbers",
                node_index
            ));
        }
    } else {
        result.add_error(format!("Node {}: invalid transform.rotation", node_index));
    }

    // Validate scale
    if let Some(scale) = transform.get("scale").and_then(|s| s.as_array()) {
        if scale.len() != 3 {
            result.add_error(format!(
                "Node {}: transform.scale must have exactly 3 elements",
                node_index
            ));
        }
        if !scale.iter().all(|v| v.is_number()) {
            result.add_error(format!(
                "Node {}: transform.scale must contain only numbers",
                node_index
            ));
        }
    } else {
        result.add_error(format!("Node {}: invalid transform.scale", node_index));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_minimal_renderspec() {
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

        let result = validate_renderspec(&spec);
        assert!(result.valid, "Errors: {:?}", result.errors);
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn test_missing_schema_version() {
        let spec = json!({
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": []
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Missing schema_version")));
    }

    #[test]
    fn test_wrong_schema_version() {
        let spec = json!({
            "schema_version": "relay-render-v2",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": []
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Invalid schema_version")));
    }

    #[test]
    fn test_missing_generated_from() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "nodes": []
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Missing generated_from")));
    }

    #[test]
    fn test_missing_nodes() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            }
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Missing nodes")));
    }

    #[test]
    fn test_nodes_not_array() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": {}
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("nodes must be array")));
    }

    #[test]
    fn test_node_missing_required_fields() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "test"
                    // Missing kind, transform, material, props
                }
            ]
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("missing kind")));
        assert!(result.errors.iter().any(|e| e.contains("missing transform")));
        assert!(result.errors.iter().any(|e| e.contains("missing material")));
        assert!(result.errors.iter().any(|e| e.contains("missing props")));
    }

    #[test]
    fn test_rgb_material_forbidden() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "test",
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

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| 
            e.contains("material must be semantic tag, not RGB")
        ));
    }

    #[test]
    fn test_invalid_transform() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "test",
                    "kind": "unit",
                    "transform": {
                        "position": [0.0, 0.0],  // Only 2 elements, should be 3
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "unit_active",
                    "props": {}
                }
            ]
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| 
            e.contains("transform.position must have exactly 3 elements")
        ));
    }

    #[test]
    fn test_unknown_top_level_key() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [],
            "experimental_feature": "test"
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| 
            e.contains("Unknown top-level key: experimental_feature")
        ));
    }

    #[test]
    fn test_unknown_node_kind() {
        let spec = json!({
            "schema_version": "relay-render-v1",
            "generated_from": {
                "endpoint": "/render/world",
                "timestamp": 1706486400000u64
            },
            "nodes": [
                {
                    "id": "test",
                    "kind": "unknown_kind",
                    "transform": {
                        "position": [0.0, 0.0, 0.0],
                        "rotation": [0.0, 0.0, 0.0],
                        "scale": [1.0, 1.0, 1.0]
                    },
                    "material": "test_material",
                    "props": {}
                }
            ]
        });

        let result = validate_renderspec(&spec);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| 
            e.contains("unknown kind")
        ));
    }

    #[test]
    fn test_valid_canonical_fixture() {
        // This should match the canonical fixture
        let fixture = std::fs::read_to_string(
            "../../fixtures/world_minimal.json"
        );
        
        if let Ok(fixture_str) = fixture {
            let spec: Value = serde_json::from_str(&fixture_str).unwrap();
            let result = validate_renderspec(&spec);
            assert!(result.valid, "Canonical fixture validation failed: {:?}", result.errors);
        } else {
            // If fixture not found, skip this test
            println!("Warning: Canonical fixture not found, skipping test");
        }
    }
}

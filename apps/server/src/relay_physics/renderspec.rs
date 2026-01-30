// apps/server/src/relay_physics/renderspec.rs
//
// RenderSpec v1 - Scene graph format for Layer 2→3 contract
// Reference: architecture@c5 (RenderSpec v1 Locked)

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// RenderSpec v1 - Top-level scene graph envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderSpec {
    /// Schema version (must be "relay-render-v1")
    pub schema_version: String,
    
    /// Provenance - what this scene was generated from
    pub generated_from: GeneratedFrom,
    
    /// Scene graph nodes
    pub nodes: Vec<Node>,
    
    /// Causal links (optional, for forensic chamber)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
    
    /// Animation intents (optional, keyed by event_id)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub animation_intents: Vec<AnimationIntent>,
}

impl RenderSpec {
    /// Create new RenderSpec with version "relay-render-v1"
    pub fn new() -> Self {
        Self {
            schema_version: "relay-render-v1".to_string(),
            generated_from: GeneratedFrom::now(),
            nodes: Vec::new(),
            links: Vec::new(),
            animation_intents: Vec::new(),
        }
    }
}

/// Provenance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFrom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_ref: Option<String>,
    
    pub timestamp: String, // RFC3339
}

impl GeneratedFrom {
    pub fn now() -> Self {
        Self {
            event_id: None,
            commit_ref: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    pub fn with_event_id(event_id: u64) -> Self {
        Self {
            event_id: Some(event_id),
            commit_ref: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    pub fn with_commit_ref(commit_ref: String) -> Self {
        Self {
            event_id: None,
            commit_ref: Some(commit_ref),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Scene graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Stable identifier
    pub id: String,
    
    /// Node type
    pub kind: String,
    
    /// Transform (position, rotation, scale)
    pub transform: Transform,
    
    /// Semantic material tag
    pub material: String,
    
    /// Node-specific properties (free-form, versioned keys)
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub props: serde_json::Map<String, JsonValue>,
    
    /// Filament geometry (only for kind="filament")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<FilamentGeometry>,
}

/// Transform (position, rotation, scale)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    /// World-space position [x, y, z]
    pub position: [f32; 3],
    
    /// Quaternion rotation [x, y, z, w]
    pub rotation: [f32; 4],
    
    /// Scale [x, y, z] (defaults to [1, 1, 1])
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            scale: [1.0, 1.0, 1.0],
        }
    }
}

/// Filament geometry (polyline representation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilamentGeometry {
    /// Polyline vertices [[x,y,z], ...]
    pub points: Vec<[f32; 3]>,
    
    /// Radius at each point (tube thickness)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radii: Option<Vec<f32>>,
    
    /// Commit indices (which commit each segment represents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_indices: Option<Vec<u64>>,
}

/// Causal link between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Source node ID
    pub from_id: String,
    
    /// Target node ID
    pub to_id: String,
    
    /// Link type ("input" | "authority" | "evidence")
    pub link_type: String,
    
    /// Semantic material tag
    pub material: String,
}

/// Animation intent (keyed by event_id)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationIntent {
    /// Target node ID
    pub target_id: String,
    
    /// Event ID that triggers this animation
    pub trigger_event_id: u64,
    
    /// Animation type
    pub animation_type: String,
    
    /// Duration in milliseconds
    pub duration_ms: u32,
    
    /// Animation-specific parameters
    #[serde(default)]
    pub params: serde_json::Map<String, JsonValue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderspec_default() {
        let spec = RenderSpec::new();
        assert_eq!(spec.schema_version, "relay-render-v1");
        assert!(spec.nodes.is_empty());
    }

    #[test]
    fn test_transform_default() {
        let transform = Transform::default();
        assert_eq!(transform.position, [0.0, 0.0, 0.0]);
        assert_eq!(transform.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(transform.scale, [1.0, 1.0, 1.0]);
    }
}

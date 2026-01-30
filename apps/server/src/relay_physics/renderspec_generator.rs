// apps/server/src/relay_physics/renderspec_generator.rs
//
// Generates RenderSpec v1 from Layer 2 truth objects
// Reference: architecture@c5 (RenderSpec v1 Locked)

use crate::relay_physics::{
    renderspec::*, CommitEvent, FilamentId, FilamentStore, Unit, UnitState, Building, BuildingType,
    Task, TaskState,
};
use serde_json::{json, Map, Value as JsonValue};
use std::collections::HashMap;

/// Generate complete world scene (globe + all units + all filaments + all buildings + all tasks)
/// PR #6: Now includes buildings (architecture@c9)
/// PR #7: Now includes tasks (architecture@c9 - Shopping as Production)
pub fn generate_world_scene(
    units: &[Unit],
    filaments: &HashMap<FilamentId, Vec<CommitEvent>>,
    buildings: &[Building],
    tasks: &[Task],
    shipments: &[Shipment],
) -> RenderSpec {
    let mut spec = RenderSpec::new();
    
    // Add globe
    spec.nodes.push(create_globe_node());
    
    // Add all buildings (PR #6)
    for building in buildings {
        spec.nodes.push(create_building_node(building));
    }
    
    // Add all units
    for unit in units {
        spec.nodes.push(create_unit_node(unit));
    }
    
    // Add all tasks (PR #7)
    for task in tasks {
        spec.nodes.push(create_task_node(task));
    }
    
    // Add all shipments (PR #8)
    for shipment in shipments {
        spec.nodes.push(create_shipment_node(shipment));
    }
    
    // Add all filaments
    for (filament_id, commits) in filaments {
        if !commits.is_empty() {
            spec.nodes.push(create_filament_node(filament_id, commits));
            
            // Add timeboxes for each commit
            for commit in commits {
                spec.nodes.push(create_timebox_node(commit));
            }
        }
    }
    
    spec
}

/// Generate forensic chamber scene for single commit
pub fn generate_commit_scene(commit: &CommitEvent) -> RenderSpec {
    let mut spec = RenderSpec::new();
    spec.generated_from.commit_ref = Some(commit.commit_ref.as_str().to_string());
    
    // Central timebox
    spec.nodes.push(create_timebox_node(commit));
    
    // TODO: Add causal links (when we have commit graph traversal)
    // TODO: Add author unit (when we have unit positions)
    
    spec
}

/// Generate filament thread scene
pub fn generate_filament_scene(
    filament_id: &FilamentId,
    commits: &[CommitEvent],
) -> RenderSpec {
    let mut spec = RenderSpec::new();
    
    if !commits.is_empty() {
        // Filament polyline
        spec.nodes.push(create_filament_node(filament_id, commits));
        
        // Timeboxes
        for commit in commits {
            spec.nodes.push(create_timebox_node(commit));
        }
    }
    
    spec
}

// --- Node Creators ---

fn create_globe_node() -> Node {
    Node {
        id: "globe.world".to_string(),
        kind: "globe".to_string(),
        transform: Transform::default(),
        material: "world_surface".to_string(),
        props: {
            let mut props = Map::new();
            props.insert("radius".to_string(), json!(10.0));
            props.insert("segments".to_string(), json!(64));
            props
        },
        geometry: None,
    }
}

fn create_unit_node(unit: &Unit) -> Node {
    // Deterministic position from unit ID hash
    let position = deterministic_position_from_id(unit.id.as_str());
    
    // Material based on state
    let material = match unit.state {
        UnitState::Idle => "scv_idle",
        UnitState::Moving => "scv_moving",
        UnitState::Working => "scv_working",
        UnitState::Blocked => "scv_blocked",
        UnitState::AwaitingAuthority => "scv_awaiting",
    };
    
    Node {
        id: unit.id.as_str().to_string(),
        kind: "unit".to_string(),
        transform: Transform {
            position,
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        },
        material: material.to_string(),
        props: {
            let mut props = Map::new();
            props.insert("state".to_string(), json!(format!("{:?}", unit.state)));
            if let Some(ref attached) = unit.attached_filament {
                props.insert("attached_filament".to_string(), json!(attached.as_str()));
            }
            if let Some(ref task) = unit.current_task_filament {
                props.insert("current_task_filament".to_string(), json!(task.as_str()));
            }
            props
        },
        geometry: None,
    }
}

fn create_filament_node(filament_id: &FilamentId, commits: &[CommitEvent]) -> Node {
    // Generate polyline from commits
    let points: Vec<[f32; 3]> = commits
        .iter()
        .enumerate()
        .map(|(i, _)| {
            // Deterministic polyline from filament ID + commit index
            let base_pos = deterministic_position_from_id(filament_id.as_str());
            [
                base_pos[0] + (i as f32) * 0.5,
                base_pos[1] + (i as f32) * 0.3,
                base_pos[2] + (i as f32) * 0.2,
            ]
        })
        .collect();
    
    let radii: Vec<f32> = vec![0.05; points.len()];
    let commit_indices: Vec<u64> = commits.iter().map(|c| c.commit_index).collect();
    
    let head_commit_index = commits.last().map(|c| c.commit_index).unwrap_or(0);
    
    Node {
        id: format!("filament.{}", filament_id.as_str()),
        kind: "filament".to_string(),
        transform: Transform::default(),
        material: "public_filament".to_string(), // TODO: Detect encryption from metadata
        props: {
            let mut props = Map::new();
            props.insert("filament_id".to_string(), json!(filament_id.as_str()));
            props.insert("head_commit_index".to_string(), json!(head_commit_index));
            props
        },
        geometry: Some(FilamentGeometry {
            points,
            radii: Some(radii),
            commit_indices: Some(commit_indices),
        }),
    }
}

fn create_timebox_node(commit: &CommitEvent) -> Node {
    // Position: Place on filament at commit index
    let position = deterministic_position_from_id(&format!(
        "{}@c{}",
        commit.filament_id.as_str(),
        commit.commit_index
    ));
    
    Node {
        id: format!("timebox.{}", commit.commit_ref.as_str()),
        kind: "timebox".to_string(),
        transform: Transform {
            position,
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [0.1, 0.1, 0.1],
        },
        material: "commit_accepted".to_string(), // TODO: Detect rejected from metadata
        props: {
            let mut props = Map::new();
            props.insert("commit_ref".to_string(), json!(commit.commit_ref.as_str()));
            props.insert("commit_index".to_string(), json!(commit.commit_index));
            props.insert("op_type".to_string(), json!(&commit.op_type));
            props.insert("author_unit_ref".to_string(), json!(commit.author_unit_ref.as_str()));
            props.insert("verification_state".to_string(), json!("accepted"));
            props
        },
        geometry: None,
    }
}

/// Create building node (PR #6 - architecture@c9)
/// Buildings are physical entities on globe with catalog data in props
fn create_building_node(building: &Building) -> Node {
    // Position from geo_anchor (primary) or deterministic hash (fallback)
    let position = [
        building.geo_anchor.lat as f32,
        building.geo_anchor.lng as f32,
        building.geo_anchor.altitude.unwrap_or(0.0) as f32,
    ];
    
    // Material based on building type
    let material = match building.building_type {
        BuildingType::Vendor => "vendor_building",
        BuildingType::Partnership => "partnership_building",
        BuildingType::Civic => "civic_building",
        BuildingType::Logistics => "logistics_building",
        BuildingType::Community => "community_building",
    };
    
    // Sort catalog for determinism
    let mut catalog = building.catalog.clone();
    catalog.sort_by(|a, b| a.product_id.cmp(&b.product_id));
    
    Node {
        id: building.building_id.as_str().to_string(),
        kind: "building".to_string(),
        transform: Transform {
            position,
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        },
        material: material.to_string(),
        props: {
            let mut props = Map::new();
            props.insert("building_id".to_string(), json!(building.building_id.as_str()));
            props.insert("building_type".to_string(), json!(format!("{}", building.building_type)));
            props.insert("status".to_string(), json!(format!("{:?}", building.status)));
            
            if let Some(ref owner) = building.owner_ref {
                props.insert("owner_ref".to_string(), json!(owner.as_str()));
            }
            
            // Catalog stays in building props (NOT separate nodes per architecture@c9 clarification)
            if !catalog.is_empty() {
                props.insert("catalog".to_string(), json!(catalog));
            }
            
            // Merge custom props
            for (key, value) in &building.props {
                props.insert(key.clone(), value.clone());
            }
            
            props
        },
        geometry: None,
    }
}

/// Create task node (PR #7 - architecture@c9 - Shopping as Production)
/// Tasks are build queue entries that show progression through production/shipping
fn create_task_node(task: &Task) -> Node {
    // Material based on state
    let material = match task.state {
        TaskState::Queued => "task_queued",
        TaskState::Packing => "task_packing",
        TaskState::Dispatched => "task_dispatched",
        TaskState::InTransit => "task_in_transit",
        TaskState::Delivered => "task_delivered",
        TaskState::Failed => "task_failed",
    };
    
    Node {
        id: task.task_id.as_str().to_string(),
        kind: "task".to_string(),
        transform: Transform::default(), // Tasks don't have spatial position (abstract build queue)
        material: material.to_string(),
        props: {
            let mut props = Map::new();
            props.insert("task_id".to_string(), json!(task.task_id.as_str()));
            props.insert("task_type".to_string(), json!(format!("{}", task.task_type)));
            props.insert("building_ref".to_string(), json!(task.building_ref.as_str()));
            props.insert("requester_ref".to_string(), json!(task.requester_ref.as_str()));
            
            if let Some(ref product) = task.product_ref {
                props.insert("product_ref".to_string(), json!(product));
            }
            
            props.insert("state".to_string(), json!(format!("{}", task.state)));
            props.insert("progress_percentage".to_string(), json!(task.progress.percentage));
            props.insert("elapsed_seconds".to_string(), json!(task.progress.elapsed_seconds));
            props.insert("total_seconds".to_string(), json!(task.progress.total_seconds));
            props.insert("estimated_completion".to_string(), json!(task.estimated_completion));
            props.insert("priority".to_string(), json!(format!("{}", task.priority)));
            props.insert("failure_count".to_string(), json!(task.failure_count));
            
            // Merge custom props
            for (key, value) in &task.props {
                props.insert(key.clone(), value.clone());
            }
            
            props
        },
        geometry: None,
    }
}

/// Create shipment node (PR #8 - architecture@c9 - Drones in flight)
/// Shipments are physical logistics units that move across the globe
fn create_shipment_node(shipment: &Shipment) -> Node {
    // Material based on state
    let material = match shipment.state {
        ShipmentState::Created => "shipment_created",
        ShipmentState::InTransit => "shipment_in_transit",
        ShipmentState::Arrived => "shipment_arrived",
        ShipmentState::Failed => "shipment_failed",
    };
    
    // Position is the current location of the drone
    let position = [
        shipment.current_position.lat as f32,
        shipment.current_position.lon as f32,
        shipment.current_position.alt as f32,
    ];
    
    // Calculate heading (rotation) based on route direction
    // If we have route points, point drone toward next waypoint
    let rotation = if shipment.route.len() >= 2 {
        let current = &shipment.current_position;
        // Find next waypoint based on progress
        let next_idx = ((shipment.progress_percentage * (shipment.route.len() - 1) as f32).ceil() as usize).min(shipment.route.len() - 1);
        let next = &shipment.route[next_idx];
        
        // Calculate bearing (simplified - just use lon difference for yaw)
        let delta_lon = next.lon - current.lon;
        let delta_lat = next.lat - current.lat;
        let yaw = delta_lon.atan2(delta_lat).to_degrees() as f32;
        
        [0.0, yaw, 0.0]
    } else {
        [0.0, 0.0, 0.0]
    };
    
    Node {
        id: shipment.shipment_id.as_str().to_string(),
        kind: "shipment".to_string(),
        transform: Transform {
            position,
            rotation,
            scale: [1.0, 1.0, 1.0],
        },
        material: material.to_string(),
        props: {
            let mut props = Map::new();
            props.insert("shipment_id".to_string(), json!(shipment.shipment_id.as_str()));
            props.insert("task_ref".to_string(), json!(shipment.task_ref.as_str()));
            props.insert("origin_building".to_string(), json!(shipment.origin_building.as_str()));
            props.insert("destination_building".to_string(), json!(shipment.destination_building.as_str()));
            props.insert("state".to_string(), json!(format!("{}", shipment.state)));
            props.insert("progress_percentage".to_string(), json!(shipment.progress_percentage));
            props.insert("carrier_type".to_string(), json!(format!("{}", shipment.carrier_type)));
            props.insert("created_at".to_string(), json!(shipment.created_at));
            props.insert("estimated_arrival".to_string(), json!(shipment.estimated_arrival));
            
            if let Some(arrival) = shipment.actual_arrival {
                props.insert("actual_arrival".to_string(), json!(arrival));
            }
            
            // Include route for visualization (polyline)
            props.insert("route".to_string(), json!(shipment.route.iter().map(|pos| {
                vec![pos.lat, pos.lon, pos.alt]
            }).collect::<Vec<_>>()));
            
            // Merge custom props
            for (key, value) in &shipment.props {
                props.insert(key.clone(), value.clone());
            }
            
            props
        },
        // Route visualized as polyline
        geometry: Some(Geometry::Polyline {
            points: shipment.route.iter().map(|pos| {
                [pos.lat as f32, pos.lon as f32, pos.alt as f32]
            }).collect(),
            color: match shipment.state {
                ShipmentState::Created => "#00FFFF".to_string(),      // Cyan
                ShipmentState::InTransit => "#00FF00".to_string(),    // Green
                ShipmentState::Arrived => "#FFFFFF".to_string(),      // White (fading)
                ShipmentState::Failed => "#FF0000".to_string(),       // Red
            },
            width: 2.0,
        }),
    }
}

// --- Deterministic Helpers ---

/// Generate deterministic position from ID (no randomness)
fn deterministic_position_from_id(id: &str) -> [f32; 3] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Map hash to spherical coordinates on globe surface
    let radius = 10.0; // Globe radius
    
    // Use hash bytes to generate deterministic angles
    let theta = ((hash & 0xFFFF) as f32 / 65535.0) * std::f32::consts::PI * 2.0; // 0 to 2π
    let phi = (((hash >> 16) & 0xFFFF) as f32 / 65535.0) * std::f32::consts::PI; // 0 to π
    
    // Convert spherical to Cartesian
    [
        radius * phi.sin() * theta.cos(),
        radius * phi.sin() * theta.sin(),
        radius * phi.cos(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::{CausalRefs, UnitId};

    #[test]
    fn test_generate_world_scene() {
        let units = vec![Unit {
            id: UnitId::new("unit.test.001"),
            state: UnitState::Idle,
            attached_filament: None,
            current_task_filament: None,
        }];
        
        let filaments = HashMap::new();
        let buildings = vec![];
        let tasks = vec![];
        let shipments = vec![];
        
        let spec = generate_world_scene(&units, &filaments, &buildings, &tasks, &shipments);
        
        assert_eq!(spec.schema_version, "relay-render-v1");
        assert!(spec.nodes.len() >= 2); // Globe + unit
        
        // Check globe
        assert_eq!(spec.nodes[0].kind, "globe");
        assert_eq!(spec.nodes[0].material, "world_surface");
        
        // Check unit (now at index 1 since no buildings)
        assert_eq!(spec.nodes[1].kind, "unit");
        assert_eq!(spec.nodes[1].material, "scv_idle");
    }

    #[test]
    fn test_deterministic_position() {
        let pos1 = deterministic_position_from_id("unit.alice.001");
        let pos2 = deterministic_position_from_id("unit.alice.001");
        let pos3 = deterministic_position_from_id("unit.bob.002");
        
        // Same ID → same position
        assert_eq!(pos1, pos2);
        
        // Different ID → different position
        assert_ne!(pos1, pos3);
    }
}

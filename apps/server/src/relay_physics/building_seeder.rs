// apps/server/src/relay_physics/building_seeder.rs
//
// Building Seeder - Load buildings from seed file (PR #6)
// Reference: architecture@c9

use crate::relay_physics::{Building, BuildingStore, EventBus, EventLog, RelayEvent};
use std::sync::{Arc, Mutex};

/// Seed buildings from fixture file if store is empty
/// 
/// This writes each building as a BUILDING_REGISTERED event to ensure
/// deterministic replay (PR #1.1 invariant).
pub fn seed_buildings_if_empty(
    building_store: &mut BuildingStore,
    event_log: &Arc<Mutex<EventLog>>,
    event_bus: &EventBus,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if buildings already exist (skip if already seeded)
    if !building_store.is_empty() {
        return Ok(());
    }
    
    // Load seed file
    let seed_path = "relay/fixtures/buildings_seed.json";
    let seed_data = std::fs::read_to_string(seed_path)?;
    let buildings: Vec<Building> = serde_json::from_str(&seed_data)?;
    
    println!("🏗️  Seeding {} buildings from {}", buildings.len(), seed_path);
    
    // Write each building as BUILDING_REGISTERED event
    for building in buildings {
        let event = RelayEvent::BuildingRegistered {
            building: building.clone(),
        };
        
        // Persist to event log (source of truth)
        let mut log = event_log.lock().unwrap();
        let event_id = log.append(event.clone())?;
        drop(log); // Release lock before emitting
        
        // Emit to event bus (live stream)
        event_bus.emit_with_id(event_id, event);
        
        // Apply to building store
        building_store.register(building)?;
    }
    
    println!("✅ Buildings seeded successfully");
    
    Ok(())
}

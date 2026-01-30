// apps/server/src/relay_physics/building_store.rs
//
// Building Store - Physical world entities (PR #6)
// Reference: architecture@c9

use crate::relay_physics::{Building, BuildingId, RelayEvent};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BuildingStore {
    buildings: HashMap<BuildingId, Building>,
}

impl BuildingStore {
    pub fn new() -> Self {
        Self {
            buildings: HashMap::new(),
        }
    }
    
    /// Check if store is empty (for seeding check)
    pub fn is_empty(&self) -> bool {
        self.buildings.is_empty()
    }
    
    /// Register a new building
    pub fn register(&mut self, building: Building) -> Result<(), String> {
        if self.buildings.contains_key(&building.building_id) {
            return Err(format!("Building {} already exists", building.building_id));
        }
        
        self.buildings.insert(building.building_id.clone(), building);
        Ok(())
    }
    
    /// Update existing building
    pub fn update(&mut self, building_id: &BuildingId, building: Building) -> Result<(), String> {
        if !self.buildings.contains_key(building_id) {
            return Err(format!("Building {} not found", building_id));
        }
        
        self.buildings.insert(building_id.clone(), building);
        Ok(())
    }
    
    /// Get building by ID
    pub fn get(&self, building_id: &BuildingId) -> Option<&Building> {
        self.buildings.get(building_id)
    }
    
    /// List all buildings
    pub fn list_all(&self) -> Vec<Building> {
        self.buildings.values().cloned().collect()
    }
    
    /// Replay events to reconstruct state (PR #1.1 determinism)
    pub fn replay_from_events(&mut self, events: &[crate::relay_physics::EventLogEntry]) {
        for entry in events {
            match &entry.event {
                RelayEvent::BuildingRegistered { building } => {
                    let _ = self.register(building.clone());
                }
                RelayEvent::BuildingUpdated { building_id, building } => {
                    let _ = self.update(building_id, building.clone());
                }
                _ => {
                    // Ignore non-building events
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::{BuildingType, BuildingStatus, GeoAnchor};

    #[test]
    fn test_register_building() {
        let mut store = BuildingStore::new();
        
        let building = Building {
            building_id: BuildingId::new("building.test.001"),
            building_type: BuildingType::Vendor,
            geo_anchor: GeoAnchor {
                lat: 40.7637,
                lng: -73.9722,
                altitude: Some(10.0),
            },
            owner_ref: None,
            catalog: vec![],
            status: BuildingStatus::Active,
            props: serde_json::Map::new(),
        };
        
        assert!(store.register(building).is_ok());
        assert_eq!(store.list_all().len(), 1);
    }

    #[test]
    fn test_register_duplicate_building() {
        let mut store = BuildingStore::new();
        
        let building1 = Building {
            building_id: BuildingId::new("building.test.001"),
            building_type: BuildingType::Vendor,
            geo_anchor: GeoAnchor {
                lat: 40.7637,
                lng: -73.9722,
                altitude: Some(10.0),
            },
            owner_ref: None,
            catalog: vec![],
            status: BuildingStatus::Active,
            props: serde_json::Map::new(),
        };
        
        let building2 = building1.clone();
        
        assert!(store.register(building1).is_ok());
        assert!(store.register(building2).is_err());
    }
}

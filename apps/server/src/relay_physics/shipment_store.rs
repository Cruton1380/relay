// apps/server/src/relay_physics/shipment_store.rs
// PR #8: Shipment Store (Drone Layer) - architecture@c9 + c10
//
// Shipments are physical logistics units (drones) that move across the globe.
// This store manages their state, position, and lifecycle.

use std::collections::HashMap;
use crate::relay_physics::{
    Shipment, ShipmentId, ShipmentState, GeoPosition, TaskId, BuildingId,
    RelayEvent, EventLogEntry,
};

/// ShipmentStore manages the state of all shipments (drones in flight)
pub struct ShipmentStore {
    shipments: HashMap<ShipmentId, Shipment>,
}

impl ShipmentStore {
    /// Create a new empty ShipmentStore
    pub fn new() -> Self {
        Self {
            shipments: HashMap::new(),
        }
    }
    
    // ========================================================================
    // CRUD OPERATIONS
    // ========================================================================
    
    /// Create a new shipment (spawn drone)
    pub fn create(&mut self, shipment: Shipment) -> Result<(), String> {
        if self.shipments.contains_key(&shipment.shipment_id) {
            return Err(format!("Shipment {} already exists", shipment.shipment_id));
        }
        
        self.shipments.insert(shipment.shipment_id.clone(), shipment);
        Ok(())
    }
    
    /// Get a shipment by ID
    pub fn get(&self, id: &ShipmentId) -> Option<&Shipment> {
        self.shipments.get(id)
    }
    
    /// Get a mutable shipment by ID
    pub fn get_mut(&mut self, id: &ShipmentId) -> Option<&mut Shipment> {
        self.shipments.get_mut(id)
    }
    
    /// Update shipment position (calculated on-demand)
    pub fn update_position(
        &mut self,
        id: &ShipmentId,
        position: GeoPosition,
        progress: f32,
    ) -> Result<(), String> {
        let shipment = self.shipments.get_mut(id)
            .ok_or_else(|| format!("Shipment {} not found", id))?;
        
        shipment.current_position = position;
        shipment.progress_percentage = progress;
        Ok(())
    }
    
    /// Mark shipment as arrived (drone landed)
    pub fn arrive(&mut self, id: &ShipmentId, arrival_time: u64) -> Result<(), String> {
        let shipment = self.shipments.get_mut(id)
            .ok_or_else(|| format!("Shipment {} not found", id))?;
        
        // Validate state transition
        if !Self::is_valid_transition(shipment.state, ShipmentState::Arrived) {
            return Err(format!(
                "Invalid transition: {} -> Arrived",
                shipment.state
            ));
        }
        
        shipment.state = ShipmentState::Arrived;
        shipment.actual_arrival = Some(arrival_time);
        shipment.progress_percentage = 1.0;
        
        // Set position to destination
        if let Some(dest_pos) = shipment.route.last() {
            shipment.current_position = dest_pos.clone();
        }
        
        Ok(())
    }
    
    /// Mark shipment as failed
    pub fn fail(&mut self, id: &ShipmentId, reason: String) -> Result<(), String> {
        let shipment = self.shipments.get_mut(id)
            .ok_or_else(|| format!("Shipment {} not found", id))?;
        
        // Validate state transition
        if !Self::is_valid_transition(shipment.state, ShipmentState::Failed) {
            return Err(format!(
                "Invalid transition: {} -> Failed",
                shipment.state
            ));
        }
        
        shipment.state = ShipmentState::Failed;
        shipment.props.insert(
            "failure_reason".to_string(),
            serde_json::Value::String(reason),
        );
        
        Ok(())
    }
    
    /// Progress shipment to InTransit state
    pub fn start_transit(&mut self, id: &ShipmentId) -> Result<(), String> {
        let shipment = self.shipments.get_mut(id)
            .ok_or_else(|| format!("Shipment {} not found", id))?;
        
        if !Self::is_valid_transition(shipment.state, ShipmentState::InTransit) {
            return Err(format!(
                "Invalid transition: {} -> InTransit",
                shipment.state
            ));
        }
        
        shipment.state = ShipmentState::InTransit;
        Ok(())
    }
    
    // ========================================================================
    // QUERY OPERATIONS
    // ========================================================================
    
    /// List all shipments
    pub fn list_all(&self) -> Vec<Shipment> {
        self.shipments.values().cloned().collect()
    }
    
    /// List shipments by task
    pub fn list_by_task(&self, task_id: &TaskId) -> Vec<Shipment> {
        self.shipments
            .values()
            .filter(|s| &s.task_ref == task_id)
            .cloned()
            .collect()
    }
    
    /// List shipments by origin building
    pub fn list_by_origin(&self, building_id: &BuildingId) -> Vec<Shipment> {
        self.shipments
            .values()
            .filter(|s| &s.origin_building == building_id)
            .cloned()
            .collect()
    }
    
    /// List shipments by destination building
    pub fn list_by_destination(&self, building_id: &BuildingId) -> Vec<Shipment> {
        self.shipments
            .values()
            .filter(|s| &s.destination_building == building_id)
            .cloned()
            .collect()
    }
    
    /// List shipments by state
    pub fn list_by_state(&self, state: ShipmentState) -> Vec<Shipment> {
        self.shipments
            .values()
            .filter(|s| s.state == state)
            .cloned()
            .collect()
    }
    
    /// List active shipments (Created or InTransit)
    pub fn list_active(&self) -> Vec<Shipment> {
        self.shipments
            .values()
            .filter(|s| {
                s.state == ShipmentState::Created || s.state == ShipmentState::InTransit
            })
            .cloned()
            .collect()
    }
    
    /// Get shipment count
    pub fn count(&self) -> usize {
        self.shipments.len()
    }
    
    /// Check if store is empty
    pub fn is_empty(&self) -> bool {
        self.shipments.is_empty()
    }
    
    // ========================================================================
    // POSITION CALCULATION
    // ========================================================================
    
    /// Calculate current position of shipment based on time elapsed
    /// Returns None if shipment doesn't exist
    pub fn calculate_current_position(
        &self,
        id: &ShipmentId,
        current_time: u64,
    ) -> Option<GeoPosition> {
        let shipment = self.shipments.get(id)?;
        Some(Self::interpolate_position(shipment, current_time))
    }
    
    /// Interpolate position along route based on time
    pub fn interpolate_position(shipment: &Shipment, current_time: u64) -> GeoPosition {
        // If arrived or failed, return current position
        if shipment.state == ShipmentState::Arrived || shipment.state == ShipmentState::Failed {
            return shipment.current_position.clone();
        }
        
        // Calculate time-based progress
        let elapsed = current_time.saturating_sub(shipment.created_at);
        let total_duration = shipment.estimated_arrival.saturating_sub(shipment.created_at);
        
        if total_duration == 0 {
            // No movement yet
            return shipment.route.first()
                .cloned()
                .unwrap_or_else(|| shipment.current_position.clone());
        }
        
        let progress = (elapsed as f64 / total_duration as f64).min(1.0);
        
        // Interpolate along route
        let route = &shipment.route;
        if route.len() < 2 {
            return route.first()
                .cloned()
                .unwrap_or_else(|| shipment.current_position.clone());
        }
        
        let total_segments = route.len() - 1;
        let segment_progress = progress * (total_segments as f64);
        let segment_index = segment_progress.floor() as usize;
        let within_segment = segment_progress - segment_index as f64;
        
        if segment_index >= total_segments {
            // Reached end
            return route.last().unwrap().clone();
        }
        
        let start = &route[segment_index];
        let end = &route[segment_index + 1];
        
        GeoPosition {
            lat: start.lat + (end.lat - start.lat) * within_segment,
            lon: start.lon + (end.lon - start.lon) * within_segment,
            alt: start.alt + (end.alt - start.alt) * within_segment,
        }
    }
    
    // ========================================================================
    // STATE VALIDATION
    // ========================================================================
    
    /// Check if state transition is valid
    fn is_valid_transition(from: ShipmentState, to: ShipmentState) -> bool {
        match (from, to) {
            // From Created
            (ShipmentState::Created, ShipmentState::InTransit) => true,
            (ShipmentState::Created, ShipmentState::Failed) => true,
            
            // From InTransit
            (ShipmentState::InTransit, ShipmentState::Arrived) => true,
            (ShipmentState::InTransit, ShipmentState::Failed) => true,
            
            // No transitions from terminal states
            (ShipmentState::Arrived, _) => false,
            (ShipmentState::Failed, _) => false,
            
            // Same state is OK (idempotent)
            (s1, s2) if s1 == s2 => true,
            
            _ => false,
        }
    }
    
    // ========================================================================
    // EVENT REPLAY (DETERMINISM)
    // ========================================================================
    
    /// Replay events to reconstruct shipment state
    pub fn replay_from_events(&mut self, events: &[EventLogEntry]) {
        for entry in events {
            match &entry.event {
                RelayEvent::ShipmentCreated { shipment, .. } => {
                    // Ignore errors during replay (shipment might already exist)
                    let _ = self.create(shipment.clone());
                }
                
                RelayEvent::ShipmentPositionUpdated { shipment_id, new_position, progress_percentage } => {
                    let _ = self.update_position(
                        shipment_id,
                        new_position.clone(),
                        *progress_percentage,
                    );
                }
                
                RelayEvent::ShipmentArrived { shipment_id, actual_arrival } => {
                    let _ = self.arrive(shipment_id, *actual_arrival);
                }
                
                RelayEvent::ShipmentFailed { shipment_id, reason } => {
                    let _ = self.fail(shipment_id, reason.clone());
                }
                
                _ => {
                    // Ignore non-shipment events
                }
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::{ShipmentId, TaskId, BuildingId, CarrierType};
    
    fn create_test_shipment() -> Shipment {
        Shipment {
            shipment_id: ShipmentId::new("shipment.drone.001"),
            task_ref: TaskId::new("task.test.001"),
            origin_building: BuildingId::new("building.origin.001"),
            destination_building: BuildingId::new("building.dest.001"),
            current_position: GeoPosition::new(40.7128, -74.0060, 0.0),
            route: vec![
                GeoPosition::new(40.7128, -74.0060, 100.0), // Start (NYC)
                GeoPosition::new(40.7589, -73.9851, 100.0), // End (Times Square)
            ],
            state: ShipmentState::Created,
            progress_percentage: 0.0,
            created_at: 1000,
            estimated_arrival: 2000,
            actual_arrival: None,
            carrier_type: CarrierType::Drone,
            props: serde_json::Map::new(),
        }
    }
    
    #[test]
    fn test_create_shipment() {
        let mut store = ShipmentStore::new();
        let shipment = create_test_shipment();
        
        assert!(store.create(shipment.clone()).is_ok());
        assert_eq!(store.count(), 1);
        
        let retrieved = store.get(&shipment.shipment_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().shipment_id, shipment.shipment_id);
    }
    
    #[test]
    fn test_position_interpolation() {
        let mut store = ShipmentStore::new();
        let shipment = create_test_shipment();
        let id = shipment.shipment_id.clone();
        
        store.create(shipment).unwrap();
        
        // At start (t=1000)
        let pos = store.calculate_current_position(&id, 1000).unwrap();
        assert!((pos.lat - 40.7128).abs() < 0.0001);
        assert!((pos.lon - (-74.0060)).abs() < 0.0001);
        
        // At 50% (t=1500)
        let pos = store.calculate_current_position(&id, 1500).unwrap();
        let expected_lat = 40.7128 + (40.7589 - 40.7128) * 0.5;
        let expected_lon = -74.0060 + (-73.9851 - (-74.0060)) * 0.5;
        assert!((pos.lat - expected_lat).abs() < 0.0001);
        assert!((pos.lon - expected_lon).abs() < 0.0001);
        
        // At end (t=2000)
        let pos = store.calculate_current_position(&id, 2000).unwrap();
        assert!((pos.lat - 40.7589).abs() < 0.0001);
        assert!((pos.lon - (-73.9851)).abs() < 0.0001);
        
        // After end (t=3000) - should clamp to destination
        let pos = store.calculate_current_position(&id, 3000).unwrap();
        assert!((pos.lat - 40.7589).abs() < 0.0001);
        assert!((pos.lon - (-73.9851)).abs() < 0.0001);
    }
    
    #[test]
    fn test_shipment_lifecycle() {
        let mut store = ShipmentStore::new();
        let shipment = create_test_shipment();
        let id = shipment.shipment_id.clone();
        
        store.create(shipment).unwrap();
        
        // Created -> InTransit
        assert!(store.start_transit(&id).is_ok());
        assert_eq!(store.get(&id).unwrap().state, ShipmentState::InTransit);
        
        // InTransit -> Arrived
        assert!(store.arrive(&id, 2000).is_ok());
        assert_eq!(store.get(&id).unwrap().state, ShipmentState::Arrived);
        assert_eq!(store.get(&id).unwrap().actual_arrival, Some(2000));
        
        // Arrived -> InTransit (INVALID)
        assert!(store.start_transit(&id).is_err());
    }
    
    #[test]
    fn test_invalid_state_transition() {
        let mut store = ShipmentStore::new();
        let shipment = create_test_shipment();
        let id = shipment.shipment_id.clone();
        
        store.create(shipment).unwrap();
        store.arrive(&id, 2000).unwrap();
        
        // Can't go back from Arrived
        assert!(store.start_transit(&id).is_err());
        assert!(store.fail(&id, "test".to_string()).is_err());
    }
    
    #[test]
    fn test_list_by_state() {
        let mut store = ShipmentStore::new();
        
        let mut shipment1 = create_test_shipment();
        shipment1.shipment_id = ShipmentId::new("shipment.drone.001");
        shipment1.state = ShipmentState::InTransit;
        
        let mut shipment2 = create_test_shipment();
        shipment2.shipment_id = ShipmentId::new("shipment.drone.002");
        shipment2.state = ShipmentState::InTransit;
        
        let mut shipment3 = create_test_shipment();
        shipment3.shipment_id = ShipmentId::new("shipment.drone.003");
        shipment3.state = ShipmentState::Arrived;
        
        store.create(shipment1).unwrap();
        store.create(shipment2).unwrap();
        store.create(shipment3).unwrap();
        
        let in_transit = store.list_by_state(ShipmentState::InTransit);
        assert_eq!(in_transit.len(), 2);
        
        let arrived = store.list_by_state(ShipmentState::Arrived);
        assert_eq!(arrived.len(), 1);
        
        let active = store.list_active();
        assert_eq!(active.len(), 2); // Only InTransit (not Arrived)
    }
}

use serde::{Deserialize, Serialize};

/// Filament identifier (e.g., "conversation.ai.001", "work.W123")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FilamentId(pub String);

impl FilamentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FilamentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unit identifier (e.g., "unit.ai.agent.001", "unit.manager.001")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitId(pub String);

impl UnitId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Commit index (monotonic per filament)
pub type CommitIndex = u64;

/// Commit reference (e.g., "work.W123@c42")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitRef(pub String);

impl CommitRef {
    pub fn new(filament_id: &FilamentId, commit_index: CommitIndex) -> Self {
        Self(format!("{}@c{}", filament_id.as_str(), commit_index))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Authority reference
pub type AuthorityRef = String;

/// Evidence reference
pub type EvidenceRef = String;

/// Causal references (inputs, evidence, authority)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalRefs {
    #[serde(default)]
    pub inputs: Vec<CommitRef>,
    
    #[serde(default)]
    pub evidence: Vec<EvidenceRef>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_ref: Option<AuthorityRef>,
}

impl Default for CausalRefs {
    fn default() -> Self {
        Self {
            inputs: Vec::new(),
            evidence: Vec::new(),
            authority_ref: None,
        }
    }
}

/// SCV unit states (LOCKED)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitState {
    Idle,
    Moving,
    Working,
    Blocked,
    AwaitingAuthority,
}

impl std::fmt::Display for UnitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitState::Idle => write!(f, "Idle"),
            UnitState::Moving => write!(f, "Moving"),
            UnitState::Working => write!(f, "Working"),
            UnitState::Blocked => write!(f, "Blocked"),
            UnitState::AwaitingAuthority => write!(f, "AwaitingAuthority"),
        }
    }
}

// ============================================================================
// BUILDINGS (PR #6) - architecture@c9
// ============================================================================

/// Building identifier (e.g., "building.apple_store.nyc_001")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BuildingId(pub String);

impl BuildingId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for BuildingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Building type (vendor, partnership, civic, logistics, community)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingType {
    Vendor,
    Partnership,
    Civic,
    Logistics,
    Community,
}

impl std::fmt::Display for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingType::Vendor => write!(f, "vendor"),
            BuildingType::Partnership => write!(f, "partnership"),
            BuildingType::Civic => write!(f, "civic"),
            BuildingType::Logistics => write!(f, "logistics"),
            BuildingType::Community => write!(f, "community"),
        }
    }
}

/// Geographic anchor (physical location on globe)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoAnchor {
    pub lat: f64,  // Latitude (-90 to 90)
    pub lng: f64,  // Longitude (-180 to 180)
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>, // Meters above sea level
}

/// Building status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingStatus {
    Active,
    Disabled, // Anti-spoof flag (future)
}

/// Catalog item (product/service in vendor building)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    pub product_id: String,
    pub name: String,
    pub price: f64, // USD (or token amount)
    pub production_time_seconds: u64,
    pub delivery_time_seconds: u64,
    pub in_stock: bool,
    
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Building (physical entity on globe)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub building_id: BuildingId,
    pub building_type: BuildingType,
    pub geo_anchor: GeoAnchor,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_ref: Option<UnitId>,
    
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub catalog: Vec<CatalogItem>,
    
    pub status: BuildingStatus,
    
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub props: serde_json::Map<String, serde_json::Value>,
}

// ============================================================================
// TASKS (PR #7) - architecture@c9 + c10
// ============================================================================

/// Task identifier (e.g., "task.apple_iphone_001")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Task type (production, shipment, verification, commitment)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Production,   // Vendor building produces product
    Shipment,     // Logistics building ships product
    Verification, // Civic building verifies claim
    Commitment,   // Partnership building fulfills commitment
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Production => write!(f, "production"),
            TaskType::Shipment => write!(f, "shipment"),
            TaskType::Verification => write!(f, "verification"),
            TaskType::Commitment => write!(f, "commitment"),
        }
    }
}

/// Task state (lifecycle progression)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    Queued,       // In build queue, not started
    Packing,      // Vendor is preparing product
    Dispatched,   // Package handed off to logistics
    InTransit,    // Drone is flying (becomes shipment)
    Delivered,    // Arrived at destination
    Failed,       // Task failed (scar recorded)
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskState::Queued => write!(f, "queued"),
            TaskState::Packing => write!(f, "packing"),
            TaskState::Dispatched => write!(f, "dispatched"),
            TaskState::InTransit => write!(f, "in_transit"),
            TaskState::Delivered => write!(f, "delivered"),
            TaskState::Failed => write!(f, "failed"),
        }
    }
}

/// Task progress (time-based or step-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<u64>,   // Timestamp when work began
    
    pub elapsed_seconds: u64,      // How long task has been active
    pub total_seconds: u64,        // Estimated total time
    pub percentage: f32,           // 0.0 to 1.0
}

impl Default for TaskProgress {
    fn default() -> Self {
        Self {
            started_at: None,
            elapsed_seconds: 0,
            total_seconds: 0,
            percentage: 0.0,
        }
    }
}

/// Task priority (for attention management - architecture@c10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Background,   // Low priority (can wait)
    Normal,       // Default
    Urgent,       // High priority (attention alert)
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Background => write!(f, "background"),
            TaskPriority::Normal => write!(f, "normal"),
            TaskPriority::Urgent => write!(f, "urgent"),
        }
    }
}

/// Task (unit production order - build queue entry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: TaskId,
    pub task_type: TaskType,
    pub building_ref: BuildingId,              // Where task originates
    pub requester_ref: UnitId,                 // Who requested (for personal HUD)
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ref: Option<String>,           // What's being produced (if vendor task)
    
    pub state: TaskState,
    pub progress: TaskProgress,
    pub estimated_completion: u64,             // Timestamp (seconds since epoch)
    pub priority: TaskPriority,
    pub failure_count: u32,                    // Number of failures (failure budget tracking - architecture@c10)
    
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub props: serde_json::Map<String, serde_json::Value>,
}

// ============================================================================
// SHIPMENTS (PR #8) - architecture@c9 + c10
// Shipments are physical logistics units (drones) that move across the globe
// ============================================================================

/// Shipment identifier (e.g., "shipment.drone.1738103456")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShipmentId(pub String);

impl ShipmentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ShipmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Geographic position (lat/lon/alt)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPosition {
    pub lat: f64,        // Latitude (-90 to 90)
    pub lon: f64,        // Longitude (-180 to 180)
    pub alt: f64,        // Altitude in meters (0 for ground)
}

impl GeoPosition {
    pub fn new(lat: f64, lon: f64, alt: f64) -> Self {
        Self { lat, lon, alt }
    }
}

/// Carrier type (delivery method)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CarrierType {
    Drone,      // Air delivery (default)
    Ground,     // Truck/car delivery
    Air,        // Plane delivery (future)
}

impl std::fmt::Display for CarrierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CarrierType::Drone => write!(f, "drone"),
            CarrierType::Ground => write!(f, "ground"),
            CarrierType::Air => write!(f, "air"),
        }
    }
}

impl Default for CarrierType {
    fn default() -> Self {
        CarrierType::Drone
    }
}

/// Shipment state (lifecycle progression)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShipmentState {
    Created,    // Spawned, not yet moving
    InTransit,  // Currently flying route
    Arrived,    // Landed at destination
    Failed,     // Delivery failed
}

impl std::fmt::Display for ShipmentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipmentState::Created => write!(f, "created"),
            ShipmentState::InTransit => write!(f, "in_transit"),
            ShipmentState::Arrived => write!(f, "arrived"),
            ShipmentState::Failed => write!(f, "failed"),
        }
    }
}

impl Default for ShipmentState {
    fn default() -> Self {
        ShipmentState::Created
    }
}

/// Shipment (physical logistics unit - drone)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub shipment_id: ShipmentId,
    pub task_ref: TaskId,                   // Linked task
    pub origin_building: BuildingId,        // Where drone spawned
    pub destination_building: BuildingId,   // Where drone is going
    
    pub current_position: GeoPosition,      // Current lat/lon/alt (calculated)
    pub route: Vec<GeoPosition>,            // Waypoints (deterministic)
    
    pub state: ShipmentState,
    pub progress_percentage: f32,           // 0.0 to 1.0
    
    pub created_at: u64,                    // Timestamp (spawn time)
    pub estimated_arrival: u64,             // Timestamp (ETA)
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_arrival: Option<u64>,        // Timestamp (actual landing)
    
    pub carrier_type: CarrierType,
    
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub props: serde_json::Map<String, serde_json::Value>,
}

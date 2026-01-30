// apps/server/src/relay_physics/task_store.rs
//
// Task Store - Build Queue Management (PR #7)
// Reference: architecture@c9 (Shopping as Production), architecture@c10 (Ontology)

use crate::relay_physics::{Task, TaskId, TaskState, RelayEvent, UnitId, BuildingId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TaskStore {
    tasks: HashMap<TaskId, Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
    
    /// Check if store is empty
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    
    /// Create a new task
    pub fn create(&mut self, task: Task) -> Result<(), String> {
        if self.tasks.contains_key(&task.task_id) {
            return Err(format!("Task {} already exists", task.task_id));
        }
        
        self.tasks.insert(task.task_id.clone(), task);
        Ok(())
    }
    
    /// Update task progress
    pub fn progress(
        &mut self,
        task_id: &TaskId,
        to_state: TaskState,
        progress_percentage: f32,
    ) -> Result<(), String> {
        let task = self.tasks.get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;
        
        // Validate state transition
        if !is_valid_transition(task.state, to_state) {
            return Err(format!(
                "Invalid state transition from {:?} to {:?}",
                task.state, to_state
            ));
        }
        
        task.state = to_state;
        task.progress.percentage = progress_percentage.min(1.0).max(0.0);
        
        Ok(())
    }
    
    /// Complete task
    pub fn complete(&mut self, task_id: &TaskId) -> Result<(), String> {
        let task = self.tasks.get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;
        
        task.state = TaskState::Delivered;
        task.progress.percentage = 1.0;
        
        Ok(())
    }
    
    /// Fail task
    pub fn fail(&mut self, task_id: &TaskId, reason: String) -> Result<(), String> {
        let task = self.tasks.get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;
        
        task.state = TaskState::Failed;
        task.failure_count += 1;
        task.props.insert("failure_reason".to_string(), serde_json::Value::String(reason));
        
        Ok(())
    }
    
    /// Get task by ID
    pub fn get(&self, task_id: &TaskId) -> Option<&Task> {
        self.tasks.get(task_id)
    }
    
    /// List all tasks
    pub fn list_all(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }
    
    /// List tasks by requester (for personal HUD)
    pub fn list_by_requester(&self, requester_ref: &UnitId) -> Vec<Task> {
        self.tasks.values()
            .filter(|task| task.requester_ref == *requester_ref)
            .cloned()
            .collect()
    }
    
    /// List tasks by building
    pub fn list_by_building(&self, building_ref: &BuildingId) -> Vec<Task> {
        self.tasks.values()
            .filter(|task| task.building_ref == *building_ref)
            .cloned()
            .collect()
    }
    
    /// List tasks by state
    pub fn list_by_state(&self, state: TaskState) -> Vec<Task> {
        self.tasks.values()
            .filter(|task| task.state == state)
            .cloned()
            .collect()
    }
    
    /// Replay events to reconstruct state (PR #1.1 determinism)
    pub fn replay_from_events(&mut self, events: &[crate::relay_physics::EventLogEntry]) {
        for entry in events {
            match &entry.event {
                RelayEvent::TaskCreated { task } => {
                    let _ = self.create(task.clone());
                }
                RelayEvent::TaskProgressed { task_id, to_state, progress_percentage } => {
                    let _ = self.progress(task_id, *to_state, *progress_percentage);
                }
                RelayEvent::TaskCompleted { task_id } => {
                    let _ = self.complete(task_id);
                }
                RelayEvent::TaskFailed { task_id, reason } => {
                    let _ = self.fail(task_id, reason.clone());
                }
                _ => {
                    // Ignore non-task events
                }
            }
        }
    }
}

/// Validate state transition (prevent invalid transitions)
fn is_valid_transition(from: TaskState, to: TaskState) -> bool {
    use TaskState::*;
    
    match (from, to) {
        // Queued can go to Packing or Failed
        (Queued, Packing) => true,
        (Queued, Failed) => true,
        
        // Packing can go to Dispatched or Failed
        (Packing, Dispatched) => true,
        (Packing, Failed) => true,
        
        // Dispatched can go to InTransit or Failed
        (Dispatched, InTransit) => true,
        (Dispatched, Failed) => true,
        
        // InTransit can go to Delivered or Failed
        (InTransit, Delivered) => true,
        (InTransit, Failed) => true,
        
        // Delivered and Failed are terminal
        (Delivered, _) => false,
        (Failed, _) => false,
        
        // Same state (no-op)
        (a, b) if a == b => true,
        
        // All other transitions invalid
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay_physics::{BuildingType, TaskType, TaskPriority, TaskProgress};

    #[test]
    fn test_create_task() {
        let mut store = TaskStore::new();
        
        let task = Task {
            task_id: TaskId::new("task.test.001"),
            task_type: TaskType::Production,
            building_ref: BuildingId::new("building.test.001"),
            requester_ref: UnitId::new("unit.alice.001"),
            product_ref: Some("test_product".to_string()),
            state: TaskState::Queued,
            progress: TaskProgress::default(),
            estimated_completion: 1706464800,
            priority: TaskPriority::Normal,
            failure_count: 0,
            props: serde_json::Map::new(),
        };
        
        assert!(store.create(task).is_ok());
        assert_eq!(store.list_all().len(), 1);
    }

    #[test]
    fn test_progress_task() {
        let mut store = TaskStore::new();
        
        let task = Task {
            task_id: TaskId::new("task.test.001"),
            task_type: TaskType::Production,
            building_ref: BuildingId::new("building.test.001"),
            requester_ref: UnitId::new("unit.alice.001"),
            product_ref: Some("test_product".to_string()),
            state: TaskState::Queued,
            progress: TaskProgress::default(),
            estimated_completion: 1706464800,
            priority: TaskPriority::Normal,
            failure_count: 0,
            props: serde_json::Map::new(),
        };
        
        let task_id = task.task_id.clone();
        store.create(task).unwrap();
        
        // Progress to packing
        store.progress(&task_id, TaskState::Packing, 0.5).unwrap();
        
        let updated = store.get(&task_id).unwrap();
        assert_eq!(updated.state, TaskState::Packing);
        assert_eq!(updated.progress.percentage, 0.5);
    }

    #[test]
    fn test_invalid_state_transition() {
        let mut store = TaskStore::new();
        
        let task = Task {
            task_id: TaskId::new("task.test.001"),
            task_type: TaskType::Production,
            building_ref: BuildingId::new("building.test.001"),
            requester_ref: UnitId::new("unit.alice.001"),
            product_ref: Some("test_product".to_string()),
            state: TaskState::Queued,
            progress: TaskProgress::default(),
            estimated_completion: 1706464800,
            priority: TaskPriority::Normal,
            failure_count: 0,
            props: serde_json::Map::new(),
        };
        
        let task_id = task.task_id.clone();
        store.create(task).unwrap();
        
        // Try invalid transition (Queued → InTransit skips Packing)
        assert!(store.progress(&task_id, TaskState::InTransit, 0.5).is_err());
    }
    
    #[test]
    fn test_list_by_requester() {
        let mut store = TaskStore::new();
        
        let task1 = Task {
            task_id: TaskId::new("task.test.001"),
            task_type: TaskType::Production,
            building_ref: BuildingId::new("building.test.001"),
            requester_ref: UnitId::new("unit.alice.001"),
            product_ref: Some("product1".to_string()),
            state: TaskState::Queued,
            progress: TaskProgress::default(),
            estimated_completion: 1706464800,
            priority: TaskPriority::Normal,
            failure_count: 0,
            props: serde_json::Map::new(),
        };
        
        let task2 = Task {
            task_id: TaskId::new("task.test.002"),
            task_type: TaskType::Production,
            building_ref: BuildingId::new("building.test.001"),
            requester_ref: UnitId::new("unit.bob.002"),
            product_ref: Some("product2".to_string()),
            state: TaskState::Queued,
            progress: TaskProgress::default(),
            estimated_completion: 1706464800,
            priority: TaskPriority::Normal,
            failure_count: 0,
            props: serde_json::Map::new(),
        };
        
        store.create(task1).unwrap();
        store.create(task2).unwrap();
        
        // List Alice's tasks
        let alice_tasks = store.list_by_requester(&UnitId::new("unit.alice.001"));
        assert_eq!(alice_tasks.len(), 1);
        assert_eq!(alice_tasks[0].product_ref, Some("product1".to_string()));
    }
}

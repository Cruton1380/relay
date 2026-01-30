# RELAY DEVELOPER GUIDE

**Purpose:** Implementation standards and practices  
**Audience:** Developers  
**Time:** Half day (initial), reference thereafter  
**Status:** 🔒 Locked

---

## 🎯 CRITICAL UNDERSTANDING FOR DEVELOPERS

### **Relay Is Physics, Not Features**

**You are not building:**
- ❌ A social network
- ❌ A dashboard
- ❌ A blockchain
- ❌ A game

**You are implementing:**
- ✅ Conservation laws (filaments)
- ✅ Causality (commits + refs)
- ✅ Determinism (replay)
- ✅ Auditability (evidence chains)

**If you violate physics, you break Relay.**

---

## 🔒 NON-NEGOTIABLE INVARIANTS

### **Before Writing Any Code:**

**1. No Mutations**
- Filaments extend, never mutate
- State is derived, not stored
- History is append-only

**2. No Randomness**
- IDs are deterministic (hash-based or filament-derived)
- Positions are calculated (not random)
- Same inputs → same outputs (always)

**3. No Hidden Operations**
- Every action creates a commit
- No silent updates
- No background processes that don't log

**4. No Global State**
- No singletons (except truly universal)
- No global scores
- No universal rankings

**5. Refs Are Truth**
- `commit_ref` is authoritative
- `filament_ref` is stable
- `causal_refs` are complete

---

## 📐 ARCHITECTURE DEPENDENCIES

### **You Must Understand These First:**

**Foundation (Required):**
- [c0.Filaments] - Append-only, conserved, immutable
- [c1.Commits] - Execution, not tooling
- [c2.Replayability] - Deterministic state derivation

**Layer Separation (Required):**
- [c3.Layers] - Layer 2 (truth) vs Layer 3 (rendering)
- [c5.RenderSpec] - Deterministic rendering contract

**Read these before implementing ANY feature.**

**Domain-Specific:**
- Tasks → [c7.Gauges], [c9.StarCraft], PR#7
- Identity → [c10.Ontology], PR#9
- Zones → [c13.RuleBasedZones]
- Audit → [c16.UniversalAudit]

---

## 🛠️ IMPLEMENTATION STANDARDS

### **1. Filament Operations**

**✅ CORRECT:**
```rust
// Append-only
pub fn append_commit(
    &mut self,
    filament_id: &str,
    payload: serde_json::Value,
    causal_refs: Vec<CommitRef>
) -> Result<CommitRef> {
    let commit_index = self.next_index(filament_id);
    let commit_ref = CommitRef {
        filament_id: filament_id.to_string(),
        commit_index
    };
    
    let commit = Commit {
        commit_ref: commit_ref.clone(),
        payload,
        causal_refs,
        timestamp: current_timestamp(),
        author: current_author()
    };
    
    self.write_to_log(filament_id, &commit)?;
    Ok(commit_ref)
}
```

**❌ WRONG:**
```rust
// Mutation (FORBIDDEN)
pub fn update_filament(&mut self, filament_id: &str, new_value: Value) {
    self.filaments.get_mut(filament_id).unwrap().value = new_value;
    // This violates c0 - no mutations allowed
}
```

---

### **2. State Derivation**

**✅ CORRECT:**
```rust
// Derive state from commits
pub fn derive_state(
    &self,
    filament_id: &str
) -> Result<DerivedState> {
    let commits = self.load_commits(filament_id)?;
    let mut state = DerivedState::default();
    
    for commit in commits {
        state = apply_commit(state, commit)?;
    }
    
    Ok(state)
}

fn apply_commit(
    mut state: DerivedState,
    commit: Commit
) -> Result<DerivedState> {
    // Deterministic application
    match commit.payload {
        // ... handle each commit type
    }
    Ok(state)
}
```

**❌ WRONG:**
```rust
// Storing derived state as source (FORBIDDEN)
pub fn save_state(&mut self, state: State) {
    self.db.write("current_state", &state);
    // State must be derived, not stored
}
```

---

### **3. Commit References**

**✅ CORRECT:**
```rust
// Always include causal_refs
let task_commit = append_commit(
    "task.budget_q1",
    json!({
        "type": "TASK_CREATED",
        "task_type": "BudgetApproval",
        "assignee": "alice"
    }),
    vec![
        CommitRef::parse("identity.alice@c142")?,  // Alice authorized
        CommitRef::parse("building.acme_hq@c15")?  // At ACME HQ
    ]
)?;
```

**❌ WRONG:**
```rust
// Missing causal_refs (FORBIDDEN)
let task_commit = append_commit(
    "task.budget_q1",
    json!({"type": "TASK_CREATED"}),
    vec![]  // Empty causal_refs = untraceable
)?;
```

---

### **4. Deterministic IDs**

**✅ CORRECT:**
```rust
use sha2::{Sha256, Digest};

// Hash-based deterministic ID
pub fn deterministic_id(inputs: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for input in inputs {
        hasher.update(input.as_bytes());
    }
    let result = hasher.finalize();
    format!("id_{:x}", result)
}

// Usage
let building_id = deterministic_id(&[
    "building",
    "vendor",
    "apple_store",
    "40.7580", // lat
    "-73.9855"  // lon
]);
```

**❌ WRONG:**
```rust
use uuid::Uuid;

// Random ID (FORBIDDEN in Relay)
let building_id = Uuid::new_v4().to_string();
// Same building will get different ID on replay = broken determinism
```

---

### **5. RenderSpec Generation**

**✅ CORRECT:**
```rust
// Deterministic rendering
pub fn generate_building_node(building: &Building) -> RenderNode {
    RenderNode {
        id: building.building_id.clone(),  // Stable ID
        kind: "building".to_string(),
        transform: Transform {
            position: calculate_position(&building.geo_anchor),  // Deterministic
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0]
        },
        material: "building_vendor".to_string(),  // Semantic tag
        props: json!({
            "building_type": building.building_type,
            "status": building.status
        })
    }
}

fn calculate_position(geo: &GeoAnchor) -> [f64; 3] {
    // Deterministic mercator projection
    let x = (geo.lon + 180.0) / 360.0;
    let y = (180.0 - (geo.lat + 90.0)) / 180.0;
    [x, y, geo.altitude_meters.unwrap_or(0.0)]
}
```

**❌ WRONG:**
```rust
// Non-deterministic rendering (FORBIDDEN)
pub fn generate_building_node(building: &Building) -> RenderNode {
    RenderNode {
        id: Uuid::new_v4().to_string(),  // Random = broken
        transform: Transform {
            position: [rand::random(), rand::random(), 0.0],  // Random = broken
            // ...
        },
        material: "#FF5733".to_string(),  // RGB = forbidden, use semantic tags
        // ...
    }
}
```

---

## 🧪 TESTING REQUIREMENTS

### **Every Feature Must Have:**

**1. Determinism Tests**
```rust
#[test]
fn test_deterministic_replay() {
    let mut store1 = Store::new();
    let mut store2 = Store::new();
    
    // Apply same commits to both
    let commits = vec![/* ... */];
    for commit in &commits {
        store1.append(commit.clone()).unwrap();
        store2.append(commit.clone()).unwrap();
    }
    
    // Derived state must be identical
    assert_eq!(
        store1.derive_state("test"),
        store2.derive_state("test")
    );
}
```

**2. Immutability Tests**
```rust
#[test]
fn test_commits_immutable() {
    let store = Store::new();
    let commit_ref = store.append(/* ... */).unwrap();
    
    let commit1 = store.get_commit(&commit_ref).unwrap();
    let commit2 = store.get_commit(&commit_ref).unwrap();
    
    // Must be identical (not cloned with changes)
    assert_eq!(commit1, commit2);
}
```

**3. Causal Integrity Tests**
```rust
#[test]
fn test_causal_refs_required() {
    let store = Store::new();
    
    // Attempt to create commit without causal_refs
    let result = store.append_commit(
        "test",
        json!({"type": "TEST"}),
        vec![]  // Empty causal_refs
    );
    
    // Should fail or warn (depending on domain)
    // At minimum, must be auditable
}
```

**4. RenderSpec Validation Tests**
```rust
#[test]
fn test_renderspec_valid() {
    let spec = generate_world_scene(/* ... */);
    let result = validate_renderspec(&spec);
    
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_renderspec_forbids_rgb() {
    let spec = json!({
        "schema_version": "relay-render-v1",
        "nodes": [{
            "material": "#FF5733"  // RGB forbidden
        }]
    });
    
    let result = validate_renderspec(&spec);
    assert!(!result.valid);
    assert!(result.errors.iter().any(|e| e.contains("RGB")));
}
```

---

## 🚫 FORBIDDEN PATTERNS

### **These Will Be Rejected in Code Review:**

**1. Silent Mutations**
```rust
// ❌ FORBIDDEN
fn update_building(building: &mut Building, status: Status) {
    building.status = status;  // Silent mutation
}
```

**2. Missing Commit Logs**
```rust
// ❌ FORBIDDEN
fn create_task(task_data: TaskData) {
    tasks.insert(task_data.id, task_data);  // No commit created
}
```

**3. Random IDs**
```rust
// ❌ FORBIDDEN
let id = Uuid::new_v4();
```

**4. Time-Dependent Geometry**
```rust
// ❌ FORBIDDEN
let position = [
    base_x + (current_time() % 100) as f64,  // Changes over time
    base_y,
    0.0
];
```

**5. Global Singletons (Except Truly Universal)**
```rust
// ❌ FORBIDDEN (usually)
static mut GLOBAL_STORE: Option<Store> = None;

// ✅ ALLOWED (if truly universal)
static CONFIG: OnceLock<Config> = OnceLock::new();
```

**6. RGB Material Tags**
```rust
// ❌ FORBIDDEN
material: "#FF5733"

// ✅ CORRECT
material: "unit_active"
```

**7. Hidden Defaults**
```rust
// ❌ FORBIDDEN
fn get_status(&self) -> Status {
    self.status.unwrap_or(Status::Active)  // Hidden default
}

// ✅ CORRECT
fn get_status(&self) -> Option<Status> {
    self.status  // Explicit None if unknown
}
```

---

## 📋 CODE REVIEW CHECKLIST

**Before Submitting PR:**

- [ ] All commits have causal_refs
- [ ] No mutations (only appends)
- [ ] No randomness (IDs deterministic)
- [ ] State derived, not stored
- [ ] RenderSpec validation passes
- [ ] Tests cover determinism
- [ ] Tests cover immutability
- [ ] No RGB materials
- [ ] No hidden defaults
- [ ] Architecture refs documented

---

## 🧭 IMPLEMENTATION WORKFLOW

### **Standard Flow:**

**1. Read Architecture**
```
Read relevant architecture commits (e.g., c7 for tasks)
Understand invariants
Check dependencies
```

**2. Read Objects**
```
Check RELAY-OBJECTS-REFERENCE.md
Verify fields
Check invariants
Note governing architecture
```

**3. Check PR Spec**
```
Read implementation/PR-XX-*.md
Follow exact schema
Implement in order
```

**4. Write Types**
```
Define structs
Add field validation
Document invariants
```

**5. Implement Store**
```
Create append-only log
Implement state derivation
Add replay function
```

**6. Add Events**
```
Define event types
Emit on commits
Enable SSE
```

**7. Add API**
```
GET /resource (derived state)
POST /resource/commits (append)
GET /resource/:id/commits (history)
```

**8. Integrate RenderSpec**
```
Add node generator
Validate output
Test determinism
```

**9. Write Tests**
```
Determinism
Immutability
Causal integrity
RenderSpec validation
```

**10. Document**
```
Update RELAY-OBJECTS if needed
Add to PR-ROADMAP
Update CONTEXT-TABLE if major
```

---

## 🔧 DEBUGGING TIPS

### **Common Issues:**

**Issue: "State is wrong after replay"**
- **Cause:** Non-deterministic commit application
- **Fix:** Check for randomness, time dependencies, or ordering issues

**Issue: "RenderSpec validation fails"**
- **Cause:** RGB materials or random IDs
- **Fix:** Use semantic materials, deterministic IDs

**Issue: "Causal chain broken"**
- **Cause:** Missing causal_refs
- **Fix:** Add all dependencies to causal_refs

**Issue: "Tests flaky"**
- **Cause:** Non-determinism in code
- **Fix:** Remove randomness, timestamps in state

---

## 📚 KEY FILES TO REFERENCE

**Always check these:**
1. `architecture/ARCHITECTURE-INDEX.md` - Physics dependencies
2. `reference/RELAY-OBJECTS-REFERENCE.md` - Canonical schemas
3. `implementation/PR-ROADMAP.md` - Implementation order
4. `reference/HOW-TO-READ-RELAY.md` - Reading guide
5. Specific PR spec (e.g., `PR-07-TASK-STORE.md`)

---

## ✅ SUCCESS CRITERIA

**Your implementation is correct when:**
- ✅ Replay produces identical state
- ✅ All commits have causal_refs
- ✅ RenderSpec validation passes
- ✅ No randomness anywhere
- ✅ No mutations
- ✅ Tests pass (including determinism tests)
- ✅ Code review checklist complete

---

## 🎯 QUICK REFERENCE

**When you see:** | **Remember:**
---|---
Filament | Append-only, conserved, immutable
Commit | Execution event with causal_refs
State | Derived by replay, not stored
RenderSpec | Deterministic, semantic materials
ID | Hash-based or filament-derived, never random
Material | Semantic tag (not RGB)
Global | Forbidden (unless truly universal)

---

**Refs:** [c0.Filaments], [c1.Commits], [c2.Replay], [c3.Layers], [c5.RenderSpec], [c16.Audit]  
**Objects:** [Filament], [Commit], [CommitRef], [RenderSpec]  
**Audit:** [Consistency], [Traceability], [Determinism]

**END OF DEVELOPER GUIDE**

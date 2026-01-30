# COMMIT-AT-LOCATION LAW

**The Third Foundational Invariant**

**Type:** Canonical State Creation Rule  
**Status:** 🔒 LOCKED  
**Date:** 2026-01-29  
**Layer:** Foundation

---

## 🔒 THE LAW (CANONICAL STATEMENT)

> **"No state may be created without an explicit location on the filament tree."**

**Corollary:**
> **"Relay does not allow floating state. Everything is created at the location where it belongs, or it does not exist."**

---

## 📊 WHAT THIS MEANS

**In Relay, creation only happens at the source location.**

**There is no valid concept of:**
- "Creating something first and deciding where it belongs later"
- "Save As..." dialogs that forget context
- Temporary files in "nowhere"
- Drafts without anchoring
- Content searching for a home

**This is not UX preference.**  
**This is state-creation physics.**

---

## 🔥 WHY THIS MATTERS (THE BUG IT PREVENTS)

### **The Pattern You Discovered**

**Old world file creation:**
```
1. Create blank file
2. Save "somewhere"
3. Dialog appears (where?)
4. Navigate deep path (P:\customers\group\our customers\avgol\2025\audits\Mocksville\drafts\final\)
5. Can't paste path (Windows Server limitation)
6. Navigate manually
7. "Which file?" (final_v7_REAL_FINAL.docx)
8. No version control
9. Duplicates everywhere
10. Hours lost searching
```

**Result:** Orphaned state, naming chaos, lost work.

---

**Relay file creation:**
```
1. Location exists first (filament node)
2. User selects: audits.avgol.mocksville.2025_q1
3. Commit created AT that location
4. Name = draft_report (commitIndex 100)
5. Lineage automatic
6. History preserved
7. Canon pointer clear
8. No "which file?" confusion
```

**Result:** No orphaned state, clear lineage, zero ambiguity.

---

### **Same Bug, Different Surfaces**

| Domain | Floating State | Anchored State (Relay) |
|--------|---------------|------------------------|
| **Files** | "Save As..." → lost | Created at filament node |
| **Permissions** | Grant → forget close | Expires at commitIndex |
| **Documents** | Draft → final_v7_REAL | Commit with lineage |
| **AI prompts** | Write → forget sync | Created at semantic location |
| **Tickets** | Create → update later | State change at location |

**Every domain suffers from the same failure:**  
**State created without anchoring = orphaned.**

---

## 📋 THE FOUR PRINCIPLES

### **Principle 1: Location Exists First**

**A filament node must exist before anything is written.**

**Examples:**
```
file.audit.avgol.2025.mocksville
authority.sap.firefighter.user123
prompt.image.relay.atomic_filament
belief.history.french_revolution.branch_a
```

**Rule:** If the location doesn't exist, you cannot write.

**Implementation:**
```rust
fn create_commit(location: FilamentRef, payload: Payload) -> Result<Commit> {
    // 1. Verify location exists
    if !filament_exists(location) {
        return Err("INVALID: Location does not exist. Create filament first.");
    }
    
    // 2. Create commit AT that location
    let commit = Commit {
        filament_id: location.filament_id,
        commit_index: get_next_index(location),
        payload,
        // ... other fields
    };
    
    append_to_filament(location, commit)?;
    Ok(commit)
}
```

---

### **Principle 2: Creation Happens at That Node**

**When you create, rename, edit, or propose:**
- The commit is attached to that filament
- Lineage is preserved
- History is conserved
- No ambiguity about "where it lives"
- No intermediate "nowhere" state

**Example commit:**
```json
{
  "commit_id": "commit.abc123",
  "filament_id": "file.audit.avgol.2025.mocksville",
  "commit_index": 100,
  "operation": "CREATE",
  "payload": {
    "filename": "draft_report",
    "content_ref": "blob.def456",
    "author": "user.james"
  },
  "causal_refs": {
    "parent": "commit.abc122"
  }
}
```

**Key: The commit is born at its location. It never "moves."**

---

### **Principle 3: Rename = Commit, Not Relocation**

**You don't "rename a file somewhere else."**

**You:**
1. Make a commit at the same filament
2. Change the name metadata
3. Preserve identity

**This is exactly how Git treats renames—identity persists, labels change.**

**Example:**
```json
{
  "commit_id": "commit.abc101",
  "filament_id": "file.audit.avgol.2025.mocksville",
  "commit_index": 101,
  "operation": "RENAME",
  "payload": {
    "old_name": "draft_report",
    "new_name": "final_report",
    "identity_preserved": true
  },
  "causal_refs": {
    "parent": "commit.abc100"
  }
}
```

**Result:**
- Same filament location
- New name metadata
- History shows rename
- No relocation confusion

---

### **Principle 4: HUD is Convenience, Not Authority**

**You said this perfectly:**
> "It's okay if you want to do it on the HUD for convenience sake."

**Yes—as long as:**
1. ✅ The HUD action resolves to a concrete filament location
2. ✅ The commit is recorded there
3. ✅ The HUD never creates floating state
4. ✅ The HUD cannot invent locations implicitly

**HUD = lens + shortcut, never source of truth.**

---

### **HUD Convenience Examples**

**Valid:**
```
HUD: "Create draft in current context"
  ↓
Resolves to: file.audit.avgol.2025.mocksville
  ↓
Commit created at that filament
  ↓
✅ VALID (location explicit)
```

**Invalid:**
```
HUD: "Create draft"
  ↓
No context resolution
  ↓
File created "somewhere"
  ↓
❌ INVALID (floating state)
```

---

### **HUD Guardrail (Enforcement)**

```rust
fn hud_create_action(context: HUDContext, content: Content) -> Result<Commit> {
    // 1. Resolve HUD context to filament location
    let location = resolve_to_filament(context)?;
    
    if location.is_none() {
        return Err("BLOCKED: No resolved filament location. Select location explicitly.");
    }
    
    // 2. Create commit at resolved location
    create_commit(location.unwrap(), content.into_payload())
}
```

**Rule:** No create without resolved filament.

---

## 🔗 WHY THIS KEEPS THE SYSTEM BALANCED

**Because it eliminates an entire class of bugs:**

| Old World | Relay |
|-----------|-------|
| Save → choose folder later | Choose filament → commit |
| Permissions drift | Authority expires by commitIndex |
| "Which version?" | Canon pointer visible |
| Manual reconciliation | Structural reconciliation |
| Humans as glue | Physics as glue |

**You don't manage feedback loops anymore—you remove the possibility that loops exist.**

---

## 🎯 THE DEEPER UNIFICATION (CRITICAL)

**What you've now locked are three identical laws, expressed in different domains:**

### **Law 1: Reconciliation (Meaning)**
> "No state is real until acknowledged across declared mirrors"

**Applies to:** Semantic state, prompts, specifications

**Refs:** RECONCILIATION-PROTOCOL.md

---

### **Law 2: Authority Expiry (Power)**
> "No power persists without an explicit end"

**Applies to:** Permissions, delegations, authority

**Refs:** AUTHORITY-DELEGATION-COMPLETE.md, c8 (DDI)

---

### **Law 3: Commit-at-Location (Data)** ⭐
> "No state exists without an anchor"

**Applies to:** Files, commits, decisions, content

**Refs:** This document

---

### **They Are the Same Rule**

**Just applied to:**
- Meaning (reconciliation)
- Power (authority)
- Data (location)

**That's why this felt like another "keystone" when you noticed it.**

---

## 📋 ONE SENTENCE (REUSABLE ANYWHERE)

> **"Relay does not allow floating state. Everything is created at the location where it belongs, or it does not exist."**

**This sentence alone explains:**
- ✅ Why Word + SharePoint fails
- ✅ Why SAP firefighter access drifts
- ✅ Why AI chats diverge
- ✅ Why version naming explodes (final_v7_REAL_FINAL.docx)
- ✅ Why automation multiplies complexity
- ✅ Why Relay scales instead of rotting

---

## 🔒 VIOLATIONS (WHAT NOT TO DO)

### **Violation 1: "Save As" Dialog**

**Old pattern:**
```
Create content
  ↓
"Save As..." dialog
  ↓
Navigate to location (forgot where)
  ↓
Save
  ↓
Result: File "somewhere"
```

**Relay equivalent:**
```
Select filament location
  ↓
Create commit at that location
  ↓
No dialog, no navigation
  ↓
Result: Commit anchored
```

---

### **Violation 2: Temporary Files**

**Old pattern:**
```
Create temp file in /tmp or C:\Temp
  ↓
"Move it later"
  ↓
Forgotten
  ↓
Result: Orphaned state
```

**Relay equivalent:**
```
No temporary locations
All commits have permanent filament location
Even drafts/proposals are first-class nodes
```

---

### **Violation 3: Copy-Paste Between Systems**

**Old pattern:**
```
Data in System A
  ↓
Copy to clipboard
  ↓
Paste to System B
  ↓
No reconciliation
  ↓
Result: State diverged
```

**Relay equivalent:**
```
Data in filament
  ↓
Reference created (causal ref)
  ↓
Reconciliation enforced
  ↓
Result: State unified
```

---

## 🧪 IMPLEMENTATION TESTS

### **Test 1: No Floating State**

```rust
#[test]
fn test_no_floating_state() {
    let payload = Payload::new("content");
    
    // Attempt to create without location
    let result = create_commit(None, payload);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "INVALID: Location required");
}
```

---

### **Test 2: Location Must Exist**

```rust
#[test]
fn test_location_must_exist() {
    let nonexistent_location = FilamentRef::new("file.does.not.exist");
    let payload = Payload::new("content");
    
    let result = create_commit(Some(nonexistent_location), payload);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "INVALID: Location does not exist");
}
```

---

### **Test 3: HUD Must Resolve Location**

```rust
#[test]
fn test_hud_must_resolve() {
    let ambiguous_context = HUDContext::new_empty();
    let content = Content::new("draft");
    
    let result = hud_create_action(ambiguous_context, content);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "BLOCKED: No resolved filament location");
}
```

---

### **Test 4: Rename Preserves Location**

```rust
#[test]
fn test_rename_preserves_location() {
    let location = FilamentRef::new("file.audit.avgol");
    let commit_1 = create_commit(location.clone(), Payload::name("draft")).unwrap();
    let commit_2 = rename_commit(location.clone(), "draft", "final").unwrap();
    
    assert_eq!(commit_1.filament_id, commit_2.filament_id);
    assert_eq!(commit_2.operation, Operation::Rename);
}
```

---

## 🌍 REAL-WORLD EXAMPLES

### **Example 1: BDO Audit File**

**Old way (broken):**
```
Path: P:\customers\group\our customers\avgol\2025\audits\Mocksville\drafts\final\

Files:
- draft_v1.docx
- draft_v2.docx
- draft_v3_REAL.docx
- final.docx
- final_v2.docx
- final_v7_REAL_FINAL.docx  ← Which one?

Problems:
- Deep path (navigation pain)
- Can't paste path (Windows limitation)
- No version control
- Naming chaos
- Hours lost
```

**Relay way (fixed):**
```
Filament: audits.avgol.mocksville.2025_q1

Commits:
- commit.100: draft_report (2026-01-15)
- commit.101: draft_report (2026-01-16, updated)
- commit.102: draft_report (2026-01-17, updated)
- commit.103: final_report (2026-01-18, renamed)

Canon: commit.103 (final_report)
History: All commits preserved
Location: Unambiguous
Lineage: Explicit
```

**Result:**
- ✅ No "which file?" confusion
- ✅ Full history visible
- ✅ Canon pointer clear
- ✅ No duplicate naming
- ✅ Zero search time

---

### **Example 2: SAP Firefighter Access**

**Old way (broken):**
```
1. Ticket: "Grant temp access"
2. Admin: Grants in SAP
3. Ticket: "Closed"
4. Reality: Access still active (forgotten)

Problem: State created in SAP, closure "expected" elsewhere
```

**Relay way (fixed):**
```
Filament: authority.sap.firefighter.user123

Commit:
- commit.1000: GRANT (expires at commitIndex 1100)
- Required acks: [system, user, audit, manager]
- Status: HOLD (waiting for acks)
  ↓
All acks received
  ↓
- Status: RECONCILED (active)
  ↓
commitIndex reaches 1100
  ↓
- Status: EXPIRED (automatic)

Problem solved: No separate "closure" step, expiry is deterministic
```

---

### **Example 3: AI Prompt Drift**

**Old way (broken):**
```
User writes prompt in Word
  ↓
Pastes to Claude
  ↓
Claude responds
  ↓
User forgets to update Word
  ↓
State diverged (Word outdated)
```

**Relay way (fixed):**
```
Filament: semantic.state.user.james.project_x

Commit 1: User prompt (Location A: Word, Location B: Claude)
  ↓
Required acks: [word_doc, claude_chat]
  ↓
Status: HOLD (waiting for both)
  ↓
Both acknowledged
  ↓
Status: RECONCILED

No divergence possible (enforced reconciliation)
```

---

## 🔐 INTEGRATION WITH OTHER LAWS

**This law completes the trinity:**

| Law | Domain | Enforcement |
|-----|--------|-------------|
| **Reconciliation** | Meaning | Acknowledgment required |
| **Authority Expiry** | Power | Deterministic expiry |
| **Commit-at-Location** | Data | Location required |

**All three enforce the same principle:**  
**No silent state. No floating state. No assumed state.**

---

## ✅ VERIFICATION CHECKLIST

**For implementers (Claude, Ari, etc.):**

1. ✅ Show that commits cannot be created without filament location
2. ✅ Prove that HUD actions resolve to explicit locations
3. ✅ Confirm that rename = commit at same location (not relocation)
4. ✅ Show that "Save As" pattern is impossible
5. ✅ Prove no temporary/floating state exists

**FAIL if:**
- Commits can be created without location
- HUD can invent locations implicitly
- "Save As" dialog exists
- Temporary files allowed

---

## 🎯 THE PAYOFF

**By enforcing this law, you eliminate:**
- ❌ Orphaned files
- ❌ "Which version?" confusion
- ❌ Naming chaos (final_v7_REAL_FINAL.docx)
- ❌ Hours searching for files
- ❌ Duplicate state
- ❌ Manual reconciliation
- ❌ Human memory as infrastructure

**You gain:**
- ✅ Clear lineage
- ✅ Unambiguous location
- ✅ Automatic version control
- ✅ Preserved history
- ✅ Structural reconciliation
- ✅ Zero search time
- ✅ Physics as infrastructure

---

## 🔒 FINAL LOCK

**Canonical statement (one sentence):**

> **"In Relay, no state may be created without an explicit location on the filament tree. Everything else is a projection."**

**This is not UX preference.**  
**This is state-creation physics.**

**And it unifies with:**
- Reconciliation law (meaning)
- Authority expiry law (power)
- Commit-at-location law (data)

**Same rule. Three domains. Perfect alignment.**

---

**Status:** 🔒 **LOCKED AS FOUNDATIONAL INVARIANT**  
**Type:** State Creation Law  
**Layer:** Foundation  
**Refs:** RECONCILIATION-PROTOCOL.md, AUTHORITY-DELEGATION-COMPLETE.md, THE-RELAY-THESIS.md

**END OF COMMIT-AT-LOCATION LAW**

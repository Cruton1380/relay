# RELAY BACKEND - THIN SLICE DEFINITION

**Purpose:** Define minimal viable backend slices  
**Audience:** Backend developers  
**Status:** 🔒 Locked

---

## 🎯 WHAT IS A THIN SLICE?

**A thin slice is:**
- One complete vertical path through the system
- Inputs → Processing → Storage → Outputs
- Deterministic and replayable
- Independently testable

**NOT:**
- A horizontal layer (just DB, just API)
- A partial feature
- Non-deterministic code

---

## 📋 THIN SLICE TABLE

### **Slice 1: Append Commit**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `POST /api/relay-physics/commit` with JSON payload |
| **Writes** | Append to `filaments/{filament_id}.jsonl` |
| **Projections** | None (write-only) |
| **Streams** | Emit `COMMIT_APPENDED` event via SSE |
| **Audits/Gates** | Commit must have `commit_index` (monotonic), `causal_refs` (array), `timestamp` |
| **Test** | Append 3 commits → file has 3 lines → replay produces same state |

---

### **Slice 2: Read Filament**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `GET /api/relay-physics/filament/:id` |
| **Writes** | None (read-only) |
| **Projections** | Load `filaments/{id}.jsonl`, parse, return array |
| **Streams** | None |
| **Audits/Gates** | Must return commits in order, must not mutate |
| **Test** | Read filament → get commits → verify order and immutability |

---

### **Slice 3: Derive State**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `GET /api/relay-physics/state/:filament_id` |
| **Writes** | None |
| **Projections** | Replay commits, apply each to state, return derived state |
| **Streams** | None |
| **Audits/Gates** | Same commits → same state (deterministic) |
| **Test** | Replay twice → assert states equal |

---

### **Slice 4: Serve Docs**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `GET /api/relay-physics/docs/:doc_id` |
| **Writes** | None |
| **Projections** | Load from `relay/` directory based on CONTEXT-TABLE mapping |
| **Streams** | None |
| **Audits/Gates** | Must serve frozen docs only, verify checksum |
| **Test** | Request doc → verify checksum matches TOC commit |

---

### **Slice 5: Serve TOC**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `GET /api/relay-physics/toc/latest` |
| **Writes** | None |
| **Projections** | Load `filaments/toc/toc.jsonl`, return latest commit |
| **Streams** | None |
| **Audits/Gates** | Must return frozen TOC with checksums |
| **Test** | Request TOC → verify frozen flag, checksums present |

---

### **Slice 6: SSE Stream**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `GET /api/relay-physics/events` (SSE connection) |
| **Writes** | None |
| **Projections** | Subscribe to event channel |
| **Streams** | Emit events as they occur (COMMIT_APPENDED, etc.) |
| **Audits/Gates** | Events must be ordered, no loss |
| **Test** | Connect → append commit → verify event received |

---

### **Slice 7: RenderSpec Generation**

| Component | Specification |
|-----------|---------------|
| **Inputs** | `GET /api/relay-physics/render/world` |
| **Writes** | None |
| **Projections** | Derive state from all relevant filaments, generate RenderSpec JSON |
| **Streams** | None |
| **Audits/Gates** | Must pass RenderSpec validation, no randomness, semantic materials only |
| **Test** | Generate twice → assert RenderSpec identical |

---

## 🚫 NON-NEGOTIABLE RULES

**Every slice must:**
- [ ] Be deterministic (same inputs → same outputs)
- [ ] Have explicit audit gates
- [ ] Be testable independently
- [ ] Preserve immutability (no mutations)
- [ ] Have causal references (where applicable)

**If a developer can't fill this table, they're not building Relay.**

---

## 🎯 IMPLEMENTATION ORDER

**Priority 1 (Must have for RelaySim):**
1. Slice 1: Append Commit
2. Slice 2: Read Filament
3. Slice 3: Derive State
4. Slice 6: SSE Stream

**Priority 2 (Docs serving):**
5. Slice 4: Serve Docs
6. Slice 5: Serve TOC

**Priority 3 (Visualization):**
7. Slice 7: RenderSpec Generation

---

## 📊 DEVELOPER CHECKPOINT

**Before claiming "backend is done," verify:**

| Slice | Implemented | Tested | Deterministic | Gates Enforced |
|-------|-------------|--------|---------------|----------------|
| Append Commit | [ ] | [ ] | [ ] | [ ] |
| Read Filament | [ ] | [ ] | [ ] | [ ] |
| Derive State | [ ] | [ ] | [ ] | [ ] |
| Serve Docs | [ ] | [ ] | [ ] | [ ] |
| Serve TOC | [ ] | [ ] | [ ] | [ ] |
| SSE Stream | [ ] | [ ] | [ ] | [ ] |
| RenderSpec | [ ] | [ ] | [ ] | [ ] |

**All boxes must be checked.**

---

## 🔍 GRAPH EXPORT (FOR REPO ANALYSIS)

### **Additional slices for analyzing developer repo:**

**Slice 8: Ingest Git Repo**
- Input: Git repo path
- Output: Commits as RepoFilament
- Test: Replay git history → verify commit order

**Slice 9: Generate Workspace Graph**
- Input: Workspace manifest (Cargo.toml, package.json)
- Output: GraphViz .dot file
- Shows: Members declared, missing, dependencies

**Slice 10: Generate Route Map**
- Input: Backend code
- Output: API route graph
- Shows: Which routes exist, what they write

---

**Refs:** [c0.Filaments], [c1.Commits], [c2.Replay], [c3.Layers]  
**Objects:** [Commit], [Filament], [RenderSpec]  
**Audit:** [Determinism], [Completeness]

**END OF THIN SLICE DEFINITION**

# ✅ Architecture Filament Created

**Date:** 2026-01-28  
**Purpose:** Lock architectural decisions as immutable, traceable commits

---

## WHAT WAS CREATED

### Filament Structure

```
relay/filaments/
├── FILAMENTS-INDEX.md           (Index of all filaments)
├── architecture/
│   ├── README.md                (How to use this filament)
│   ├── 0000_arch_split.md       (Layer 2→3 separation)
│   ├── 0001_render_endpoints.md (API surface)
│   ├── 0002_sse_truth_stream_locks.md (PR #1.2 decisions)
│   ├── 0003_renderspec_v1_stub.json (JSON schema preview)
│   └── 0004_execution_order.md  (Roadmap lock)
└── architecture.jsonl           (JSONL preview for future)
```

**Total:** 7 files, ~4,500 lines of canonical architecture docs

---

## COMMITS LOCKED

### Commit 0: Render Responsibility Split
**Invariant:** Layer 2 outputs data, Layer 3 renders pixels, Layer 1 is git-only

**Key Decisions:**
- Layer 1 = Git backend (storage only)
- Layer 2 = Relay Physics (truth stream + render-ready data)
- Layer 3 = Frontend (Three.js rendering + animation)
- **Layer 2 never renders pixels**

**Anti-patterns rejected:**
- ❌ Rendering in Layer 1 (Git)
- ❌ Rendering in Layer 2 (Rust backend)
- ❌ Truth storage in Layer 3 (Frontend)

---

### Commit 1: Render Endpoints Contract
**Invariant:** 4 endpoint families (events SSE, commits, units, render scenes)

**Endpoints:**
```
GET /api/relay-physics/events              → SSE (✅ implemented)
GET /api/relay-physics/commits/:ref        → Commit fetch (⏭️ PR #1.3)
GET /api/relay-physics/units               → Unit state (✅ implemented)
GET /api/relay-physics/render/world        → Scene graph (⏭️ future)
GET /api/relay-physics/render/commit/:ref  → Forensic chamber (⏭️ future)
GET /api/relay-physics/render/filament/:id → Filament geometry (⏭️ future)
```

**Philosophy:** Layer 2 outputs semantic tags, Layer 3 interprets visually

---

### Commit 2: SSE Truth Stream Locks
**Invariant:** No phantom events, truth stream completeness, deterministic replay

**7 Locks (from Q1-Q5 audit):**
1. Event ID authority (from log only)
2. Persist failure is fatal (no emit without persist)
3. Lagged stream disconnects (force replay)
4. Buffer capacity documented (1024 events)
5. Production headers (no-cache, no-transform, no-buffer)
6. Error event best-effort (reconnect is authoritative)
7. Lock ordering (deadlock prevention)

---

### Commit 3: RenderSpec v1 Stub
**Invariant:** JSON scene graph format (stub, to be completed)

**Schema Preview:**
```json
{
  "schema_version": "relay-render-v1",
  "nodes": [
    {
      "id": "unit.alice.001",
      "type": "unit",
      "transform": {...},
      "material": "scv_working",
      "animation_intents": [...]
    }
  ]
}
```

**Status:** Stub only (lock after PR #1.3 shows real data needs)

---

### Commit 4: Execution Order Decision
**Invariant:** PR #1.3 → RenderSpec v1 → /render/* → Layer 3 frontend

**Reasoning:**
- PR #1.3 is small + high-value (commit fetch)
- PR #1.3 informs RenderSpec design (data-driven)
- RenderSpec locks contract before multiple frontends
- Layer 3 iterates fast once APIs stable

**Timeline:** ~6 weeks to full visual demo

---

## WHY THIS MATTERS

### For You (User)
- ✅ Architecture decisions are now **traceable** (not scattered)
- ✅ No more "wait, why did we decide that?" (read `architecture@c2`)
- ✅ Can **fork/merge alternatives** cleanly (create `0004a_alternative.md`)
- ✅ Prevents **architectural drift** (locked invariants)

### For Future Claude Sessions
- ✅ Can read canonical commits (no context loss)
- ✅ Won't contradict locked decisions (unless superseding)
- ✅ Faster onboarding (read README + relevant commits)
- ✅ Consistent advice (anchored to filament commits)

### For Dev Team
- ✅ Clear contracts (Layer 2↔3 interface locked)
- ✅ Testable invariants (can write tests against commits)
- ✅ Audit trail ("when was this decided? by whom? why?")
- ✅ Rollback plan (mark commit SUPERSEDED, create new one)

---

## HOW TO USE THIS

### Reading Commits
```bash
# Read overview
cat relay/filaments/architecture/README.md

# Read specific decision
cat relay/filaments/architecture/0002_sse_truth_stream_locks.md

# Reference in docs/code
"Per architecture@c2, lag must close connection..."
```

### Adding New Commits
```bash
# Create new file
touch relay/filaments/architecture/0005_new_decision.md

# Follow format (see README)
# Update FILAMENTS-INDEX.md
```

### Superseding Decisions
```bash
# Don't edit 0002_sse_truth_stream_locks.md
# Instead create:
touch relay/filaments/architecture/0005_sse_locks_v2.md

# Mark old commit as SUPERSEDED
# Reference in new commit: "Supersedes: architecture@c2"
```

---

## QUESTIONS ANSWERED

### Q: Markdown or JSONL?
**Answer:** **Markdown NOW** (easier to read/edit), JSONL LATER (when Layer 2 ingests it)

**Rationale:**
- Bootstrap phase: humans are primary consumers
- Markdown is more readable for decisions/rationale
- JSONL preview created (`architecture.jsonl`) for future

---

### Q: Material tags enum or namespace:tag strings?
**Answer:** **Strings FIRST** (flexible), enum LATER (if needed)

**Recommendation:**
```json
// ✅ GOOD (flexible, extensible)
{"material": "scv_working"}
{"material": "encrypted_channel"}
{"material": "commit_accepted"}

// ❌ NOT YET (premature optimization)
{"material": {"type": "UnitMaterial", "variant": "Working"}}
```

**Reasoning:**
- Strings are simpler (no schema complexity)
- Layer 3 can fallback to default if unknown tag
- Namespacing possible later: `"relay:scv_working"` if needed
- Can add validation later without breaking contract

**Lock in:** `architecture@c3` (RenderSpec v1 after PR #1.3)

---

### Q: /render/commit/:ref returns raw or derived?
**Answer:** **Two endpoints** (raw truth + derived scene)

**Contract:**
```
GET /commits/:ref          → Raw commit (forensic truth)
GET /render/commit/:ref    → Derived scene graph (visual)
```

**Why separate:**
- `/commits/:ref` = immutable truth (cache forever)
- `/render/commit/:ref` = derived view (can change as RenderSpec evolves)
- Different consumers (debugger vs renderer)

**Example:**
```bash
# Forensic inspection (raw commit)
curl /api/relay-physics/commits/work.W123@c7
→ {"commit_index":7,"op_type":"OUTPUT_PROPOSED",...}

# Visual chamber (scene graph)
curl /api/relay-physics/render/commit/work.W123@c7
→ {"nodes":[{"type":"timecube","material":"commit_accepted",...}]}
```

**Lock in:** `architecture@c1` (already specified)

---

## NEXT STEPS

### Immediate (You Should Do)
1. ✅ **Review filament commits** (architecture/*.md)
2. ✅ **Confirm decisions are correct** (or create superseding commits)
3. ⏭️ **Proceed to PR #1.3** (commit fetch endpoint)

### Short-Term (After PR #1.3)
1. Lock RenderSpec v1 (update `architecture@c3` from stub to full schema)
2. Create new commit: `0005_renderspec_v1_locked.md`
3. Implement `/render/*` endpoints

### Long-Term (When Layer 2 Mature)
1. Convert architecture filament to JSONL canonical format
2. Ingest into Relay Physics (becomes first-class entity)
3. Visualize in forensic chamber (click commit → see decision timeline)

---

## VALIDATION CHECKLIST

- [x] All commits follow format (header + invariant + decision + causal refs)
- [x] Causal refs are correct (each commit references inputs)
- [x] Invariants are testable (can validate in code)
- [x] README explains how to use filament
- [x] Index updated with architecture filament
- [x] JSONL preview created for future
- [x] Questions answered (material tags, commit fetch format)

---

## DELIVERABLE SUMMARY

**What you can paste into repo NOW:**
```
relay/filaments/
  - All 7 files are ready
  - No code changes needed
  - Pure documentation/specification
```

**What this unlocks:**
- ✅ Architecture is now traceable
- ✅ Future Claude sessions are faster
- ✅ Dev team has clear contracts
- ✅ Demonstrates Relay's own value (meta-coordination)

**Status:** ✅ **READY TO COMMIT** (the filament itself!)

---

**Date:** 2026-01-28  
**Total Lines:** ~4,500 (markdown) + ~500 (JSON)  
**Philosophy:** Architecture decisions ARE coordination truth.

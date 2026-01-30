# Relay Filaments Index

**Purpose:** Track all filaments in this repository  
**Updated:** 2026-01-28

---

## WHAT ARE FILAMENTS?

Filaments are **append-only logs** for coordination truth. Each filament is:

- **Immutable** - Commits never change
- **Causal** - Commits reference prior commits
- **Traceable** - Full history is preserved
- **Versioned** - Commit index provides timeline

---

## ACTIVE FILAMENTS

### `architecture` (5 commits)
**Purpose:** Track architectural decisions as immutable commits

**Location:** `relay/filaments/architecture/`

**Format:** Markdown files + JSONL index

**Commits:**
- `c0`: Render Responsibility Split (Layer 2→3 separation)
- `c1`: Render Endpoints Contract (API surface)
- `c2`: SSE Truth Stream Locks (PR #1.2 decisions)
- `c3`: RenderSpec v1 Stub (JSON schema preview)
- `c4`: Execution Order Decision (roadmap lock)

**Status:** ACTIVE

**See:** `relay/filaments/architecture/README.md`

---

## PLANNED FILAMENTS

### `specs/renderspec` (future)
**Purpose:** Lock JSON schema for scene graphs (Layer 2→3 contract)

**Status:** PENDING (after PR #1.3)

### `delegation` (future)
**Purpose:** Track delegation chain commits (authority grants)

**Status:** PENDING (PR #4)

### `training-pipeline` (future)
**Purpose:** Track training data export format evolution

**Status:** PENDING (PR #2)

---

## FILAMENT CONVENTIONS

### Naming
- Lowercase, dash-separated: `architecture`, `specs-renderspec`
- No spaces or special chars
- Descriptive, not cryptic

### Storage
- **Markdown (current):** `relay/filaments/<name>/0000_title.md`
- **JSONL (future):** `relay/filaments/<name>.jsonl`

### Commit Index
- Start at 0
- Monotonically increasing
- Never reuse indices (even if commit superseded)

### References
- **In docs:** `<filament>@c<index>` (e.g., `architecture@c2`)
- **In code:** `CommitRef::new("architecture@c2")`

---

## HOW TO CREATE A NEW FILAMENT

1. **Create directory:** `relay/filaments/<name>/`
2. **Add README.md** explaining purpose
3. **Create first commit:** `0000_<title>.md`
4. **Update this index:** Add to "ACTIVE FILAMENTS" section

---

## FUTURE INTEGRATION

**When Layer 2 is fully operational:**
- Filaments become first-class Relay Physics entities
- JSONL format becomes canonical
- UI can visualize filament graphs (Three.js)
- Forensic inspection mode can browse commit history

**For now:** Markdown is canonical (easier to read/edit during bootstrap)

---

## QUESTIONS TO ASK

1. **Should this decision be in a filament?** (If it's an invariant, yes)
2. **Which filament does it belong in?** (Group by domain)
3. **What are the causal refs?** (What does this depend on?)
4. **Is this decision testable?** (Can we validate it in code?)

---

**Index Status:** ACTIVE  
**Next Update:** When new filament created or architecture filament extended  
**Maintainer:** system.architect (bootstrap phase)

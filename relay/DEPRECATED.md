# DEPRECATED DOCUMENTATION

**Last Updated:** 2026-01-29  
**Purpose:** Index of deprecated documents and their replacements

---

## 🚫 DEPRECATION POLICY

**Documents listed here are no longer authoritative.**

**Rules:**
1. Deprecated docs may remain for historical reference
2. All new work MUST use replacement documents
3. Deprecated docs MUST NOT be imported or referenced
4. Any document not in CONTEXT-TABLE.json is considered deprecated by default

---

## 📋 DEPRECATED FILES

### **ARCHITECTURE-INDEX.md** (Root level)
- **Status:** Deprecated (moved)
- **Replaced by:** `architecture/ARCHITECTURE-INDEX.md`
- **Reason:** Consolidated into proper location
- **Action:** Use new location only

---

### **RELAY-BUILDINGS-EXPLAINED-FOR-AI.md**
- **Status:** Deprecated (superseded)
- **Replaced by:** `reference/RELAY-OBJECTS-REFERENCE.md#Building`
- **Reason:** Consolidated into canonical object reference
- **Action:** Use object reference for all object definitions

---

### **PERSONAL-HUD-STARCRAFT-MODEL-LOCKED.md**
- **Status:** Deprecated (superseded)
- **Replaced by:** `architecture/filaments/0009_personal_hud_physical_globe.md`
- **Reason:** Duplicate of architecture@c9
- **Action:** Use architecture filament as source of truth

---

### **ECONOMIC-SUBSTRATE-LOCKED.md**
- **Status:** Deprecated (superseded)
- **Replaced by:** 
  - `architecture/filaments/0006_economic_primitives_foundation.md`
  - `architecture/ARCHITECTURE-INDEX.md#c6`
- **Reason:** Consolidated into architecture filament
- **Action:** Use architecture filaments for physics

---

### **COORDINATION-SUBSTRATE-GOLD-STANDARD.md**
- **Status:** Deprecated (superseded)
- **Replaced by:**
  - `architecture/filaments/0007_coordination_gauges_hud.md`
  - `architecture/ARCHITECTURE-INDEX.md`
- **Reason:** Consolidated into architecture system
- **Action:** Use architecture index + filaments

---

### **HUD-GLOBE-VISUAL-SPEC.md**
- **Status:** Deprecated (superseded)
- **Replaced by:**
  - `architecture/filaments/0009_personal_hud_physical_globe.md`
  - `reference/RELAY-OBJECTS-REFERENCE.md`
- **Reason:** Visual spec merged into architecture and object reference
- **Action:** Use architecture for model, object reference for details

---

### **RENDERSPEC-V1-LOCKED.md** (Root level)
- **Status:** Deprecated (moved)
- **Replaced by:** `architecture/filaments/0005_renderspec_v1_locked.md`
- **Reason:** Duplicate of architecture@c5 (may contain additional details - audit before removal)
- **Action:** Use architecture filament; merge any unique content

---

### **PR-6-COMPLETION-REPORT.md**
- **Status:** Deprecated (historical)
- **Replaced by:** `DOCUMENTATION-GENERATION-STATUS.md`
- **Reason:** Superseded by comprehensive status tracking
- **Action:** Historical artifact only

---

### **ARCHITECTURE-FILAMENT-CREATED.md**
- **Status:** Deprecated (historical)
- **Replaced by:** `architecture/filaments/README.md`
- **Reason:** Meta-doc about creation; now documented in README
- **Action:** Historical artifact only

---

### **SESSION-SUMMARY-2026-01-28.md**
- **Status:** Historical artifact (not deprecated, preserved)
- **Replaced by:** N/A (unique historical record)
- **Reason:** Session notes are historical artifacts, not authoritative docs
- **Action:** Preserve as-is; move to `history/` folder

---

### **SESSION-SUMMARY-2026-01-28-ECONOMIC.md**
- **Status:** Historical artifact (not deprecated, preserved)
- **Replaced by:** N/A (unique historical record)
- **Reason:** Session notes are historical artifacts
- **Action:** Preserve as-is; move to `history/` folder

---

### **THE-RELAY-EXPERIENCE.md**
- **Status:** Historical artifact (not deprecated, preserved as vision)
- **Replaced by:** N/A (unique North Star vision)
- **Reason:** North Star vision document; valuable for understanding goals
- **Action:** Preserve as-is; consider moving to `vision/` folder or integrating into MasterConversationFilament

---

### **FILAMENTS-INDEX.md**
- **Status:** Under review
- **Needs audit:** Check if superseded by `architecture/ARCHITECTURE-INDEX.md`
- **Action:** Audit content; merge or deprecate

---

## 📊 DEPRECATION SUMMARY

| File | Status | Replacement | Action |
|------|--------|-------------|--------|
| ARCHITECTURE-INDEX.md (root) | Deprecated | architecture/ARCHITECTURE-INDEX.md | Delete after verification |
| RELAY-BUILDINGS-EXPLAINED-FOR-AI.md | Deprecated | RELAY-OBJECTS-REFERENCE.md | Delete after content audit |
| PERSONAL-HUD-STARCRAFT-MODEL-LOCKED.md | Deprecated | c9 architecture filament | Delete |
| ECONOMIC-SUBSTRATE-LOCKED.md | Deprecated | c6 architecture filament | Delete |
| COORDINATION-SUBSTRATE-GOLD-STANDARD.md | Deprecated | Architecture system | Delete |
| HUD-GLOBE-VISUAL-SPEC.md | Deprecated | c9 + object reference | Delete |
| RENDERSPEC-V1-LOCKED.md (root) | Deprecated | c5 architecture filament | Audit, then delete |
| PR-6-COMPLETION-REPORT.md | Historical | Status doc | Move to history/ |
| ARCHITECTURE-FILAMENT-CREATED.md | Historical | README | Move to history/ |
| SESSION-SUMMARY-*.md | Historical | N/A | Move to history/ |
| THE-RELAY-EXPERIENCE.md | Vision | N/A | Preserve (vision/) |
| FILAMENTS-INDEX.md | Under review | TBD | Audit required |

---

## ✅ VERIFICATION

**After deprecation, verify:**
- [ ] No code imports deprecated docs
- [ ] No README references deprecated docs
- [ ] All cross-links updated
- [ ] CONTEXT-TABLE.json is sole navigation source
- [ ] Deprecated docs moved to history/ or deleted

---

## 🚨 CRITICAL RULE

**If a document is not in `reference/CONTEXT-TABLE.json`, it is NOT authoritative.**

**Only documents listed in CONTEXT-TABLE are canonical.**

---

## 📝 AUDIT REQUIRED

**Before finalizing deprecation:**

1. **Audit RENDERSPEC-V1-LOCKED.md**
   - Compare with c5 architecture filament
   - Merge any unique content
   - Then deprecate

2. **Audit FILAMENTS-INDEX.md**
   - Check if content superseded by ARCHITECTURE-INDEX
   - Merge if needed
   - Deprecate if redundant

3. **Audit THE-RELAY-EXPERIENCE.md**
   - Preserve as vision document
   - Consider integration into MasterConversationFilament
   - Keep as separate vision artifact

---

## 🎯 CANONICALIZATION STATUS

**Current state:**
- ✅ New canonical docs created
- ⏳ Legacy docs identified
- ⏳ Deprecation actions pending
- ⏳ Content audit incomplete
- ⏳ Verification incomplete

**After completing deprecation:**
- ✅ Single source of truth enforced
- ✅ No parallel definitions
- ✅ CONTEXT-TABLE is navigation source
- ✅ System is canonical

---

**END OF DEPRECATION INDEX**

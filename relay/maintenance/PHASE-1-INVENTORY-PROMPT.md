# PHASE 1: INVENTORY - EXECUTABLE PROMPT

**Status:** Ready to execute  
**Duration:** ~15-30 minutes  
**No edits, just data collection**

---

## 📋 COPY-PASTE THIS TO AGENT

```
PHASE 1: FULL FILE INVENTORY

Your task is to create a complete inventory of all files in the Relay documentation
system. This is data collection only - NO EDITS, NO DECISIONS.

## Directory to Scan

Start from: c:\Users\eitana\Desktop\App Development\Relay\clevertree-relay\relay\

Recursively scan all subdirectories.

## For Each File, Record:

1. **path** (relative to relay/)
2. **type** (file extension: md, json, jsonl, docx, png, txt, etc.)
3. **last_modified** (ISO 8601 timestamp)
4. **size_bytes** (file size in bytes)
5. **referenced_by** (grep through all .md and .json files for mentions of this filename)

## Output Format

Generate: `maintenance/FILE-INVENTORY.json`

Structure:
```json
{
  "inventory_date": "2026-01-29T[current_timestamp]",
  "scan_root": "relay/",
  "total_files": [count],
  "total_size_bytes": [sum],
  "files": [
    {
      "path": "RELAY-CONTEXT-MAP.md",
      "type": "markdown",
      "last_modified": "2026-01-29T12:34:56Z",
      "size_bytes": 12543,
      "referenced_by": [
        "reference/CONTEXT-TABLE.json",
        "summaries/RELAY-IN-100-WORDS.md"
      ],
      "reference_count": 2
    },
    {
      "path": "reference/CONTEXT-TABLE.json",
      "type": "json",
      "last_modified": "2026-01-29T14:22:10Z",
      "size_bytes": 8234,
      "referenced_by": [
        "RELAY-CONTEXT-MAP.md",
        "reference/HOW-TO-READ-RELAY.md"
      ],
      "reference_count": 2
    }
  ]
}
```

## Reference Detection Rules

When checking "referenced_by":
- Search for exact filename in all .md and .json files
- Include relative path references (e.g., "reference/CONTEXT-TABLE.json")
- Include bare filename references (e.g., "CONTEXT-TABLE.json")
- Ignore references in the file to itself
- Case-insensitive matching

## Exclusions

DO NOT include:
- node_modules/
- .git/
- Hidden files (starting with .)
- Build artifacts
- Temporary files

## Summary Statistics

At the end of the JSON, include:
```json
{
  ...
  "summary": {
    "by_type": {
      "markdown": 45,
      "json": 12,
      "jsonl": 3,
      "docx": 5
    },
    "orphans": [
      "files with reference_count === 0"
    ],
    "most_referenced": [
      {
        "path": "...",
        "reference_count": N
      }
    ]
  }
}
```

## Deliverable

1. Generate complete FILE-INVENTORY.json
2. Report total files found
3. Report total size
4. Highlight any orphans (reference_count === 0)
5. List top 5 most-referenced files

## STOP AFTER THIS PHASE

Do NOT:
- Make any decisions
- Classify files
- Edit any files
- Delete anything
- Move anything

Just collect data and stop.

Report: "Phase 1 complete. FILE-INVENTORY.json generated. Ready for review."
```

---

## ✅ EXPECTED OUTPUT

**Agent should produce:**

1. **`maintenance/FILE-INVENTORY.json`** (~50-100KB)
2. **Summary report:**
   ```
   Phase 1 Complete: File Inventory
   
   Total files scanned: 89
   Total size: 2.4 MB
   
   By type:
   - Markdown: 58 files
   - JSON: 8 files
   - JSONL: 3 files
   - Word docs: 4 files
   - Images: 6 files
   - Other: 10 files
   
   Orphans found: 12 files
   (Files never referenced by any other document)
   
   Top 5 most-referenced:
   1. CONTEXT-TABLE.json (23 references)
   2. RELAY-CONTEXT-MAP.md (15 references)
   3. ARCHITECTURE-INDEX.md (12 references)
   4. RELAY-OBJECTS-REFERENCE.md (11 references)
   5. HOW-TO-READ-RELAY.md (9 references)
   
   Ready for Phase 2: Classification
   ```

---

## 🔍 WHAT TO REVIEW

**After Phase 1, check:**

1. **File count** - Does 89 files seem right?
2. **Orphans** - Are there files you forgot about?
3. **Most-referenced** - Are these the right entry points?
4. **File types** - Any unexpected types?
5. **Total size** - Any surprisingly large files?

**Look for:**
- Forgotten temporary files
- Duplicate files
- Files you didn't know existed
- Incorrectly placed files

---

## 🎯 APPROVAL TO PROCEED

**Once reviewed, if everything looks correct:**

**Issue command:** "Proceed to Phase 2: Classification"

**If issues found:**

**Issue command:** "Phase 1 revealed [issue]. Before proceeding to Phase 2, [action needed]."

---

## 📌 STOPPING POINT

**This is a deliberate stopping point.**

**Do NOT proceed to Phase 2 without:**
- Human review of FILE-INVENTORY.json
- Verification of file count
- Identification of any unexpected files
- Explicit approval to continue

**Systematic process requires systematic pauses.**

---

**Ready to execute:** YES  
**Safe to run:** YES (read-only)  
**Reversible:** N/A (no changes made)

**END OF PHASE 1 PROMPT**

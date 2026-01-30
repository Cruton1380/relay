# HUD HELP LAYER SPECIFICATION

**Object-Embedded Training System**  
**Type:** Core Architecture Module  
**Layer:** Reference  
**Status:** 🔒 Locked  
**Date:** 2026-01-29

---

## 🔑 CORE PRINCIPLE

**In Relay, Help IS the HUD.**

**There is no:**
- Separate help center
- External documentation site
- Tutorial flow outside the system
- "Go read the docs" redirect

**Help manifests as:**
- An in-world HUD layer
- Always instantly available
- On any surface, object, panel, tool, or generator
- Governed by the same control primitives as all Relay operations

---

## ⚠️ CRITICAL INVARIANT

**Help is not UI chrome. Help is control physics.**

**User triggers help via:**
- ⏸ **HOLD** = freeze state + enter explain mode
- ✋ **STOP** = cancel explain mode + mark incomplete
- 🌱 **FORK** = branch explanation path (alternate learning route)

**These are the same primitives used for all execution control.**

---

## 🎬 FROM FILM TO OBJECT-LOCAL TRAINING

### **The Relay Film Evolution**

**Global Film (Genesis):**
- One upfront movie
- Introduces: problem, physics, inevitability
- No object specificity
- No interaction required

**In-System Training (Everywhere Else):**
- Film is decomposed into micro-training units
- Each object exposes its relevant film fragments
- Plus object-specific explanation layers
- Reused, not rewritten

### **The Decomposition Rule**

**Every scene and every object demonstrated in the film MUST also exist as:**

1. A standalone micro-training unit
2. Bound directly to the object it explains
3. Available in multiple modalities
4. Callable instantly from any surface
5. Governed by STOP/HOLD/FORK primitives

**Think: The film shatters into lenses.**

---

## 📐 OBJECT-LOCAL TRAINING UNITS

### **Required Components**

**Every object that requires understanding must provide:**

**Examples:**
- Filament
- Vote
- Queue
- SCV
- Zone boundary
- Refusal
- Fork
- Rewind
- Building
- Shipment
- Commit
- Authority chain

---

## 🎨 THREE EXPLANATION MODES (ALL REQUIRED)

### **Mode 1: Graphics-Only**

**No narration**  
**No text paragraphs**  
**Visuals + motion + emphasis only**

**Use cases:**
- Users who prefer silent learning
- Deaf/hard-of-hearing users (with captions)
- Environments where sound is inappropriate
- Users who want fastest possible explanation

**Must show:**
- Object structure
- State changes
- Interaction points
- Control primitives
- Failure modes

**Format:**
- Animated 3D/2D graphics
- Motion design
- Highlighting/emphasis
- Arrows/flows
- Before/after states

---

### **Mode 2: Narrated**

**Voice explanation + graphics**

**Canonical narrator:** The human creator (default)

**Voice characteristics:**
- Clear, conversational
- Technical but accessible
- Non-patronizing
- Confidence without authority claims

**AI avatar allowed as:**
- Placeholder during development
- Guide for creator re-recording
- Demonstration of pacing/framing

**Critically:** Spoken words must be swappable later

**Narration must:**
- Match graphics timing exactly
- Reference visible elements
- Avoid abstract metaphors
- State invariants clearly
- Acknowledge unknowns

**Format:**
- Audio track (mp3/wav)
- Synchronized to graphics
- Timestamped
- Replaceable without breaking sync

---

### **Mode 3: Hybrid**

**Graphics + narration + minimal labels**

**Labels are:**
- Identity hints (object names)
- Property indicators (values)
- State markers (active/paused/sealed)

**Labels are NOT:**
- Summaries
- Explanations
- Marketing copy
- Instructions

**Use cases:**
- Users who want both visual and audio
- Mixed learning preferences
- Accessibility (captions available)
- Default recommended mode

**Format:**
- All Mode 1 + Mode 2 elements
- Plus: on-screen text labels
- Captions available (SRT/VTT)

---

## ⚠️ CRITICAL RULE: NO NEW SEMANTICS

**No mode may introduce new semantics.**

**All three modes must:**
- Teach the same invariants
- Reference the same objects
- Demonstrate the same physics
- Result in the same understanding

**If:**
- Graphics-only teaches X
- Narrated teaches X + Y
- **→ INVALID**

**Modes are presentation choices, not content variants.**

---

## 🎮 CONTROL PRIMITIVES (NON-NEGOTIABLE)

**All training units must obey:**

### **⏸ HOLD — Pause Explanation**
- Preserves current state
- Allows inspection of current frame
- User can zoom into details
- Resume from exact point

### **✋ STOP — Cancel Explanation**
- Exits explain mode
- Marks explanation incomplete
- Preserves object state (not training state)
- No penalty

### **🌱 FORK — Branch Explanation Path**
- Creates alternate learning route
- "Show me the details" vs "Keep it high-level"
- "Show me the failure case" vs "Show me the success path"
- All branches preserved

**This applies to:**
- Video playback
- Narration
- Step-by-step walkthroughs
- AI-generated guidance

**Training is execution, not media.**

---

## 🔗 SOURCE OF TRUTH: FILM → OBJECT MAPPING

### **Traceability Rule**

**No explanation may:**
- Contradict the film
- Exceed the film's physics
- Introduce concepts not in film or architecture

**If ambiguity exists:**
1. Film invariant wins
2. Architecture wins over projection
3. Silence is preferable to speculation

### **Required Metadata (Per Training Unit)**

```json
{
  "training_unit_id": "object.filament.intro.v1",
  "object_id": "filament",
  "film_scenes": ["s02_magic_move", "s08_crisis", "s10_rewind"],
  "architecture_refs": ["c0", "c1", "c2"],
  "invariants_demonstrated": [
    "Filaments are append-only",
    "History is visible",
    "Commits are discrete"
  ],
  "modes": {
    "graphics_only": "assets/training/filament_intro_graphics.mp4",
    "narrated": "assets/training/filament_intro_narrated.mp4",
    "hybrid": "assets/training/filament_intro_hybrid.mp4"
  },
  "transcript": "assets/training/filament_intro_transcript.txt",
  "captions": "assets/training/filament_intro_captions.srt",
  "duration_seconds": 45,
  "control_points": {
    "pause_safe": [5, 15, 30, 40],
    "branch_points": [15, 30]
  },
  "dependencies": [],
  "status": "locked"
}
```

---

## 👤 THE HUD CHARACTER IS A SKIN (SWAPPABLE)

### **Default: Creator Avatar**

**The human creator will appear as an avatar that can show up everywhere as the default help entity.**

**Characteristics:**
- Visual representation (3D/2D avatar)
- Voice (human-recorded)
- Mannerisms (pacing, pauses)
- Personality (accessible, technical, honest)

**Critically:**
- The avatar is not required
- The avatar is not authority
- The avatar is a themeable, replaceable HUD character template

---

### **User Choices (All Valid)**

Users can choose:

1. **Creator Avatar** (default)
   - Full human narrator experience
   - Visual + voice + presence

2. **HUD-Only** (no character)
   - Pure interface
   - Graphics + text
   - No anthropomorphic presence

3. **Alternate Character Templates**
   - Different avatar styles
   - Different voice actors
   - Different visual aesthetics

4. **Silent Mode** (graphics-only)
   - No narration
   - No voice
   - Pure visual teaching

5. **Subtitles-Only**
   - Text captions
   - No audio
   - For accessibility or preference

6. **Any Combination**
   - Mix and match
   - User-defined

---

### **Swappability Invariant**

**The narration track is swappable without changing the explanation structure.**

**"Character" is presentation.**  
**Training structure is truth.**

**If swapping breaks understanding → the training unit is invalid.**

---

## 🛠️ WHERE HELP MUST EXIST

### **Relay is an IDE for Coordination Physics**

**Every tool surface must have HUD training built in:**

#### **1. IDE/Editor Surfaces**
- Code editing
- Prompt authoring
- Schema design
- Spec writing

#### **2. Movie/Image Generation Surfaces**
- AI movie generator
- AI image generator
- Asset creation tools
- Visual composer

#### **3. Simulation/Replay Surfaces**
- Forensic chamber
- Commit inspector
- Filament viewer
- Time scrubber

#### **4. Globe World Surfaces**
- 3D navigation
- Building selection
- Unit tracking
- Zone boundaries

#### **5. SCV/Task Orchestration**
- Agent roster
- Build queue
- Task delegation
- Execution contracts

#### **6. Voting/Governance Panels**
- Vote proposals
- Delegation chains
- Authority tracking
- Influence decay

#### **7. Merge/Conflict Panels**
- Branch comparison
- Scar inspection
- Fork creation
- Dispute resolution

#### **8. Resource/Capacity Gauges**
- Commitment slots
- Escrow tracking
- Time pressure
- Dispute signals

#### **9. Theme/Customization Marketplace**
- Theme browser
- Character templates
- Installation
- Forking/remixing

---

## 🚫 FORBIDDEN PATTERNS

**The following are INVALID:**

❌ **"Watch this later"** — No deferred learning  
❌ **Centralized help screens** — Help is local to object  
❌ **Tooltips that summarize** — Must explain, not hint  
❌ **New metaphors** — Film + architecture define language  
❌ **UX shortcuts** — No bypassing physics  
❌ **Assumed prior knowledge** — Every object is entry point

**If something is confusing:**  
**→ Expose more structure, not fewer rules.**

---

## ✅ SUCCESS CONDITIONS

### **A user must be able to:**

1. Enter Relay with no prior knowledge
2. Interact with any object
3. Trigger HOLD
4. Learn exactly what that object is
5. Resume without losing state

### **And:**

**A sufficiently skilled viewer can reconstruct Relay without ever watching the full movie again.**

---

## 📋 TRAINING PACK STRUCTURE (CANONICAL)

```json
{
  "training_pack_id": "pack.object.filament.v1",
  "object_id": "filament",
  "version": "1.0.0",
  "created": "2026-01-29",
  "modes": {
    "graphics_only": {
      "video": "assets/training/filament_graphics.mp4",
      "duration": 45,
      "keyframes": "assets/training/filament_keyframes.json",
      "storyboard": "assets/training/filament_storyboard.pdf"
    },
    "narrated": {
      "video": "assets/training/filament_narrated.mp4",
      "audio": "assets/training/filament_audio.wav",
      "duration": 45,
      "transcript": "assets/training/filament_transcript.txt",
      "narrator": "creator_default",
      "voice_meta": {
        "speaker": "human_creator",
        "swappable": true,
        "timing_locked": true
      }
    },
    "hybrid": {
      "video": "assets/training/filament_hybrid.mp4",
      "duration": 45,
      "captions": "assets/training/filament_captions.srt"
    }
  },
  "control_points": {
    "pause_safe": [5, 15, 30, 40],
    "branch_points": [
      {
        "time": 15,
        "branches": ["detail", "skip"]
      },
      {
        "time": 30,
        "branches": ["success_path", "failure_path"]
      }
    ]
  },
  "refs": {
    "film_scenes": ["s02", "s08", "s10"],
    "architecture": ["c0", "c1", "c2"],
    "objects": ["Filament", "Commit", "Timebox"]
  },
  "invariants": [
    "Filaments are append-only",
    "History is visible",
    "Commits are discrete"
  ],
  "dependencies": [],
  "checksums": {
    "graphics_only": "sha256:abc123...",
    "narrated": "sha256:def456...",
    "hybrid": "sha256:ghi789..."
  },
  "status": "locked"
}
```

---

## 🎯 PRODUCTION PIPELINE

### **For Each Object/Surface/Tool:**

**1. Identify**
- What needs explaining?
- What invariants does it demonstrate?
- Which film scenes show it?

**2. Extract**
- Relevant film fragments
- Keyframe sequences
- Timing/pacing

**3. Generate**
- Graphics-only version
- Narrated version (AI avatar placeholder)
- Hybrid version

**4. Record**
- Creator narration (replaces AI)
- Timing locked to graphics
- Multiple takes if needed

**5. Bind**
- Training pack to object
- Available on HOLD
- All modes functional

**6. Validate**
- STOP preserves object state
- FORK creates alternate paths
- Replay yields same understanding
- No mode introduces new semantics

---

## 🔒 FINAL RULES

### **The training system must obey the same laws as Relay:**

1. **No hidden authority** — Explanations don't claim correctness, they show structure
2. **No irreversible explanation** — STOP/HOLD/FORK always available
3. **No silent state change** — Training never modifies object state
4. **No semantic shortcuts** — Full physics, always

### **If an explanation cannot be paused, forked, or replayed:**
**→ It is invalid.**

---

## 📊 IMPLEMENTATION CHECKLIST

**Before any object ships:**

- [ ] Training pack exists
- [ ] Graphics-only mode functional
- [ ] Narrated mode functional
- [ ] Hybrid mode functional
- [ ] STOP/HOLD/FORK work
- [ ] Film scenes referenced
- [ ] Architecture refs present
- [ ] Invariants documented
- [ ] No new semantics introduced
- [ ] Swappable narrator confirmed
- [ ] Checksums recorded
- [ ] Status = locked

---

**Refs:** [RELAY-FILM-SPECIFICATION.md], [c0-c16], [COMPLETE-RELAY-EXPERIENCE.md]  
**Objects:** [TrainingPack], [HUDCharacter], [ExplanationMode], [ControlPrimitive]  
**Audit:** [Traceability], [Reproducibility], [Accessibility], [Non-Coercion]  
**Principle:** Help is physics, not UX

**END OF HUD HELP LAYER SPECIFICATION**

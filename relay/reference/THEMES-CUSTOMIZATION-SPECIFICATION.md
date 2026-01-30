# THEMES & CUSTOMIZATION SPECIFICATION

**First-Class Filaments, Tradeable, Governed**  
**Type:** Core Architecture Module  
**Layer:** Reference  
**Status:** 🔒 Locked  
**Date:** 2026-01-29

---

## 🎨 WHAT THEMES ARE (IN RELAY PHYSICS)

**A "theme" is not just colors.**

**A theme is a lens bundle:**

1. **HUD Visual Style**
   - Typography (fonts, sizes, weights)
   - Layout density (compact, comfortable, spacious)
   - Reticle style (cursor, selection indicators)
   - Spacing/padding rules

2. **Character Template**
   - Avatar (visual representation)
   - Voice (audio characteristics)
   - Persona shell (mannerisms, pacing)

3. **Explanation Mode Defaults**
   - Graphics-only
   - Narrated
   - Hybrid
   - Per-user preference

4. **Accessibility Presets**
   - Captions style
   - Contrast levels
   - Reduced motion
   - Screen reader optimizations

5. **Visual Grammar Mappings**
   - Semantic tags → materials
   - Semantic tags → shaders
   - Semantic tags → animations
   - Semantic tags → sound effects

---

## ⚠️ CRITICAL INVARIANT

**Themes are projection artifacts, not sources of truth.**

**They must NEVER change semantics — only presentation.**

**Invariant:** If a theme changes meaning, it is invalid.

**Example:**
- ✅ Theme changes gauge color from blue to purple
- ❌ Theme changes what the gauge represents
- ✅ Theme changes avatar appearance
- ❌ Theme changes explanation content
- ✅ Theme adds accessibility features
- ❌ Theme removes audit visibility

---

## 📦 THEME FILAMENTS (HOW THEMES EXIST)

**Themes must be stored as first-class filaments, with versioned commits.**

### **Entity Model**

#### **1. Theme Definition**
```
theme.<themeId>
```

**Fields:**
- `theme_id` (unique, stable)
- `name` (human-readable)
- `description`
- `version` (semantic versioning)
- `author_id` (creator identity)
- `created_at`
- `license` (usage rights)
- `category` (visual, minimal, high-contrast, etc.)

**Components:**
- `visual_style_ref` (pointer to style filament)
- `character_template_ref` (pointer to character)
- `lens_mapping_ref` (pointer to semantic mappings)
- `accessibility_preset_ref` (pointer to a11y config)

---

#### **2. Character Template**
```
character.<characterId>
```

**Fields:**
- `character_id`
- `name`
- `type` (human, abstract, HUD-only, custom)
- `avatar_assets` (3D model, textures, rigging)
- `voice_profile` (samples, characteristics)
- `animation_set` (idle, explaining, paused, etc.)
- `swappable` (boolean, always true)

**Voice Profile:**
```json
{
  "voice_id": "voice.creator_default",
  "type": "human_recorded",
  "characteristics": {
    "pitch": "mid",
    "pace": "conversational",
    "tone": "technical_accessible"
  },
  "sample_ref": "assets/voices/creator_sample.wav",
  "narration_tracks": [
    {
      "training_pack_id": "pack.object.filament.v1",
      "audio_ref": "assets/voices/creator_filament_narration.wav"
    }
  ]
}
```

---

#### **3. Lens Mapping**
```
lens.<lensId>
```

**Purpose:** Map semantic tags to visual/audio presentation

**Example:**
```json
{
  "lens_id": "lens.industrial_minimal",
  "mappings": {
    "building.vendor": {
      "material": "matte_concrete",
      "color": "#8B8B8B",
      "glow": false,
      "sound": "ambient_low_hum"
    },
    "filament.active": {
      "material": "luminous_thread",
      "color": "#4A9EFF",
      "animation": "gentle_pulse",
      "thickness": 2
    },
    "gauge.commitment_capacity": {
      "color": "#FFD700",
      "fill_style": "solid",
      "alert_threshold_visual": "pulse_urgent"
    }
  },
  "constraints": {
    "no_hidden_elements": true,
    "audit_always_visible": true,
    "refs_always_readable": true
  }
}
```

---

#### **4. Theme Pack**
```
pack.<packId>
```

**Purpose:** Bundle theme + character + lens + accessibility

```json
{
  "pack_id": "pack.industrial_complete.v2",
  "name": "Industrial Complete",
  "description": "Matte, minimal, forensic aesthetic",
  "version": "2.1.0",
  "author_id": "identity.creator",
  "components": {
    "theme": "theme.industrial",
    "character": "character.creator_default",
    "lens": "lens.industrial_minimal",
    "accessibility": "a11y.wcag_aaa"
  },
  "preview_assets": [
    "assets/themes/industrial_preview_hud.png",
    "assets/themes/industrial_preview_globe.png",
    "assets/themes/industrial_preview_chamber.png"
  ],
  "dependencies": [],
  "safe_mode_compatible": true,
  "audit_visibility_score": 1.0,
  "license": "CC-BY-4.0",
  "price": null,
  "status": "locked"
}
```

---

## 🏪 THEME MARKETPLACE / TRADING

**Theme distribution is done through Relay's own mechanisms:**

### **Publishing**

**Themes are published as packages with a manifest:**

```json
{
  "manifest_version": "1.0",
  "pack_id": "pack.cyberpunk_neon.v1",
  "name": "Cyberpunk Neon",
  "author_id": "identity.designer_xyz",
  "description": "High-contrast, animated, futuristic",
  "version": "1.0.0",
  "dependencies": [],
  "assets": [
    {
      "path": "assets/themes/cyberpunk/hud_style.json",
      "checksum": "sha256:..."
    },
    {
      "path": "assets/themes/cyberpunk/character.glb",
      "checksum": "sha256:..."
    }
  ],
  "preview": "assets/themes/cyberpunk/preview.mp4",
  "license": "MIT",
  "tags": ["high-contrast", "animated", "futuristic"],
  "safety": {
    "audit_visibility": true,
    "no_hidden_refs": true,
    "safe_mode": true
  }
}
```

---

### **User Actions**

Users can:

#### **1. Install**
- Download theme pack
- Verify checksums
- Preview before applying
- Apply to personal scope

#### **2. Fork**
- Create descendant branch
- Modify components
- Preserve original
- Publish fork (with attribution)

#### **3. Remix**
- Combine elements from multiple themes
- Create custom pack
- Must preserve safety rules

#### **4. Merge**
- Combine updates from parent theme
- Resolve conflicts (with scars if needed)
- Create new version

#### **5. Vote on Adoption**
- Signal theme quality (not truth)
- Influence discovery/ranking
- Delegated voting allowed

#### **6. Rate Reliability**
- Signal stability (as signal, not truth)
- Report safety violations
- Track patch history

---

### **Governance Constraints**

**Themes can be scoped:**

#### **1. Personal**
- User's own Relay instance
- No approval required
- Full customization freedom

#### **2. Project**
- Shared project workspace
- Team vote/approval
- Consistency for collaboration

#### **3. Organization**
- Enterprise-wide
- Requires authority chain
- May mandate accessibility compliance

#### **4. Global**
- Public marketplace
- Community-driven
- Safety review required

---

### **Enterprise Controls**

**Organizations may:**

- Lock allowed theme packs via `authorityRef`
- Require accessibility certification
- Mandate audit visibility thresholds
- Prohibit certain visual styles (e.g., low contrast)
- Enforce "safe mode" themes for critical operations

---

## 🚫 THEME SAFETY RULES (NON-NEGOTIABLE)

**Themes must NOT be able to:**

❌ **Hide refs** — All references must remain visible  
❌ **Obscure authority chains** — Delegation must be traceable  
❌ **Remove audit views** — Forensic chamber always accessible  
❌ **Suppress refusal states** — "No authority" must be clear  
❌ **Reduce legibility of conflict indicators** — Scars/disputes visible  
❌ **Fake legitimacy signals** — Gauges/influence accurate  
❌ **Hide time pressure** — Deadlines/decay clear  
❌ **Obscure zone boundaries** — Rule changes visible

---

### **Safety Validation**

**Before a theme can be applied:**

```json
{
  "safety_check": {
    "audit_visibility": true,
    "refs_readable": true,
    "authority_traceable": true,
    "refusal_clear": true,
    "conflicts_visible": true,
    "legitimacy_accurate": true,
    "time_pressure_clear": true,
    "zone_boundaries_visible": true
  }
}
```

**If any check fails → theme is marked "unsafe" and:**
- Requires explicit authority override
- Warning shown to user
- Or: prohibited entirely (org policy)

---

### **Dangerous Modifications**

**If a theme:**
- Reduces audit visibility
- Hides critical information
- Obscures control primitives (STOP/HOLD/FORK)
- Makes text unreadable (contrast too low)
- Removes accessibility features

**→ It is treated as a dangerous modification and:**
- Requires explicit `authorityRef`
- Or: is prohibited
- Logged as security event

---

## 🎨 THEME INSTALLATION & SWITCHING UX

**Theme switch is not "settings."**  
**It is an explainable action with HUD assistance.**

### **Installation Flow**

**1. Browse / Search**
- Marketplace interface
- Filter by category, safety, accessibility
- Preview available

**2. Preview**
- Show before/after comparison
- Highlight what changes
- Highlight what cannot change
- Show safety scores

**3. HOLD to Learn**
- User can hold on theme
- Training unit explains:
  - What it modifies
  - What it preserves
  - Safety implications
  - Performance impact

**4. Install**
- Download assets
- Verify checksums
- Apply to scope (personal/project/org)

**5. Activate**
- Switch current theme
- Immediate visual feedback
- Reversible

---

### **Switching Flow**

**User selects new theme:**

**System shows:**
- Current theme name
- New theme name
- What will change (visual diff)
- What will NOT change (invariants preserved)
- Safety comparison

**User can:**
- ⏸ **HOLD** — Learn differences
- ✋ **STOP** — Revert to previous
- 🌱 **FORK** — Try branch without committing
- ✅ **Apply** — Switch theme

**Theme changes are reversible through snapshot restore.**

---

## 📐 CANONICAL SCHEMAS

### **Theme Pack Manifest v1**

```json
{
  "$schema": "relay/schemas/theme-pack-manifest.v1.json",
  "pack_id": "pack.example_theme.v1",
  "name": "Example Theme",
  "description": "A sample theme for demonstration",
  "version": "1.0.0",
  "author_id": "identity.user_123",
  "created": "2026-01-29T12:00:00Z",
  "license": "MIT",
  "category": "minimal",
  "tags": ["clean", "accessible", "professional"],
  
  "components": {
    "theme": {
      "visual_style": {
        "typography": {
          "primary_font": "Inter",
          "monospace_font": "JetBrains Mono",
          "base_size": 14,
          "line_height": 1.5
        },
        "layout": {
          "density": "comfortable",
          "grid_spacing": 8,
          "panel_padding": 16
        },
        "colors": {
          "background": "#1E1E1E",
          "foreground": "#E0E0E0",
          "accent": "#4A9EFF",
          "warning": "#FFB74D",
          "error": "#F44336"
        }
      }
    },
    
    "character": {
      "character_id": "character.creator_default",
      "avatar_asset": "assets/characters/creator.glb",
      "voice_profile": "voice.creator_default"
    },
    
    "lens": {
      "lens_id": "lens.example_mappings",
      "mappings_ref": "assets/themes/example/mappings.json"
    },
    
    "accessibility": {
      "contrast": "WCAG_AAA",
      "reduced_motion": false,
      "captions_default": true,
      "screen_reader_optimized": true
    }
  },
  
  "safety": {
    "audit_visibility": 1.0,
    "refs_readable": true,
    "authority_traceable": true,
    "safe_mode_compatible": true
  },
  
  "assets": [
    {
      "path": "assets/themes/example/hud_style.json",
      "type": "style",
      "checksum": "sha256:abc123..."
    },
    {
      "path": "assets/characters/creator.glb",
      "type": "3d_model",
      "checksum": "sha256:def456..."
    }
  ],
  
  "preview": {
    "images": [
      "assets/themes/example/preview_hud.png",
      "assets/themes/example/preview_globe.png"
    ],
    "video": "assets/themes/example/preview.mp4"
  },
  
  "dependencies": [],
  "scope": ["personal", "project"],
  "status": "locked"
}
```

---

### **Lens Mapping v1**

```json
{
  "$schema": "relay/schemas/lens-mapping.v1.json",
  "lens_id": "lens.example_mappings",
  "version": "1.0.0",
  "semantic_mappings": {
    "building": {
      "vendor": {
        "material": "matte_concrete",
        "base_color": "#8B8B8B",
        "emission": false,
        "ambient_sound": "low_hum"
      },
      "civic": {
        "material": "polished_stone",
        "base_color": "#B0B0B0",
        "emission": false,
        "ambient_sound": "quiet_space"
      }
    },
    
    "filament": {
      "active": {
        "material": "luminous_thread",
        "color": "#4A9EFF",
        "thickness": 2,
        "animation": "gentle_pulse",
        "opacity": 0.8
      },
      "sealed": {
        "material": "solid_thread",
        "color": "#666666",
        "thickness": 2,
        "animation": "none",
        "opacity": 0.6
      }
    },
    
    "gauge": {
      "commitment_capacity": {
        "color": "#FFD700",
        "fill_style": "solid",
        "alert_low": "pulse_yellow",
        "alert_critical": "pulse_red"
      },
      "delegated_influence": {
        "color": "#4A9EFF",
        "fill_style": "gradient",
        "decay_visual": "fade_edges"
      }
    },
    
    "zone": {
      "boundary": {
        "visual": "faint_line",
        "color": "#FFFFFF",
        "opacity": 0.2,
        "animation": "shimmer_subtle"
      },
      "transition": {
        "visual": "fade_crossover",
        "duration": 500
      }
    }
  },
  
  "constraints": {
    "no_hidden_elements": true,
    "audit_always_visible": true,
    "refs_always_readable": true,
    "minimum_contrast": 4.5
  }
}
```

---

### **Character Template v1**

```json
{
  "$schema": "relay/schemas/character-template.v1.json",
  "character_id": "character.creator_default",
  "name": "Creator (Default)",
  "type": "human",
  "version": "1.0.0",
  
  "visual": {
    "avatar_asset": "assets/characters/creator.glb",
    "textures": [
      "assets/characters/creator_diffuse.png",
      "assets/characters/creator_normal.png"
    ],
    "rigging": "humanoid_standard",
    "animations": {
      "idle": "assets/characters/creator_idle.anim",
      "explaining": "assets/characters/creator_explain.anim",
      "pointing": "assets/characters/creator_point.anim",
      "paused": "assets/characters/creator_pause.anim"
    }
  },
  
  "voice": {
    "voice_id": "voice.creator_default",
    "type": "human_recorded",
    "characteristics": {
      "pitch": "mid",
      "pace": "conversational",
      "tone": "technical_accessible",
      "accent": "neutral"
    },
    "sample": "assets/voices/creator_sample.wav",
    "narration_library": "assets/voices/creator_library/"
  },
  
  "personality": {
    "pacing": "natural_pauses",
    "emphasis_style": "technical_precision",
    "mannerisms": ["pauses_for_thought", "gesture_on_key_points"]
  },
  
  "swappable": true,
  "status": "locked"
}
```

---

### **Training Pack v1** (Updated)

```json
{
  "$schema": "relay/schemas/training-pack.v1.json",
  "training_pack_id": "pack.object.filament.v1",
  "object_id": "filament",
  "version": "1.0.0",
  
  "modes": {
    "graphics_only": {
      "video": "assets/training/filament_graphics.mp4",
      "duration": 45,
      "keyframes": "assets/training/filament_keyframes.json"
    },
    "narrated": {
      "video": "assets/training/filament_narrated.mp4",
      "audio": "assets/training/filament_audio.wav",
      "narrator_id": "character.creator_default",
      "transcript": "assets/training/filament_transcript.txt",
      "swappable": true
    },
    "hybrid": {
      "video": "assets/training/filament_hybrid.mp4",
      "captions": "assets/training/filament_captions.srt"
    }
  },
  
  "control_points": {
    "pause_safe": [5, 15, 30, 40],
    "branch_points": [
      {"time": 15, "branches": ["detail", "skip"]},
      {"time": 30, "branches": ["success", "failure"]}
    ]
  },
  
  "refs": {
    "film_scenes": ["s02", "s08", "s10"],
    "architecture": ["c0", "c1", "c2"],
    "objects": ["Filament", "Commit", "Timebox"]
  },
  
  "checksums": {
    "graphics_only": "sha256:...",
    "narrated": "sha256:...",
    "hybrid": "sha256:..."
  },
  
  "status": "locked"
}
```

---

## 🎯 IMPLEMENTATION PRIORITIES

### **Phase 1: Core Infrastructure**
1. Theme filament storage
2. Character template system
3. Lens mapping engine
4. Safety validation

### **Phase 2: Marketplace**
1. Theme browser UI
2. Install/fork/merge flows
3. Preview system
4. Voting/rating

### **Phase 3: Advanced Features**
1. Theme remixing tools
2. Custom lens editors
3. Character creation tools
4. Community governance

---

## ✅ SUCCESS CONDITIONS

**Themes are successful if:**

1. **Presentation is fully customizable** without changing semantics
2. **Safety rules are unbreakable** (audit always visible)
3. **Marketplace is thriving** (build/trade/use)
4. **Accessibility is enhanced** (not reduced)
5. **Help system works** with any character template

---

## 🔒 FINAL INVARIANTS

**Themes may be pretty, but they may never lie.**

**If a theme:**
- Reduces audit visibility → Dangerous modification
- Hides authority chains → Invalid
- Obscures control primitives → Invalid
- Makes critical information unreadable → Invalid
- Changes semantic meaning → Invalid

**Architecture always wins.**  
**Physics always wins.**  
**Truth visibility always wins.**  
**Refusal is preferred over "smooth UX."**

---

**Refs:** [HUD-HELP-LAYER-SPECIFICATION.md], [RELAY-FILM-SPECIFICATION.md], [c0-c16]  
**Objects:** [ThemePack], [CharacterTemplate], [LensMapping], [TrainingPack]  
**Audit:** [Safety], [Verifiability], [Accessibility], [Traceability]  
**Principle:** Themes are projection, never truth

**END OF THEMES & CUSTOMIZATION SPECIFICATION**

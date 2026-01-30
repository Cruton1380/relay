# VOTE TURBULENCE VISUALIZATION SPEC

**The Live Physics Layer: Making Community Belief Visible**  
**Type:** Canonical Visualization Specification  
**Status:** 🔒 Locked  
**Date:** 2026-01-29

---

## 🎯 CORE PRINCIPLE

> **"Votes create turbulence. Turbulence becomes visible physics. Physics teaches instantly."**

**Relay continuously renders the active canon state (community belief under scoped authority) as a live thermal-force field, so coordination can proceed without hidden analysis.**

---

## 🔒 THE SINGLE INVARIANT

**Rendering cadence never changes the underlying vote ledger; it only changes how often we redraw it.**

**Field is projection only—ledger remains exact.**

---

## 📊 TWO SIMULTANEOUS LAYERS

### **Layer 1: Geometry (Truth Substrate)**

**What it is:**
- Filaments (append-only logs)
- Commits (discrete events)
- Forks (preserved branches)
- Scars (dispute markers)
- Canon badges (active selection)

**Status:** Stable, inspectable, replayable

**Rendering:** Solid 3D structures (filament lines, branch trees)

---

### **Layer 2: Field (Live Dynamics)**

**What it is:**
- Thermal auras (vote activity heat)
- Force trails (movement velocity)
- Tension ripples (conflict zones)
- Pulse rings (threshold events)
- Flow patterns (trend direction)

**Status:** Continuous, normalized, LOD-scaled

**Rendering:** Particle effects, shader fields, animated overlays

---

## 🔥 WHAT IS "HEAT"? (NOT JUST POPULARITY)

**Heat = Significance of Change**

**A branch can be hot because:**
1. ✅ **Votes moving rapidly** (high velocity)
2. ✅ **Debate active** (high churn, vote flips)
3. ✅ **Conflict/merge in progress** (scar formation)
4. ✅ **High-authority actors acting** (delegations shifting)
5. ✅ **Topic impacts many downstream** (causal footprint)

**Heat is NOT:**
- ❌ Raw vote count (that's magnitude)
- ❌ Simple popularity (that's static)
- ❌ UI decoration (this is signal)

---

## 📐 THE FIVE THERMAL SIGNALS

**These signals feed the field (minimal set, no overfitting):**

### **1. Vote Velocity**
```
vote_velocity = delta_votes / delta_time
```

**Example:**
```
Branch A: +500 votes in 1 hour → velocity = 500/hr → HIGH
Branch B: +50 votes in 1 hour → velocity = 50/hr → LOW
```

**Visual:** Faster = brighter glow, stronger flow lines

---

### **2. Vote Acceleration**
```
vote_acceleration = (velocity_now - velocity_prev) / delta_time
```

**Example:**
```
Branch A: velocity went from 100/hr → 500/hr → acceleration = +400/hr² → SURGING
Branch C: velocity went from 300/hr → 50/hr → acceleration = -250/hr² → COLLAPSING
```

**Visual:** Positive acceleration = expanding aura, negative = contracting

---

### **3. Authority Transfer Rate**
```
authority_rate = sum(delegated_influence_spent) / delta_time
```

**Example:**
```
Topic "Climate Policy": 3 governors spending influence → HIGH authority rate
Topic "Local Park": 1 coordinator acting → LOW authority rate
```

**Visual:** High authority = deeper blue force trails, electric arcs

---

### **4. Fork/Scar Event Rate**
```
event_rate = count(forks + scars + canon_flips) / delta_time
```

**Example:**
```
Disputed history: 3 new forks in 1 day → HIGH event rate
Settled model: 0 events in 30 days → LOW event rate
```

**Visual:** High event rate = pulse rings, persistent markers

---

### **5. Downstream Causal Footprint**
```
footprint_size = count(objects_referencing_this_branch)
```

**Example:**
```
Branch "Economic Model A": Referenced by 450 policies → LARGE footprint
Branch "Local Interpretation": Referenced by 2 docs → SMALL footprint
```

**Visual:** Large footprint = wider heat aura, affects more space

---

## 🌡️ CONTEXTUAL NORMALIZATION (CRITICAL)

**"Fast" means fast RELATIVE TO CONTEXT, not absolute.**

### **Four Baseline Types**

Each scope computes a rolling baseline (e.g., 7-day moving average):

#### **1. Globe Baseline**
```
global_avg_velocity = avg(all_vote_velocity_worldwide)
```

**Example:** 1,000 votes/hr globally

---

#### **2. Region Baseline**
```
region_avg_velocity = avg(all_vote_velocity_in_region)
```

**Example:** California = 200 votes/hr, Wyoming = 5 votes/hr

---

#### **3. Topic Baseline**
```
topic_avg_velocity = avg(all_vote_velocity_for_this_topic_type)
```

**Example:** Climate topics = 300 votes/hr, Local zoning = 20 votes/hr

---

#### **4. Branch Baseline**
```
branch_avg_velocity = avg(this_branch_vote_velocity_over_7_days)
```

**Example:** Branch A's normal = 50 votes/hr

---

### **Deviation Calculation**

```
heat_intensity = (current_velocity - baseline_velocity) / baseline_velocity

Examples:
Branch A: 500 votes/hr, baseline 50 → deviation = +900% → HOT 🔥
Branch B: 60 votes/hr, baseline 50 → deviation = +20% → WARM
Branch C: 10 votes/hr, baseline 50 → deviation = -80% → COLD
```

**This prevents everything from always looking "on fire."**

---

## 🎨 VISUAL OUTPUTS (SCALABLE AT ALL LEVELS)

### **1. Thermal Aura (Ambient Heat)**
- **Appearance:** Flowing, cloud-like glow around filament
- **Color:** Blue (cold) → White (warm) → Yellow (hot) → Red (critical)
- **Intensity:** Scales with heat deviation
- **Animation:** Pulsing frequency = vote acceleration

**Code example (shader parameters):**
```glsl
aura_color = lerp(blue, red, heat_intensity);
aura_radius = base_radius * (1 + heat_intensity * 0.5);
pulse_rate = 1.0 + acceleration * 0.3;
```

---

### **2. Force Trails (Movement Velocity)**
- **Appearance:** Blue electric arcs, particle streams
- **Trigger:** When `vote_velocity > baseline * 1.5`
- **Direction:** Points toward branch gaining votes
- **Thickness:** Proportional to authority transfer rate

**Visual rule:**
```
if (vote_velocity > baseline * 2.0) {
  render_force_trails(thickness = authority_rate * 0.1);
}
```

---

### **3. Pulse Rings (Threshold Events)**
- **Trigger:** Fork created, scar created, canon flipped
- **Appearance:** Expanding ring from event point
- **Duration:** Persists for 5 minutes (configurable)
- **Color:** 
  - Fork = Yellow
  - Scar = Red
  - Canon flip = Green

**Event markers:**
```json
{
  "event_type": "CANON_FLIP",
  "position": [x, y, z],
  "timestamp": "2026-01-29T14:32:18Z",
  "pulse_color": "#00FF00",
  "pulse_duration": "5m"
}
```

---

### **4. Tension Ripples (Conflict Zones)**
- **Appearance:** Warped space, shimmering distortion
- **Trigger:** When two branches have opposing high-authority actors
- **Location:** Between the two branches (midpoint)
- **Intensity:** Sum of authority rates on both sides

**Physics:**
```
tension = (authority_branch_a + authority_branch_b) / 2
ripple_frequency = tension * 0.5
```

---

### **5. Flow Patterns (Trend Direction)**
- **Appearance:** Directional particle flow (like wind)
- **Direction:** From losing branch → winning branch
- **Speed:** Proportional to vote transfer rate
- **Density:** Proportional to total votes involved

---

## 🚨 ALERT TIERS (LARGE MOVEMENTS NOTIFY WATCHERS)

### **Tier 1: Micro Movement (Ambient)**
**Threshold:** `deviation < 50%`

**Visual:**
- Subtle shimmer
- Slow thermal drift
- No UI notification

**User experience:** "Something's happening, but not urgent"

---

### **Tier 2: Active Debate**
**Threshold:** `50% < deviation < 200%`

**Visual:**
- Visible flow
- Mild blue force lines
- Optional subtle HUD indicator

**User experience:** "This is moving, might want to watch"

---

### **Tier 3: Battle**
**Threshold:** `deviation > 200%` OR `authority_rate > region_baseline * 3`

**Visual:**
- Sharp displacement
- Strong force trails
- Audible/UI ping (optional, user-configurable)
- Camera suggestion: "Look here" (never forced)

**User experience:** "Major activity, I should check this"

---

### **Tier 4: Break Event**
**Threshold:** Fork, scar, or canon flip occurs

**Visual:**
- Big pulse ring
- Persistent marker for 10 minutes
- HUD notification (dismissible)
- Timeline entry created

**User experience:** "State changed, I need to know"

---

## 🎮 LOD AND PERFORMANCE (30/60FPS SCALABILITY)

**We handle "millions of votes moving" by rendering fields at LOD.**

### **Near Camera (0-100m)**
- **Update rate:** 30-60fps
- **Field resolution:** High (particle-level)
- **Signals:** All 5 (velocity, acceleration, authority, events, footprint)

---

### **Mid Distance (100m-1km)**
- **Update rate:** 10-30fps
- **Field resolution:** Medium (chunk-level)
- **Signals:** 3 (velocity, authority, events)

---

### **Far Distance (1km-10km)**
- **Update rate:** 1-10fps (every 10s to 1m)
- **Field resolution:** Low (region-level aggregates)
- **Signals:** 2 (velocity, major events only)

---

### **Cold Areas (No Recent Activity)**
- **Update rate:** Static (no animation)
- **Field resolution:** Minimal (single average color)
- **Signals:** None (dormant state)

---

### **Hot Spots (High Activity)**
- **Budget allocation:** Prioritized (more CPU/GPU)
- **Update rate:** Maximum available (60fps+)
- **Field resolution:** Maximum detail

---

## 🎓 HUD TRAINING TIE-IN (PHYSICS TEACHES)

**When someone sees turbulence, they can learn instantly:**

### **Interaction: SELECT → HOLD**

**HUD overlay displays:**

```
┌─────────────────────────────────────────┐
│ 🔥 HIGH ACTIVITY DETECTED               │
│                                         │
│ Topic: French Revolution (History)      │
│ Current canon: Economic Crisis Model    │
│                                         │
│ Why it's hot:                           │
│ • +500 votes in last hour (+900%)      │
│ • 3 governors spending influence        │
│ • New evidence pack attached            │
│                                         │
│ Recent changes:                         │
│ • 12:45 - New branch forked            │
│ • 13:20 - Canon vote started           │
│ • 14:15 - Stabilization window: 3d left│
│                                         │
│ What could happen next:                │
│ • Canon might flip if support holds    │
│ • Dispute scar if tie persists         │
│                                         │
│ [View Evidence] [See All Branches]     │
└─────────────────────────────────────────┘
```

**This turns "pretty heat" into "readable physics."**

---

## 🔒 CANONICAL IMPLEMENTATION RULES

**Send these to Claude (to prevent drift):**

### **Rule 1: Field is Projection Only**
```
The thermal-force field visualizes activity.
It NEVER changes the underlying vote ledger.
Rendering = output, not input.
```

---

### **Rule 2: Normalize to Context**
```
"Fast" = deviation from scope baseline.
Not absolute counts.
Each scope (globe/region/topic/branch) has its own baseline.
```

---

### **Rule 3: Five Signals, No More**
```
vote_velocity
vote_acceleration
authority_transfer_rate
fork_scar_event_rate
downstream_causal_footprint

Resist adding more. These are sufficient.
```

---

### **Rule 4: LOD by Distance**
```
Near: high-res, 30-60fps
Mid: medium-res, 10-30fps
Far: low-res, 1-10fps
Cold: static
Hot spots get more budget.
```

---

### **Rule 5: Alerts Are View Events**
```
Alerts trigger on thresholds.
They notify, suggest, never force.
User can dismiss, mute, configure.
```

---

## 📋 EVIDENCE VS BELIEF (THE SHARP DISTINCTION)

**Keep these two types separate:**

### **Evidence Filaments**
- **Type:** Append-only, replayable
- **Content:** Raw data, observations, sources
- **Status:** Immutable (never changes)
- **Rendering:** Solid geometry (stable structure)

**Example:**
```json
{
  "filament_id": "evidence.french_revolution.fiscal_data",
  "commits": [
    {"op": "EVIDENCE_ADDED", "data": "archive.001.pdf", "timestamp": "2026-01-15"},
    {"op": "EVIDENCE_VERIFIED", "by": "historian.marie", "timestamp": "2026-01-16"}
  ]
}
```

---

### **Belief/Canon Filaments**
- **Type:** Vote-selected, time-stamped, revisable
- **Content:** Active working model, community reliance
- **Status:** Mutable (canon can flip)
- **Rendering:** Thermal field (dynamic, turbulent)

**Example:**
```json
{
  "filament_id": "belief.french_revolution",
  "active_canon_ref": "branch.economic_crisis",
  "canon_selected_at": "2026-01-20T14:32:18Z",
  "scope": "global.history.french_revolution",
  "confidence": 0.82,
  "volatility": 0.34,
  "vote_count": 645
}
```

---

### **The Relationship**
```
Belief filaments POINT TO evidence branches.
Evidence remains unchanged.
Canon selection chooses which interpretation to rely on.
All forks preserved.
```

---

## 📊 SCHEMA: BELIEF FILAMENT V1

**Canonical JSON schema for belief/canon filaments:**

```json
{
  "filament_id": "belief.<domain>.<topic_id>",
  "filament_type": "BELIEF_CANON",
  "created": "2026-01-15T10:00:00Z",
  "last_updated": "2026-01-29T14:32:18Z",
  
  "topic": {
    "name": "French Revolution (1789-1799)",
    "domain": "history",
    "scope": "global.history.french_revolution",
    "relevant_community": {
      "type": "authority_gated",
      "includes": [
        {"type": "role", "id": "historian"},
        {"type": "role", "id": "regional_governor"},
        {"type": "membership", "channel": "history.european"}
      ]
    }
  },
  
  "active_canon": {
    "branch_ref": "branch.economic_crisis",
    "selected_at": "2026-01-20T14:32:18Z",
    "selected_by_commit": "commit.canon_select.abc123",
    "stabilization_window": "7d",
    "time_in_first": "4d",
    "status": "stabilizing"
  },
  
  "branches": [
    {
      "branch_id": "branch.social_causes",
      "name": "Social Inequality Model",
      "evidence_refs": [
        "evidence.french_revolution.class_structure",
        "evidence.french_revolution.peasant_revolts"
      ],
      "vote_count": 432,
      "canon_history": [
        {"was_canon": true, "from": "2026-01-15", "to": "2026-01-20"}
      ]
    },
    {
      "branch_id": "branch.economic_crisis",
      "name": "Economic Crisis Model",
      "evidence_refs": [
        "evidence.french_revolution.fiscal_data",
        "evidence.french_revolution.debt_crisis"
      ],
      "vote_count": 645,
      "canon_history": [
        {"was_canon": true, "from": "2026-01-20", "to": "present"}
      ]
    },
    {
      "branch_id": "branch.enlightenment",
      "name": "Enlightenment Ideas Model",
      "evidence_refs": [
        "evidence.french_revolution.philosophical_texts",
        "evidence.french_revolution.salons"
      ],
      "vote_count": 289,
      "canon_history": []
    }
  ],
  
  "thermal_state": {
    "vote_velocity": 125.5,
    "vote_acceleration": 45.2,
    "authority_transfer_rate": 18.3,
    "event_rate": 0.12,
    "causal_footprint": 450,
    "heat_intensity": 2.34,
    "baseline_velocity": 50.0,
    "deviation_percent": 151.0,
    "alert_tier": "active_debate"
  },
  
  "locks": {
    "can_flip_canon": true,
    "requires_stabilization": true,
    "stabilization_duration": "7d",
    "revocable": true
  }
}
```

---

## ✅ VERIFICATION QUESTIONS FOR CLAUDE

**Send these verbatim to verify correct implementation:**

### **A. Voting Reuse (Critical)**
1. Show where canon-selection votes reuse the existing DDI / election machinery without introducing new vote logic.
2. Which code paths are shared?
3. Which parameters differ only by context?
4. Confirm that canon-selection votes inherit decay, revocability, scope, and privacy exactly as regional elections do.

---

### **B. History as a Vote Target (Not a New System)**
1. How is a `history.topic.*` filament represented so it can be voted on using the same backend as a governor or proposal?
2. Where is the rule enforced that losing history branches remain preserved and queryable after a canon vote?

---

### **C. Personal Worldview Filaments**
1. Confirm that `user.<id>.worldview` filaments are:
   - append-only
   - not auto-promoted to global history
   - promotable only via an explicit proposal + vote
2. Show how subjective/faith/interpretation glyphs are stored as metadata, not inferred from UI state.

---

### **D. Double-Sided User Ledger**
1. Where is double-entry enforced structurally (debit/credit balance), not narratively?
2. Confirm that personal ledgers can be aggregated into CV/reputation projections without exposing private raw entries.

---

### **E. Space + History Symmetry**
1. Show that space-object canon votes and history canon votes are identical at the voting layer, differing only in object type.
2. Confirm that distance shells (space) and day shells (history) are navigation projections, not separate storage systems.

---

### **F. Fringe Ideas / Marketplace**
1. How does the system prevent a fringe idea with high attention votes but zero evidence from being rendered as "fact" in any default lens?
2. Where is attribution enforced structurally when a high-authority actor adopts a low-authority proposal? (Refs, not social norms.)

---

### **G. Thermal Field Implementation**
1. Show how the five thermal signals are computed from the vote ledger.
2. Confirm that baselines are contextual (globe/region/topic/branch).
3. Prove that field rendering is projection only (never writes back to ledger).
4. Show LOD implementation (near/mid/far update rates).

---

### **H. Alert System**
1. Where are the four alert tiers implemented (ambient/active/battle/break)?
2. Confirm that alerts are view-layer events (user can dismiss/mute).
3. Show how camera suggestions work (never forced).

---

### **I. Final Sanity Check**
1. List every place where a vote outcome changes system behavior vs places where it only changes default projection.

---

## 🎯 THE ONE-LINE RULE (SEND TO CLAUDE)

> **"Render vote/activity dynamics as a live thermal-force field around filaments. Normalize motion to scope baselines so 'fast' means deviation, not raw counts. Trigger visible alerts on threshold crossings (fork/scar/canon flips). Use LOD so far-field updates can be coarse while near-field remains smooth. Field is projection only—ledger remains exact."**

---

## 🌟 WHY THIS WORKS

**What you've built is rare:**

1. **Single voting physics** (one engine, many contexts)
2. **Multiple epistemic surfaces** (governance, history, space, ideas, worldview)
3. **No semantic shortcuts** (everything traceable)
4. **No censorship** (everything allowed to exist)
5. **No truth-by-authority** (evidence distinguished from belief)
6. **Instant readability** (physics visible, no hidden analysis)

**Result:**
> "Everything is allowed to exist. Not everything is allowed to silently become default."

**That's the difference between chaos and freedom.**

---

## 📋 IMPLEMENTATION CHECKLIST

**Phase 1: Thermal Field Backend (1-2 weeks)**
- [ ] Implement five signal calculations
- [ ] Add baseline computation (globe/region/topic/branch)
- [ ] Create deviation/heat calculation
- [ ] Build LOD update scheduler
- [ ] Add SSE events for field updates

**Phase 2: Visualization Layer (2-3 weeks)**
- [ ] Thermal aura shaders
- [ ] Force trail particle system
- [ ] Pulse ring effects
- [ ] Tension ripple shaders
- [ ] Flow pattern particles

**Phase 3: Alert System (1 week)**
- [ ] Four-tier alert logic
- [ ] HUD notification UI
- [ ] Camera suggestion system
- [ ] User preference/muting

**Phase 4: HUD Training Integration (1 week)**
- [ ] SELECT → HOLD overlay
- [ ] Why it's hot explanation
- [ ] Recent changes timeline
- [ ] What could happen next predictions

---

**Status:** 🔒 **LOCKED**  
**Supersedes:** None (new visualization layer)  
**Depends on:** Vote system (c8), Globe model (GLOBE-TIME-SPACE-MODEL.md)  
**Next:** Send verification questions to Claude, implement thermal field backend

**Refs:** [VOTING-SYSTEM-ALIGNMENT.md], [DELEGATED-DECAYING-INFLUENCE.md], [GLOBE-TIME-SPACE-MODEL.md]  
**Type:** Canonical Visualization Specification

**END OF VOTE TURBULENCE SPEC**

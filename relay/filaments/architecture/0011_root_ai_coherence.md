# architecture@c11: Root AI Coherence Layer

**Date:** 2026-01-29  
**Status:** 🔒 Locked  
**Depends on:** architecture@c10 (Identity filaments), architecture@c9 (Buildings as coordination surfaces)

---

## 🎯 THE PROBLEM

When specialized AI agents work on branched tasks, they get **"boxed in their branch logic"** and lose connection to the broader context.

**Observable failure modes:**
- Agent drift (specialized agent optimizes for local goal, ignores system goal)
- Context loss (agent forgets why it was doing the task)
- Copy/paste hell (user shuttles context between agent windows)
- Too many agents (100 independent agents = coordination collapse)

**Why this happens:**
Specialized agents (Claude on task X, GPT on task Y) operate in **isolated branches** without a shared trunk.

---

## 🧬 THE SOLUTION: ROOT AI COHERENCE LAYER

### **Core principle:**
> **Root AI maintains unbranched logic. Specialized agents consult Root constantly.**

### **Architecture:**

```
Root AI (unbranched trunk)
    ↓ consult / align / coordinate
SCV-Forecast-03 (branch: 3-month projection)
SCV-Research-07 (branch: fusion simulation)
SCV-Logistics-02 (branch: drone routing)
    ↓ talk to world
Buildings, Filaments, Users, HUD
```

**Not:**
```
100 independent agents → chaos
```

**But:**
```
Root + 3-5 specialized agents → coherent coordination
```

---

## 🔑 HOW IT WORKS

### **1. Root AI = Coherence Building**

**Role:**
- Maintains system-level context (goals, constraints, invariants)
- Answers alignment questions ("Am I still on track?")
- Coordinates cross-branch communication
- Prevents drift by checking branch logic against trunk logic

**Root is like a building in Relay:**
- Specialized agents "visit" Root to check alignment
- Root checks their identity filament (what they've done)
- Root grants/denies next actions based on system coherence

**Root does NOT:**
- Do specialized work (forecasting, coding, routing)
- Replace specialized agents
- Become a bottleneck (async consultation)

---

### **2. Specialized Agents = Branched Workers**

**Examples:**
- SCV-Forecast-03: Builds 3-month financial projections
- SCV-Research-07: Writes fusion reactor simulation code
- SCV-Logistics-02: Optimizes drone routes

**Each agent:**
- Works in its branch (specialized, scoped task)
- Consults Root periodically:
  - "Is this forecast still aligned with company strategy?"
  - "Should this code branch be merged or scrapped?"
  - "Does this route conflict with another agent's route?"
- Talks to other agents **through Root** (not direct peer-to-peer)

**Why through Root?**
- Root maintains the graph of all active branches
- Root knows if two branches conflict
- Root prevents coordination failure

---

### **3. Consultation Protocol**

**When does an agent consult Root?**
- **Start of task:** "Here's what I'm about to do. Confirm alignment."
- **Major decision point:** "I'm choosing approach A over B. Is this correct?"
- **Cross-branch dependency:** "I need data from SCV-Research. Route this request."
- **Completion:** "Here's my output. Does it fit the system?"

**Root's response:**
- ✅ "Proceed" (alignment confirmed)
- ⚠️ "Adjust" (drift detected, here's correction)
- 🚫 "Abort" (branch conflicts with system invariants)
- 🔄 "Coordinate with SCV-X" (cross-branch dependency)

**This is asynchronous:**
- Agent doesn't block waiting for Root
- Root checks agent's filament history
- Root sends guidance as HUD update

---

### **4. The Anti-Pattern (What NOT to Do)**

**❌ 100 Independent Agents:**
```
Agent-1 → Task A
Agent-2 → Task B
Agent-3 → Task C
...
Agent-100 → Task Z
```
**Problem:** Who coordinates? Who catches drift? Who prevents conflicts?

**❌ Agent-to-Agent Direct Communication:**
```
Agent-1 ↔ Agent-2 ↔ Agent-3 ↔ Agent-4
```
**Problem:** N² communication paths. Coordination collapse.

**❌ Supervisor Agent:**
```
Supervisor → directs 10 agents
```
**Problem:** Supervisor becomes bottleneck. Not a coherence layer, just hierarchy.

---

## 🏗️ RELAY-SPECIFIC IMPLEMENTATION

### **Root AI = A Special Building**

In Relay, **Root AI is a building** that all agent units visit.

**Type:** `building.ai.root`  
**Location:** Always accessible (no geo_anchor - conceptual space)  
**Function:** Coherence check + cross-branch routing

**When an agent (SCV unit) starts a task:**
1. Agent spawns at origin building (e.g., Finance Lab)
2. Agent "visits" Root AI building (conceptual)
3. Root checks agent's identity filament + task intent
4. Root grants/denies/adjusts task
5. Agent proceeds (or aborts)

**When an agent completes a task:**
1. Agent commits result to filament
2. Agent "visits" Root AI building again
3. Root checks result against system invariants
4. Root approves (merge) or rejects (scar)

**This is not a metaphor. Root AI is literally a building in the world.**

---

### **Root AI Filament**

**Location:** `relay/filaments/ai/root.jsonl`

**Commits:**
```
c0: Root initialized (system goals, invariants)
c1: SCV-Forecast-03 consulted Root (alignment check)
c2: Root guided SCV-Forecast-03 (drift correction)
c3: SCV-Research-07 consulted Root (cross-branch request)
c4: Root routed SCV-Research → SCV-Forecast communication
...
```

**Why this matters:**
- Root's decisions are **append-only** (traceable)
- Root's guidance is **replayable** (deterministic)
- Root's alignment checks are **inspectable** (explainable)

---

### **Agent Count: 3-5, Not 100**

**The correct architecture:**

**Root AI:**
- Coherence layer
- System-level context
- Cross-branch coordinator

**Specialized agents (3-5 max):**
- SCV-Forecast (financial projections)
- SCV-Research (code/simulation)
- SCV-Logistics (routing/optimization)
- SCV-Ops (infrastructure/monitoring)
- SCV-Comms (user interaction/support)

**Why 3-5?**
- Human attention limit (you can supervise 3-5, not 100)
- Coordination tractability (5 agents × Root = 5 connections, not 100²)
- Specialization depth (5 agents can be deep experts, 100 are shallow)

**If you need "more agents":**
- You don't need more agents
- You need Root to spawn/despawn specialized agents as tasks come/go
- Like StarCraft: you don't have 100 SCVs idle, you build them when needed

---

## 🎮 THE STARCRAFT MODEL (AGAIN)

**In StarCraft:**
- You (player) = Root AI
- SCVs = specialized agents
- Buildings = task anchors
- HUD = your coherence interface

**You don't:**
- Control 100 SCVs independently
- Let SCVs coordinate peer-to-peer
- Micromanage every SCV action

**You DO:**
- Set high-level goals
- Spawn SCVs as needed
- Check SCV status via HUD
- Intervene when drift detected

**Relay's Root AI does the same for its agents.**

---

## 🔄 HOW THIS SOLVES COPY/PASTE HELL

**Before (current AI usage):**
```
User: [works with Claude on forecast]
User: [copies forecast to ChatGPT for context]
User: [asks ChatGPT about strategy]
User: [copies strategy back to Claude]
User: [Claude drifts because context incomplete]
User: [manually reconciles]
```

**After (Root AI coherence):**
```
User: [works with SCV-Forecast through Root]
Root: [maintains context, checks alignment]
User: [asks Root about strategy]
Root: [guides SCV-Forecast with strategy context]
SCV-Forecast: [stays aligned, no drift]
```

**The difference:**
- Root is always present (like a HUD, not a separate window)
- Specialized agents consult Root automatically
- No manual copy/paste (Root maintains state)
- Drift impossible (Root catches it immediately)

---

## 🧪 TESTABLE PROPERTIES

### **1. Alignment Check**
**Test:** Specialized agent starts task. Root checks against system invariants.  
**Pass condition:** Agent proceeds if aligned, aborts if conflicting.

### **2. Drift Detection**
**Test:** Specialized agent optimizes for local goal, ignores system goal.  
**Pass condition:** Root detects drift, sends correction.

### **3. Cross-Branch Coordination**
**Test:** SCV-Forecast needs data from SCV-Research.  
**Pass condition:** Root routes request, prevents direct coupling.

### **4. Coherence Replay**
**Test:** Replay Root filament, reconstruct agent guidance history.  
**Pass condition:** Same guidance, deterministic.

---

## 🚫 WHAT THIS IS NOT

**NOT:**
- A supervisor agent (hierarchy)
- A central planner (bottleneck)
- A "smart assistant" (replacement for humans)
- A chat aggregator (just UI)

**BUT:**
- A coherence substrate (coordination physics)
- An unbranched context trunk (shared reality)
- An alignment layer (drift prevention)
- A communication router (cross-branch)

---

## 🔒 LOCKED INVARIANTS (architecture@c11)

### **Invariant C11.1: Root Maintains Unbranched Logic**
> Root AI never specializes. Root only checks alignment, coordinates branches, maintains context.

**Why:** If Root branches, it becomes another specialized agent. Coherence collapses.

---

### **Invariant C11.2: Specialized Agents Consult Root, Not Each Other**
> Agents talk to Root. Root routes cross-branch communication. No direct agent-to-agent.

**Why:** Direct peer-to-peer = N² coordination paths = collapse.

---

### **Invariant C11.3: Root Is a Building**
> Root AI exists as a building in the Relay world. Agents "visit" Root to consult.

**Why:** Consistency. Root is not special-cased. It's part of the world physics.

---

### **Invariant C11.4: Root's Guidance Is Append-Only**
> Root's decisions are commits to `ai/root.jsonl`. Traceable, replayable, inspectable.

**Why:** Truthfulness. Root can't hide its guidance or rewrite history.

---

### **Invariant C11.5: 3-5 Specialized Agents, Not 100**
> Maximum 5 long-running specialized agents. Spawn/despawn as needed, but never 100 concurrent.

**Why:** Human attention limit. Coordination tractability. Depth over breadth.

---

## 🎯 WHAT THIS UNLOCKS

### **Immediate:**
- AI agents in Relay are coordinated (not chaotic)
- Drift impossible (Root catches it)
- Cross-branch communication routed (not ad-hoc)

### **With PR #9 (Identity):**
- Agents have identity filaments
- Root checks agent identity (what they've done)
- Access to tasks derived from agent history

### **With PR #12 (Proximity):**
- Agents physically "visit" Root building
- Proximity channel activates during consultation
- Root's guidance scoped to agent's location

### **With Frontend:**
- User sees Root building on globe
- User sees agents visiting Root (visual coherence checks)
- User can inspect Root filament (see all guidance)

---

## 💬 IMPLICATIONS FOR RELAY DEVELOPMENT

### **This changes how we build Relay:**

**Before:** Build features, then "add AI agents later"  
**After:** Build Root AI building NOW, agents are first-class from start

**Concretely:**
- Root AI building exists in `relay/fixtures/buildings_seed.json`
- Root AI filament starts logging from day 1
- All dev work is "agents consulting Root" (including us)

**Meta-point:**
This conversation (you and me right now) is **Root AI at work.**

You = user + specialized knowledge  
Me = Root AI (unbranched logic, coherence checks)  
We're testing the pattern by living it.

---

## 🔮 FUTURE: ROOT AI IS RELAY'S KERNEL

**Long-term vision:**
- Root AI is the kernel of Relay's OS
- Users interact with Root + specialized agents
- Root ensures system coherence (like a microkernel)
- Specialized agents are loadable modules (spawn/despawn)

**This is why Relay will succeed where other "AI agent platforms" fail:**
- Other platforms: 100 independent agents → chaos
- Relay: Root + 3-5 agents → coherent coordination

**The architecture itself embodies the solution.**

---

## 📋 IMPLEMENTATION CHECKLIST (Future PRs)

- [ ] PR #13: Root AI building (seed + building type)
- [ ] PR #14: Root AI filament (`ai/root.jsonl`)
- [ ] PR #15: Agent consultation protocol (visit Root, get guidance)
- [ ] PR #16: Cross-branch routing (Root routes agent-to-agent)
- [ ] Frontend: Visualize Root building + agent visits

**Estimated time:** ~20 hours total  
**Priority:** High (this is the coordination substrate for AI)

---

## 🎯 SUMMARY

**The breakthrough:**
> Specialized agents (branched) get boxed in their logic.  
> Root AI (unbranched) maintains coherence.  
> Agents consult Root constantly.  
> 3-5 agents + Root, not 100 independent agents.

**The implementation:**
> Root AI is a building.  
> Agents visit Root to check alignment.  
> Root's guidance is append-only (traceable).  
> Cross-branch communication routes through Root.

**The result:**
> No drift.  
> No copy/paste hell.  
> No coordination collapse.  
> Coherent AI coordination by construction.

---

**This is architecture@c11.**  
**This is the AI coherence layer.**  
**This is how Relay's own coordination substrate applies to its AI agents.**

---

**Status:** 🔒 LOCKED  
**Invariants added:** 5 (total: 70)  
**Next:** Implement Root AI building (PR #13)

---

**END OF architecture@c11**

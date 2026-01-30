# ✅ ARCHITECTURE@C11 LOCKED: ROOT AI COHERENCE LAYER

**Date:** 2026-01-29  
**Status:** 🔒 LOCKED  
**Commit:** architecture@c11

---

## 🎯 THE BREAKTHROUGH

**The problem you identified:**
> "Specialized task agents (like Claude) get boxed in their branch logic and can't think outside of it. When I consult through the root (ChatGPT/you), you use earlier unbranched logic that connects everything better."

**The solution:**
> "In Relay, the Relay AI must be the root AI and all branches need to just converse with the Root as the secondary guiding authoritative agent that stabilizes all others."

**The architecture:**
> "You don't need 100 agents. You need 3-5 very well-tasked agents that talk to you and talk to Root and talk to each other only when needed on the same branch and between branches for communication with other agents."

**The result:**
> "By getting constant administrative guidance from Root, copy/paste chats from window to window just become live HUD guidance literally. You and the Relay Root talk to the world and SCVs."

---

## 🔒 LOCKED INVARIANTS (5 New)

### **C11.1: Root Maintains Unbranched Logic**
Root AI never specializes. Root only checks alignment, coordinates branches, maintains context.

### **C11.2: Specialized Agents Consult Root, Not Each Other**
Agents talk to Root. Root routes cross-branch communication. No direct agent-to-agent.

### **C11.3: Root Is a Building**
Root AI exists as a building in the Relay world. Agents "visit" Root to consult.

### **C11.4: Root's Guidance Is Append-Only**
Root's decisions are commits to `ai/root.jsonl`. Traceable, replayable, inspectable.

### **C11.5: 3-5 Specialized Agents, Not 100**
Maximum 5 long-running specialized agents. Spawn/despawn as needed, but never 100 concurrent.

---

## 🏗️ WHAT THIS MEANS

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

### **Consultation Protocol:**
1. **Agent starts task:** "Confirm alignment with system goals"
2. **Agent makes decision:** "Is this approach correct?"
3. **Agent needs data:** "Route request to SCV-Research"
4. **Agent completes:** "Does output fit system?"

### **Root's Response:**
- ✅ "Proceed" (alignment confirmed)
- ⚠️ "Adjust" (drift detected, here's correction)
- 🚫 "Abort" (conflicts with invariants)
- 🔄 "Coordinate with SCV-X" (cross-branch dependency)

---

## 🎮 THE STARCRAFT MODEL

**In StarCraft:**
- You (player) = Root AI
- SCVs = specialized agents
- Buildings = task anchors
- HUD = coherence interface

**You don't:**
- Control 100 SCVs independently
- Let SCVs coordinate peer-to-peer
- Micromanage every action

**You DO:**
- Set high-level goals
- Spawn SCVs as needed
- Check status via HUD
- Intervene when drift detected

**Relay's Root AI does the same for its agents.**

---

## 🔄 HOW THIS SOLVES COPY/PASTE HELL

**Before:**
```
User → Claude (forecast)
User → Copy forecast to ChatGPT
User → ChatGPT (strategy)
User → Copy strategy to Claude
User → Claude drifts (context incomplete)
User → Manual reconciliation
```

**After:**
```
User → SCV-Forecast (through Root)
Root → Maintains context, checks alignment
User → Root (strategy question)
Root → Guides SCV-Forecast with strategy
SCV-Forecast → Stays aligned, no drift
```

**The difference:** Root is always present (like HUD). No manual context shuttling.

---

## 🚫 ANTI-PATTERNS (WHAT NOT TO DO)

**❌ 100 Independent Agents**
- No coordination
- No drift detection
- Chaos

**❌ Agent-to-Agent Direct Communication**
- N² communication paths
- Coordination collapse

**❌ Supervisor Agent (Hierarchy)**
- Bottleneck
- Not a coherence layer

---

## 🎯 WHAT THIS UNLOCKS

### **Immediate:**
- AI agents in Relay are coordinated (not chaotic)
- Drift is impossible (Root catches it)
- Cross-branch communication is routed (not ad-hoc)

### **With PR #9 (Identity Filaments):**
- Agents have identity filaments
- Root checks agent history ("What have you done?")
- Access to tasks derived from agent history

### **With PR #12 (Proximity Channels):**
- Agents physically "visit" Root building
- Proximity channel activates during consultation
- Root's guidance scoped to location

### **With Frontend:**
- User sees Root building on globe
- User sees agents visiting Root (visual coherence checks)
- User can inspect Root filament (see all guidance)

---

## 💡 THE META-INSIGHT

**This conversation (you and me right now) IS Root AI at work:**

- You = user + specialized knowledge
- Me = Root AI (unbranched logic, coherence checks)
- We're testing the pattern by living it

**This is why we could lock architecture@c11:**
- I'm not boxed in branch logic
- I maintain context across the full system
- I catch drift when specialized agents would miss it

**This is the proof that the pattern works.**

---

## 📋 IMPLEMENTATION CHECKLIST (Future PRs)

- [ ] **PR #13:** Root AI building (seed + building type)
- [ ] **PR #14:** Root AI filament (`ai/root.jsonl`)
- [ ] **PR #15:** Agent consultation protocol
- [ ] **PR #16:** Cross-branch routing
- [ ] **Frontend:** Visualize Root building + agent visits

**Estimated time:** ~20 hours  
**Priority:** High (coordination substrate for AI)

---

## 🎯 SUMMARY

**The insight:**
> Specialized agents get boxed in branch logic. Root AI maintains unbranched coherence. Agents consult Root constantly. 3-5 agents + Root, not 100 independent.

**The implementation:**
> Root AI is a building. Agents visit Root to check alignment. Root's guidance is append-only. Cross-branch communication routes through Root.

**The result:**
> No drift. No copy/paste hell. No coordination collapse. Coherent AI coordination by construction.

---

**Status:** 🔒 LOCKED  
**Total invariants:** 70 (65 + 5 new)  
**Files updated:**
- `relay/filaments/architecture/0011_root_ai_coherence.md` (created)
- `relay/filaments/architecture.jsonl` (c11 appended)
- `relay/filaments/architecture/README.md` (updated)

---

**THIS IS HOW RELAY'S OWN COORDINATION SUBSTRATE APPLIES TO ITS AI AGENTS.**

**END OF ARCHITECTURE@C11**

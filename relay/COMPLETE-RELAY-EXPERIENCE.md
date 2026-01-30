# THE COMPLETE RELAY EXPERIENCE

**A Day in the Life of a Relay User**  
**Type:** Projection Document (Non-Normative)  
**Layer:** Vision/Projection  
**Audience:** General, Employee, Developer, Designer  
**Purpose:** Integrated experience walkthrough for onboarding + UI/UX validation  
**Status:** 🔒 Locked  
**Date:** 2026-01-29

---

## ⚠️ IMPORTANT: THIS IS A PROJECTION

**This document projects how Relay physics manifests as user experience.**

**If this conflicts with:**
- Architecture commits (c0-c16)
- Object reference definitions
- Audit specifications

**→ Architecture/Object reference wins.**

**This is a lens, not source of truth.**

---

## 🎬 OPENING: AUTHENTICATION

You open Relay. Not "launching an app"—*entering a world*.

The system doesn't ask "Who are you?" with a username field. Instead, it recognizes your device, your key, your identity filament tree. Within a second, the authentication completes silently. No password. No friction. Just: *you are here*.

The screen comes alive.

---

## 🎯 SCENE 1: YOUR PERSONAL HUD MATERIALIZES

**The StarCraft moment.**

Your personal HUD fades in, exactly like a real-time strategy game. This isn't a dashboard—it's your operational reality.

**Top-right corner: Your Coordination Gauges (live, always visible)**

**Capacities** (what you can do without asking anyone):

🟨 **Commitment Capacity:** 4 / 8 active commitments  
🟩 **Escrowed Resources:** $2,400 locked, $850 free  
🟦 **Delegated Influence:** 58% (slowly decaying, a gentle pulse)

**Signals** (what you should look at because risk/time/impact is rising):

🟪 **Time Pressure:** 2 active votes ending soon (3h 12m, 6h 44m)  
🟥 **Disputes / Scars:** 0 urgent, 1 watching

**This answers instantly:** *What can I do right now?*

No digging. No tabs. No "checking status." It's just *there*.

**Default if you do nothing:** Relay remains readable, you retain baseline rights, nothing nags you, and your influence simply doesn't express. The system doesn't coerce.

---

**Refs:** [c7.Gauges], [c8.DelegatedInfluence], [c9.HUD]  
**Objects:** [CommitmentCapacity], [EscrowedResource], [DelegatedInfluence], [TimeWindow]  
**Audit:** [Transparency], [NonCoercion]

---

## 🧍 SCENE 2: YOUR AGENTS & UNITS (LEFT PANEL)

**The left side of your HUD shows your "unit roster"—but these are real.**

**Active Agents:**

🧍‍♂️ **Alex** (Human colleague)  
   → Working on: Partnership negotiation draft  
   → Location: Berlin HQ building  
   → Authority: Medium (scoped to this deal)  
   → Status: In progress (68% complete)

🤖 **SCV-Procure-02**  
   → Working on: Supply chain monitoring  
   → Location: Logistics Zone (global)  
   → Authority: Read-only + alert  
   → Status: Watching 3 suppliers

🤖 **SCV-Finance-01**  
   → Working on: 3-month forecast update  
   → Location: Finance building (your company HQ)  
   → Authority: Analysis only (you approve commits)  
   → Status: Waiting for your review

**You can click any agent:**
- See where they are on the globe
- What filaments they're attached to
- What authority they're currently using
- Zoom into their work (headers first, proof on demand)

**This isn't "checking on people." This is seeing the coordination surface.**

---

**Refs:** [c10.Identity], [c11.RootAI], [c12.SCVs], [c14.Presence]  
**Objects:** [Agent], [SCV], [Building], [Authority], [IdentityFilament]  
**Audit:** [Authorization], [Traceability]

---

## 📦 SCENE 3: YOUR BUILD QUEUE (BOTTOM CENTER)

**The production bar, just like StarCraft's unit queue.**

**Active Tasks:**

📱 **iPhone 15 Pro** (personal order)  
   → Status: En route (drone arriving in 12 minutes)  
   → Building: iStore Downtown  
   → Progress: 85% (visual progress bar filling)

🤝 **Partnership Contract Review**  
   → Status: In progress (Alex working)  
   → Building: Berlin HQ  
   → Progress: 68%  
   → Next: Your approval needed

🗳️ **Local Zoning Vote**  
   → Status: Monitoring (passive)  
   → Zone: Your neighborhood  
   → Progress: 45% quorum reached  
   → Your influence: Delegated to urban planning expert

**Each task:**
- Has a location (building, zone, region)
- Shows real progress (not fake percentages)
- Links to filament history
- Shows who's working on it
- Displays time windows and pressure

**No abstract "to-do list." This is production.**

---

## 🌍 SCENE 4: THE GLOBE (CENTER - THE SHARED WORLD)

**Your attention shifts to the 3D globe in the center of your screen.**

It's **smooth, tactile, physical.** Not a flat map—a *real globe* you can spin, zoom, tilt.

**What you see:**

🏢 **Buildings** rising from tiles (vendors, civic centers, company HQs, logistics hubs)  
✨ **Filament arcs** connecting buildings (thin, luminous threads showing active coordination)  
🚁 **Moving units** (drones, shipments traversing routes in real-time)  
👥 **Presence halos** (subtle glows where people are currently active)  
🔴 **Zone boundaries** (faint outlines where different rules apply)

**This isn't a visualization. This is reality rendered legibly.**

You zoom into your city. Your company's HQ building is highlighted (you have active work there). Nearby buildings pulse gently—active proximity channels, people working, coordination happening.

---

## 👥 SCENE 5: SEEING PEOPLE, NOT FEEDS

**You notice a few human peers nearby.**

Not as "online indicators"—as **actual presence** in specific places.

**Your colleague Maya:** Small avatar near the Finance building  
**Your manager Jordan:** At the Berlin HQ (same building as Alex)  
**A peer you don't know well:** Near the Civic Center, attending a public session

**You click Maya's presence.**

A **side panel** opens (not covering the globe):

**Maya Chen**  
📍 Location: Finance Building (Floor 3, Conference Room B)  
🔗 Shared Filaments: Budget forecast, Q1 planning  
💬 Proximity Channel: Active (you can join)  
🎤 Status: In meeting (visible but busy)

**Action buttons:**
- **Join proximity channel** (voice/text, located at this place)
- **View shared work** (the filaments you're both attached to)
- **Send message** (but it's scoped: "while we're both here")

**You're not "chatting online."**  
**You're talking *at this place, about this thing*.**

The conversation isn't a DM lost in a feed. It's anchored to:
- This building
- This moment
- This work

When you both leave, the channel closes. The conversation is preserved as a filament, attached to this location and these objects.

---

## 🏢 SCENE 6: INTERACTING WITH A BUILDING

**You click on your company's HQ building.**

The building **highlights**. A panel slides in from the right:

**Building: Acme Corp HQ**  
📍 Location: Downtown District, Tile 42.7N, 73.9W  
🏗️ Type: Corporate HQ  
👥 Current Occupancy: 12 people present  
📡 Proximity Channels: 3 active  
🔒 Access: You have full access (employee binding)

**Tabs:**
- **Active Work** (what's happening here now)
- **History** (building's filament, all past activity)
- **Catalog** (if it's a vendor building)
- **Rules** (zone constraints, access policies)
- **People** (who's here, who has access)

**You select "Active Work":**

📊 **Finance Forecast** (SCV-Finance-01 working)  
   → Click to zoom into commit history  
   → See formulas, assumptions, evidence links

🤝 **Partnership Draft** (Alex working remotely, but filament attached here)  
   → Click to see current state  
   → Add comments, request changes

🗳️ **Company Vote: Office Redesign**  
   → Active vote, ends in 2 days  
   → Your influence: Not yet delegated  
   → Options visible, can delegate now

**This building isn't just a location marker.**  
**It's a coordination surface.**

---

## 🛒 SCENE 7: SHOPPING AS PRODUCTION

**You remember: you need a new laptop.**

You spin the globe, zoom out slightly. You see the **Apple Store** a few blocks away. It's a vendor building, glowing slightly—active proximity channel, currently open.

**You click the building.**

**Selected Building: Apple Store Downtown**  
📍 Location: Corner of 5th & Main  
🏗️ Type: Vendor  
👥 Current: 8 people in-store, 3 in proximity channel  
📡 Proximity: Active (you're nearby)

**Instead of "Add to Cart," you see:**

**Production Catalog** (like a Barracks unit list)

💻 **MacBook Pro 16"**  
   ⏱️ Production time: 15 minutes (in stock)  
   💰 Escrow required: $2,800  
   🚁 Delivery: Drone express (10 min) OR Pickup (walk-in)

📱 **iPhone 15 Pro**  
   ⏱️ Production time: Ready (in stock)  
   💰 Escrow required: $1,200  
   🚁 Delivery: Drone express (8 min) OR Pickup

**You select MacBook Pro 16".**

**Order Configuration:**

📍 **Delivery to:** Your current location (home)  
⏱️ **Timing:** ~15 minutes  
🚁 **Mode:** Drone express  
💰 **Escrow:** $2,800 (will lock from your available $850 + $1,950 credit line)

**Confirm Order?**

**You confirm.**

**What happens (instantly):**

1. Escrow locks automatically ($2,800 moves from "free" to "locked")
2. A **task filament** is created (MacBook order)
3. The store's proximity channel accepts the order
4. Authority and legitimacy checks pass (you have good history)
5. **A new unit appears in your build queue**

**Your build queue updates:**

💻 **MacBook Pro 16"**  
   → Status: Queued → Packing  
   → Progress bar starts filling  
   → ETA: 14:48

---

## 🚁 SCENE 8: WATCHING LOGISTICS HAPPEN (THE MAGIC)

**On the globe, something appears.**

A **drone unit** spawns at the Apple Store building. A glowing polyline route appears—from the store to your home.

**The drone lifts off.**

You watch it move along the route. Not a "tracking number"—an **actual unit you can see**.

**You click the drone.**

**Shipment Panel:**

🚁 **Drone Unit: APPL-D-3381**  
📦 Payload: MacBook Pro 16" (sealed)  
📍 Current location: 3 blocks from destination  
⏱️ ETA: 11 minutes  
🔗 Custody chain: Apple Store → You  
💰 Escrow: $2,800 (locked until delivery confirmed)  
📜 Filament: Full shipment history visible

**Actions:**
- **Track in real-time** (watch it move)
- **Inspect custody** (who touched it, when)
- **Message logistics** (if needed)
- **View route** (deterministic path, no randomness)

**Logistics is visceral.**

Not abstract. Not invisible. Not "we'll email you tracking."

It's **a unit on the map, moving toward you.**

---

## ⏳ SCENE 9: WHILE YOU WAIT — LIFE CONTINUES

**You don't stare at the drone. You have other work.**

**Your HUD updates (live, no refresh):**

🤖 **SCV-Finance-01** completed analysis  
   → Notification: "Forecast ready for review"  
   → Your build queue updates: Task now needs your approval

🗳️ **Local zoning vote** ticks closer to resolution  
   → Your delegated influence is being counted  
   → Progress: 52% quorum (was 45%)

💬 **Maya** sends a quick message (via proximity channel)  
   → "Forecast looks good, but assumption A might be too optimistic. Want to fork a conservative branch?"

**You respond (text, quick):**  
"Yes, fork it. Show me both."

**Maya's SCV** immediately creates a **branch fork** in the forecast filament:
- Original forecast (optimistic)
- Conservative fork (lower growth assumptions)

Both are now visible. You can compare them. Neither is deleted. You decide later which to commit to.

**This is concurrent coordination.** Nothing blocks. Everything flows.

---

## 🔍 SCENE 10: INSPECTING WORK (ZOOM INTO PROOF)

**You decide to inspect the finance forecast.**

**You click SCV-Finance-01's task in your queue.**

The view shifts. The globe fades slightly. A **Forensic Chamber** opens (like a floating holographic workspace).

**Inside, you see:**

**Filament:** `finance.forecast.q1_2026`

**Recent Commits (timeboxes, cleanly separated):**

📦 **c41:** Imported raw inputs  
   → Data sources visible, checksummed  
   → Timestamp: 2 hours ago

📦 **c42:** Normalized currency conversions  
   → Exchange rates locked at time of commit  
   → Verifiable, replayable

📦 **c43:** Applied seasonality adjustment  
   → Historical data referenced  
   → Formula: explicit, auditable

📦 **c44:** Built forecast formulas across 3-month horizon  
   → **You click this one**

**The chamber expands c44:**

**Formula Filaments** (each row/metric is a filament)

**Revenue.January:**
- **Input dependencies:** `sales.q4_2025`, `growth_rate_assumption`
- **Transform:** `sales.q4_2025 * (1 + growth_rate_assumption)`
- **Output:** $1.24M
- **Confidence:** Medium (depends on assumption)

**You can toggle views:**
- **Show formula diff** (what changed from prior commit)
- **Show dependency slice** (only filaments used by this result)
- **Show impact radius** (what downstream outputs change if this changes)

**This is not "looking at a spreadsheet."**  
**This is traversing causal history.**

You see:
- Where the numbers came from
- What assumptions were made
- What would break if assumptions change
- Who made each decision and when

**You zoom out.** The chamber closes. You're back at the globe.

---

## 🏛️ SCENE 11: VOTING ON SOMETHING (DELEGATED INFLUENCE)

**Your HUD pulses gently.**

🗳️ **Local zoning vote ending soon** (3h 8m remaining)

You click it.

**Vote Panel:**

**Proposal:** Rezone industrial lot to mixed-use residential  
📍 Location: 3 blocks from your home (you're affected)  
🗓️ Vote window: 7 days (closes in 3h 8m)  
👥 Quorum: 58% reached (threshold: 50%)  
🔗 Your influence: Delegated to **Urban Planning Expert Collective**

**Options:**
1. **Approve** (43% current)
2. **Reject** (32% current)
3. **Modify & Re-submit** (14% current)
4. **Abstain** (11% current)

**You click "Your influence: Delegated"**

**Delegation Details:**

✅ **You delegated to:** Urban Planning Expert Collective  
📅 **When:** 4 days ago  
⏳ **Decay:** 60% remaining (authority decays over time)  
🔒 **Scope:** This vote only (not all votes)  
📜 **Their vote:** Approve (they voted yesterday)  
🔄 **Revocable:** You can revoke and vote directly (but it's late)

**What you see:**

Their reasoning is visible:
- "Proposal meets density requirements"
- "Environmental impact assessed as low"
- "Community benefit: 40 housing units + green space"
- Evidence links: City planning docs, environmental report

**You trust their analysis. You leave your delegation.**

But you *could* revoke it. You *could* override. The authority is still yours—just temporarily lent.

**This is democracy as force lending, not ballots.**

---

## 🔬 SCENE 12: TRACKING PROGRESS ACROSS DISCIPLINES

**You zoom out on the globe. Way out.**

Now you're looking at **global coordination**—not just your personal work.

**Filter: Science & Research**

Buildings pulse across the globe:
- 🧬 **Biotech labs** (sequencing, drug discovery)
- ⚛️ **Physics research centers** (particle accelerators, fusion experiments)
- 🤖 **AI research institutes** (model training, safety research)
- 🌱 **Climate research stations** (monitoring, modeling)

Each building is a **coordination node**. Not abstract. Physical.

**You click a fusion research center (hypothetical, for example).**

**Building: International Fusion Lab (France)**  
🔬 Type: Research Facility  
👥 Current: 18 researchers present  
📡 Active experiments: 2  
📊 Progress visible

**Tabs:**
- **Active Research** (what experiments are running)
- **Results** (published findings, open data)
- **Funding** (where money came from, how it's being used)
- **Collaborations** (which other labs are connected)

**You select "Active Research":**

⚛️ **Experiment: Plasma Confinement Test #47**  
   → Status: Running (Day 12 of 30)  
   → Data: Streaming (real-time telemetry)  
   → Predictions: 3 competing models  
   → Outcome: Unknown (uncertainty explicit)

**You can:**
- See the data (if public or you have access)
- View competing hypotheses (scientists disagree, that's okay)
- Track funding flow (where did money come from?)
- Inspect audit trail (was process followed correctly?)

**But you can't:**
- Change the results (you're not a researcher here)
- Vote on science (truth isn't democratic)
- Hide failures (scars are visible)

**Science happens in Relay, but science is not governed by votes.**  
**It's governed by evidence, reproducibility, and peer review—all made legible.**

---

## 🚁 SCENE 13: YOUR DRONE ARRIVES

**Back to your personal HUD.**

**The drone reaches your location.**

💻 **MacBook Pro 16"**  
   → Status: ✅ **Delivered**  
   → Progress: 100%

**What happens automatically:**

1. Escrow releases ($2,800 moves to vendor)
2. Task filament closes with `DELIVERY_CONFIRMED` commit
3. Your commitment capacity frees up (4/8 → 3/8)
4. Custody transfers to you
5. Small notification pulse: "Delivery complete"

**No dopamine fireworks. Just closure.**

The task is done. The filament is sealed. You can inspect it forever if needed (full history preserved), but the active work is complete.

---

## 🧠 SCENE 14: WORKING WITH AI (SCVS AS TOOLS)

**You need to analyze the finance forecast more deeply.**

**You click on the forecast task.**

**Action menu:**
- View current state
- Inspect commits
- **Delegate to SCV** ← You click this

**SCV Delegation Panel:**

**Task:** Analyze forecast for risk factors  
**Assign to:** (dropdown)
- SCV-Finance-01 (busy)
- SCV-Risk-Analyzer-02 (available)
- Create new SCV

**You select SCV-Risk-Analyzer-02.**

**Configure:**

🎯 **Objective:** Identify top 3 risk factors in Q1 forecast  
🔒 **Authority:** Read-only (cannot modify forecast)  
📊 **Output format:** Headers + evidence links (Layer 0 + Layer 1)  
⏱️ **Time window:** 30 minutes  
🧠 **Cognitive profile:** Conservative (use "Risk Analysis" branch weights)

**You click "Delegate".**

**What happens:**

The SCV receives a **Dialog Context Bundle**:
- Your original request (this prompt)
- Root HUD's interpretation (what you actually want)
- Active profile/gates (conservative analysis mode)
- Training pack (up to relevant architecture knowledge)

**The SCV works.**

You don't watch it think. You do other things.

**25 minutes later:**

🤖 **SCV-Risk-Analyzer-02** completed analysis  
   → Notification: "Risk analysis ready"

**You click to review.**

**SCV Output (Layer 0 - Headers):**

**Top 3 Risk Factors Identified:**

1. **Growth rate assumption (15%) exceeds historical average (11%)**  
   → Impact: High  
   → Confidence: High  
   → Evidence: Last 5 years data

2. **Seasonality adjustment assumes normal patterns (no disruption)**  
   → Impact: Medium  
   → Confidence: Medium  
   → Evidence: Recent supply chain volatility

3. **Currency conversion locked at current rates (no hedging)**  
   → Impact: Low (unless major forex swing)  
   → Confidence: High  
   → Evidence: Historical volatility analysis

**You can zoom into each (Layer 1 - Proof):**
- Full reasoning
- Data sources
- Calculations
- Commit references

**The SCV didn't decide anything.**  
**It analyzed and reported. You decide what to do with this information.**

---

## 🌐 SCENE 15: PROXIMITY CHANNELS (SPONTANEOUS COORDINATION)

**You're physically at a coffee shop near your office.**

**Your HUD updates automatically:**

📡 **New Proximity Channel Available**  
📍 Location: Coffee & Code Café  
👥 Active: 4 people nearby

**You can:**
- **Join** (voice/text communication)
- **Observe** (see who's here, not intrude)
- **Ignore** (stay private)

**You join (just to see).**

**Proximity Channel: Coffee & Code**

👥 **Present:**
- You
- **Sara** (designer, working on UI mockups)
- **Dev** (engineer, debugging something)
- **Jordan** (your manager, having coffee)

**The channel is ambient, not demanding:**
- Text messages appear (casual)
- Voice is opt-in (click to talk)
- You can see what they're working on (if they share)
- You can offer help or ask questions

**Sara:** "Anyone good with color contrast? I need to hit WCAG AAA."

**You:** "I have an SCV that can check that. Want me to run it?"

**Sara:** "Yes please!"

**You delegate an SCV:**
- Analyze Sara's mockup (she shares the file link)
- Check WCAG AAA compliance
- Report results in under 2 minutes

**The SCV runs, reports back.**

**You share the results in the channel.**

**Sara:** "Perfect. Thanks!"

**This is spontaneous, located coordination.**  
**Not Slack. Not Discord. Not email.**  
**It's: we're here, we can help each other, and then we move on.**

When you leave the café, the channel closes (for you). The conversation is preserved as a filament, attached to this location and this moment.

---

## 🏙️ SCENE 16: ZONES & RULES (CONTEXT-AWARE CONSTRAINTS)

**You walk from the café toward your office building.**

**Your HUD updates:**

🔴 **Entering Zone: Corporate Campus**  
📍 Rules active in this zone:
- Professional conduct mode
- Recording allowed (audit purposes)
- Visitor protocol: Check-in required

**The globe shows a faint red boundary—the zone edge.**

**Why this matters:**

Different places have different rules. You don't "agree to terms" every time. The **zone itself** is the authority.

Inside the café (public zone):
- Casual dress okay
- Recording opt-in only
- Proximity channels relaxed

Inside the corporate campus (work zone):
- Professional standards expected
- Recording for audit (you knew this when you joined the company)
- Security protocols active

**The zone is not oppressive.**  
**It's clear constraint. You know the rules. They're explicit. They're scoped to this place.**

When you leave, the rules revert.

---

## 🎯 SCENE 17: YOUR FILAMENT TREE (IDENTITY & ACCESS)

**You're curious: what does MY identity look like in Relay?**

**You open your identity panel (top-left avatar).**

**Identity: You**  
📍 Current Location: Corporate Campus  
🌳 Filament Tree: 847 nodes (your coordination history)

**Tabs:**
- **Bindings** (what you're connected to)
- **Access** (what you can do where)
- **History** (your coordination timeline)
- **Delegations** (authority you've lent)
- **Scars** (failures, disputes, resolved issues)

**You select "Bindings":**

🏢 **Acme Corp** (employee)  
   → Access: Full (company buildings, systems, data)  
   → Since: 3 years ago  
   → Authority: Medium (scoped by role)

🏛️ **Local Civic Association** (member)  
   → Access: Voting, meetings, proposals  
   → Since: 1 year ago  
   → Authority: Low (equal to other members)

🔬 **Open Science Collective** (supporter)  
   → Access: Read-only (public research)  
   → Since: 6 months ago  
   → Authority: None (observer)

**This is your identity as a tree of relationships, not a profile.**

**You select "Access" (see what you can do):**

**At Acme Corp:**
- ✅ Enter buildings
- ✅ Read financial data
- ✅ Create tasks
- ✅ Delegate to SCVs
- ❌ Approve budgets >$50K (need manager approval)
- ❌ Access HR records (not your role)

**At Local Civic Association:**
- ✅ Vote on proposals
- ✅ Attend meetings
- ✅ Submit proposals
- ❌ Unilaterally approve (requires quorum)

**This is derived access, not assigned permissions.**

Your access comes from:
- Your bindings (employee, member, supporter)
- The rules of each context
- Time-based authority grants
- Delegation chains

**It's legible. You can trace it. You can understand why you can or can't do something.**

---

## 📊 SCENE 18: WORK AS FILAMENT ADVANCEMENT

**You've been at Acme Corp for 3 years.**

**You open your work history.**

**Work Filament: `identity.you.work`**

**Recent Commits:**

📦 **c847:** Completed Q1 forecast review  
   → Approved conservative branch  
   → Decision: Lower growth targets  
   → Rationale: Risk factors identified by SCV

📦 **c846:** Delivered partnership negotiation draft  
   → Collaboration with Alex (Berlin)  
   → Outcome: Contract terms agreed  
   → Next: Legal review

📦 **c845:** Participated in office redesign vote  
   → Delegated influence to design committee  
   → Outcome: Approved  
   → Implementation: Q2

**You zoom out (scrub time backward):**

The filament stretches back 3 years. You can see:
- Projects you completed
- Decisions you made
- Who you worked with
- What you learned
- Where you grew

**This is your career as append-only history.**

**Not a resume. Not self-reported. Not curated.**  
**It's what you actually did, with proof.**

**When you apply for a new role:**
- You don't write a resume
- You share your filament tree (scoped)
- Employers can verify everything
- Scars are visible (failures happened, you learned)
- Authority is provable (you had real responsibility)

**Work is legible.**

---

## 🌍 SCENE 19: THE GLOBAL VIEW (LOD REALITY)

**You zoom all the way out.**

The globe spins. You see the **entire Earth**.

Not as "users online" or "activity heatmap."

As **coordination reality**, rendered at the appropriate level of detail.

**Zoom levels (automatic LOD):**

**Global view:**
- Major coordination nodes (cities, research centers, trade routes)
- Large-scale filaments (international agreements, supply chains)
- Force units (if conflict exists—hoping not)
- Climate/infrastructure (energy grids, water systems)

**Regional view:**
- Buildings become visible
- Proximity channels appear
- Shipments moving
- People present (as aggregates, not individuals)

**Local view:**
- Individual buildings clear
- Units identifiable
- People visible (if you have access)
- Filament details

**Inspection view:**
- Full commit history
- Evidence links
- Audit trails
- Forensic detail

**The system never shows you everything.**  
**It shows you *what's relevant* at *your current scale*.**

This is **Level of Detail (LOD) rendering** applied to **governance and coordination**.

---

## 🎬 CLOSING: WHAT YOU FEEL

**After a day in Relay, you never once felt:**

- Lost (navigation was always clear)
- Abstracted away (everything was physical, located)
- Unsure who did what (history was visible)
- Unsure why something happened (evidence was traceable)
- Powerless (authority was explicit and revocable)
- Surveilled (privacy was tiered, controlled by you)
- Forced (participation was voluntary)

**You weren't:**
- Scrolling feeds
- Checking notifications
- Filling forms
- Trusting invisible systems
- Wondering "who decided this?"

**You were:**
- **Operating in a world** where history is visible
- **Coordinating with others** without losing yourself
- **Using tools (SCVs)** without being controlled by them
- **Making decisions** with full context
- **Tracking work** that actually matters
- **Seeing logistics** as physical reality
- **Voting** with explicit, revocable influence
- **Building** toward outcomes you care about

---

## 🌟 THE QUIET REALIZATION

**This isn't social media + e-commerce + governance.**

**This is a playable coordination reality.**

**StarCraft didn't just inspire the UI.**  
**It inspired the truth model.**

**Buildings aren't icons—they're capability anchors.**  
**Filaments aren't data—they're history.**  
**SCVs aren't bots—they're force multipliers.**  
**The globe isn't a map—it's the shared build space.**

**And you?**

**You're not a user.**  
**You're a coordinator in a legible world.**

---

## 🎯 THE PROMISE KEPT

**Relay promised:**
- Truth without tyranny
- Coordination without centralization
- AI collaboration without loss of agency
- Governance without hidden defaults
- Work without hierarchy worship
- Money without abstraction
- Logistics without mystery

**And today, using Relay, you experienced all of it.**

**Not as metaphor.**  
**As reality.**

---

**This is Relay.**  
**This is how humans coordinate when the system stops lying.**

**🌍 END OF EXPERIENCE**

---

**Refs:** [All architecture c0-c16], [All objects], [All principles]  
**Status:** 🔒 Gold Standard  
**Purpose:** Show the complete integrated experience  
**Next:** Lock this down, then execute cleanup

**THE COMPLETE RELAY EXPERIENCE**

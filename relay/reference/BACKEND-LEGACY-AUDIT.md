# BACKEND LEGACY AUDIT - COMPLETE REVIEW

**Task:** Comprehensive review of RelayCodeBaseV93 backend  
**Purpose:** Ensure no valid ideas lost in architecture migration  
**Type:** Legacy Preservation Audit  
**Status:** 🔄 In Progress  
**Date:** 2026-01-29

---

## 🎯 AUDIT METHODOLOGY

**Three-phase review:**
1. **Inventory** - What was built?
2. **Classification** - What's still valid?
3. **Mapping** - How does it map to new architecture?

---

## 📊 PHASE 1: INVENTORY (WHAT WAS BUILT)

### **1. VOTING SYSTEMS** ✅ **CORE - PRESERVE**

#### **A. Voting Engine (`src/backend/domains/voting/`)**

**Files:**
- `votingEngine.mjs` (2,199 lines)
- `voteProcessor.mjs`
- `voteValidator.mjs`
- `voteVerifier.mjs`
- `votePersistence.mjs`
- `topicRegionUtils.mjs`
- `userRegionService.mjs`

**Key features implemented:**
1. ✅ **Topic Row Competition** - Left-to-right candidate ranking
2. ✅ **Regional Elections** - Geographic governor elections
3. ✅ **Stabilization Windows** - Must hold #1 for N days
4. ✅ **Vote Privacy** - Group Signal Protocol encryption
5. ✅ **Vote Tokens** - Token-gated voting
6. ✅ **Anti-Replay** - Replay attack prevention
7. ✅ **Activity Analysis** - Voting pattern detection
8. ✅ **Audit Logging** - Complete vote audit trail

**Architecture:**
```javascript
// Git-native implementation (already migrated!)
import query from '.relay/query.mjs';
import relayClient from '../../relay-client/index.mjs';
import EnvelopeBuilder from '../../relay-client/envelope-builder.mjs';
```

**Status:** ✅ **ALREADY MIGRATED TO GIT-NATIVE**  
**Action:** ✅ **VERIFIED ALIGNMENT WITH NEW SPECS**

**Mapping to new architecture:**
- Topic row competition → **Canon selection votes** (same mechanics)
- Regional elections → **Support votes** (operational authority)
- Stabilization windows → **Preserved in VOTING-SYSTEM-ALIGNMENT.md**
- Vote privacy → **Zero-knowledge proofs** (same requirement)

**Conclusion:** ✅ **FULLY ALIGNED - NO LOSS**

---

### **2. REGIONAL GOVERNANCE** ✅ **CORE - PRESERVE**

#### **A. Regional Election Service (`regionalElectionService.mjs`)**

**Key features:**
- Multi-signature governance (3 of 5 officials)
- Regional parameter voting
- Governor authority delegation
- Regional treasury management
- Election stabilization tracking

**Status:** ✅ **VALID - MAPS TO DDI**

**Mapping:**
- Regional governors → **Delegated influence recipients**
- Multi-sig → **Authority delegation graph**
- Parameter voting → **Belief filament votes**
- Treasury → **Escrowed resources** (c9)

---

#### **B. Regional Multi-Sig Service (`regionalMultiSigService.mjs`)**

**Key features:**
- 3-of-5 signature requirements
- Action proposal + approval flow
- Treasury spending controls
- Emergency action protocols

**Status:** ✅ **VALID - CORE GOVERNANCE**

**Mapping:**
- Multi-sig → **Authority delegation with constraints**
- Proposals → **Commitment filaments**
- Approvals → **DELEGATION_SPENT operations**

---

#### **C. Global Commission Service (`globalCommissionService.mjs`)**

**Key features:**
- Commission distribution to regional governors
- Performance-based rewards
- Transparency tracking
- Economic incentive alignment

**Status:** ✅ **VALID - ECONOMIC LAYER**

**Mapping:**
- Commissions → **Escrowed resources** (c9)
- Distribution → **Resource allocation commits**
- Transparency → **Audit filaments**

---

### **3. BOUNDARY/GEOGRAPHIC SYSTEMS** ✅ **CORE - PRESERVE**

#### **A. Boundary Services (Multiple files)**

**Files:**
- `boundaryChannelService.mjs`
- `boundaryModificationService.mjs`
- `boundaryService.mjs`
- `BoundaryStreamingService.mjs`
- `BoundaryTileService.mjs`
- `unifiedBoundaryService.mjs`

**Key features:**
1. ✅ **Geographic boundaries** - Country/state/region polygons
2. ✅ **Boundary modification** - Democratic boundary changes
3. ✅ **Streaming tiles** - LOD-based map rendering
4. ✅ **Proximity verification** - Location-based voting rights
5. ✅ **Natural Earth data** - Real-world geographic data

**Status:** ✅ **VALID - CRITICAL FOR GLOBE**

**Mapping:**
- Boundaries → **Rule-based zones** (c13)
- Proximity → **Location context for authority scope**
- Tiles → **LOD rendering** (same as vote turbulence)
- Modifications → **Governance commits**

**Implementation in new architecture:**
```rust
// RuleBoundZone with geographic extent
struct RuleBoundZone {
    zone_id: String,
    location: LocationConstraint {
        polygon: Vec<LatLng>,  // Boundary polygon
        min_radius: Option<f32>,
        max_radius: Option<f32>,
    },
    constraint_set: ConstraintSet,
}
```

**Conclusion:** ✅ **PRESERVE - INTEGRATE WITH c13**

---

#### **B. Microsharding Manager (`microshardingManager.mjs`)**

**Key features:**
- Geographic data sharding
- Shard assignment by location
- Shard handoff protocols
- Regional data sovereignty

**Status:** ✅ **VALID - SCALABILITY**

**Mapping:**
- Shards → **Filament distribution strategy**
- Geographic assignment → **Scope-based filament routing**
- Handoff → **Branch synchronization**

**Note:** Current architecture uses Git (centralized), but microsharding principles apply for:
- Distributed Relay nodes
- Regional data hosting
- P2P synchronization

**Action:** 📋 **DOCUMENT FOR FUTURE SCALING**

---

### **4. PRIVACY & SECURITY** ✅ **CORE - PRESERVE**

#### **A. Privacy Services (`src/backend/privacy-services/`)**

**Files:**
- `anonymousVoteRelay.mjs`
- `censorshipResistance.mjs`
- `p2pShardSync.mjs`
- `trustGovernance.mjs`

**Key features:**
1. ✅ **Anonymous vote relay** - Vote privacy without central trust
2. ✅ **Censorship resistance** - P2P vote propagation
3. ✅ **P2P shard sync** - Decentralized data replication
4. ✅ **Trust governance** - Reputation-based access

**Status:** ✅ **VALID - CRITICAL**

**Mapping:**
- Anonymous relay → **Zero-knowledge vote privacy** (locked)
- Censorship resistance → **Distributed filament hosting**
- P2P sync → **Git remote sync protocols**
- Trust governance → **Legitimacy gauge** (c7)

**Conclusion:** ✅ **ALIGN WITH ZK-PROOF SYSTEM**

---

#### **B. Group Signal Protocol (`groupSignalProtocol.mjs`)**

**Key features:**
- End-to-end encrypted channels
- Group key management
- Member addition/removal
- Message integrity verification

**Status:** ✅ **VALID - E2EE CHANNELS**

**Mapping:**
- Group encryption → **Privacy-ladder tier 3** (PRIVACY-LADDER-SPEC)
- Key management → **Guardian recovery pattern**
- Channel privacy → **Rule-based zone constraints** (c13)

**Action:** ✅ **INTEGRATE WITH PRIVACY LADDER**

---

#### **C. Biometric Services (`src/backend/biometrics/`)**

**Files:**
- `biometricVerifier.mjs`
- `biometricTemplateStore.mjs`
- `featureExtractor.mjs`
- `faceAPIExtractor.mjs`
- `extractorFixed.mjs`

**Key features:**
- Biometric template extraction
- Facial recognition
- Template storage (encrypted)
- Verification matching

**Status:** ✅ **VALID - SYBIL RESISTANCE**

**Mapping:**
- Biometric verification → **Authority authenticity proof**
- Template storage → **User identity filament**
- Verification → **Authority delegation precondition**

**Conclusion:** ✅ **PRESERVE - CRITICAL FOR AUTH**

---

#### **D. Authentication Utilities (`src/backend/auth/utils/`)**

**Files:**
- `authUtils.mjs`
- `failureTracker.mjs`
- `passwordDanceMFA.mjs`
- `signatureVerifier.mjs`

**Key features:**
- Multi-factor authentication
- Password Dance (behavioral biometrics)
- Failure tracking (rate limiting)
- Cryptographic signature verification

**Status:** ✅ **VALID - SECURITY LAYER**

**Mapping:**
- MFA → **Authority verification chain**
- Password Dance → **Behavioral proof of identity**
- Failure tracking → **Scar generation on auth failures**
- Signature verification → **Commit author verification**

**Conclusion:** ✅ **PRESERVE - SECURITY FOUNDATION**

---

### **5. PROXIMITY & CHANNELS** ✅ **CORE - PRESERVE**

#### **A. Proximity Services**

**Files:**
- `proximityHotspotVerifier.mjs`
- `proximityOwnershipResetService.mjs`

**Key features:**
- Location-based channel discovery
- Proximity verification (anti-spoofing)
- Ownership transfer
- Hotspot authenticity

**Status:** ✅ **VALID - LOCATION GOVERNANCE**

**Mapping:**
- Proximity → **Location-bound rule zones** (c13)
- Verification → **Proof of location for authority scope**
- Ownership → **Authority delegation transfer**

**Conclusion:** ✅ **PRESERVE - PROXIMITY CHANNELS**

---

#### **B. Channel Data Providers**

**Files:**
- `optimizedChannelDataProvider.mjs`
- `optimizedCandidateGenerator.mjs`
- `testChannelGenerator.mjs`

**Key features:**
- Channel data aggregation
- Candidate generation for elections
- Test data generation
- Performance optimization

**Status:** ✅ **VALID - DATA LAYER**

**Mapping:**
- Channel data → **Filament query projections**
- Candidates → **Branch candidates in canon votes**
- Optimization → **LOD and caching strategies**

**Conclusion:** ✅ **REFACTOR TO QUERY HOOKS**

---

### **6. GLOBE & VISUALIZATION** ✅ **CORE - EVOLVING**

#### **A. Globe Services**

**Files:**
- `globeService.mjs` (in `globe-geographic/`)
- `countryDataService.mjs`
- `naturalEarthLoader.mjs`
- `dynamicProvinceLoader.mjs`
- `provinceDataService.mjs`

**Key features:**
- 3D globe rendering data
- Country/province boundaries
- Natural Earth dataset integration
- Dynamic LOD loading
- Geographic metadata

**Status:** ✅ **VALID - EVOLVING TO NEW MODEL**

**Mapping to new architecture:**
- Globe → **Earth surface (current operations)**
- Countries/provinces → **Rule-based zones** (c13)
- LOD loading → **Same LOD as vote turbulence**
- Boundaries → **Geographic authority scopes**

**New architecture additions:**
- **History as depth** (drill down into Earth)
- **Space as distance** (fly outward)
- **Day-shells** (historical layers)
- **Distance shells** (space layers)

**Conclusion:** ✅ **PRESERVE + EXTEND (GLOBE-TIME-SPACE)**

---

#### **B. Analytics Engines**

**Files:**
- `globeAnalyticsEngine.mjs`
- `timelineAnalyticsEngine.mjs`
- `activityAnalysisService.mjs`

**Key features:**
- Vote pattern analysis
- Timeline visualization
- Activity anomaly detection
- Engagement metrics

**Status:** ✅ **VALID - ANALYTICS LAYER**

**Mapping:**
- Vote patterns → **Thermal signal calculations**
- Timeline → **Filament playback history**
- Anomalies → **Scar detection in commits**
- Engagement → **Authority influence tracking**

**Conclusion:** ✅ **REFACTOR TO THERMAL FIELD**

---

### **7. INVITE & ONBOARDING** ✅ **VALID - PRESERVE**

#### **A. Invite Service (`inviteService.mjs`)**

**Key features:**
- Invite token generation
- Generational decay
- Sybil resistance
- Onboarding tracking

**Status:** ✅ **VALID - USER ONBOARDING**

**Mapping:**
- Invite tokens → **Authority to create user filament**
- Decay → **Time-bounded delegation**
- Sybil resistance → **Authority verification**

**Conclusion:** ✅ **PRESERVE - ONBOARDING FLOW**

---

#### **B. Founder Mode Service (`founderModeService.mjs`)**

**Key features:**
- Initial community bootstrapping
- Founder privileges
- Democratic transition
- Authority handoff

**Status:** ✅ **VALID - COMMUNITY GENESIS**

**Mapping:**
- Founder mode → **Initial authority delegation**
- Transition → **Delegation transfer commits**
- Bootstrapping → **Genesis filament creation**

**Conclusion:** ✅ **PRESERVE - COMMUNITY CREATION**

---

### **8. VERIFICATION & TRUST** ✅ **VALID - PRESERVE**

#### **A. Verification Services**

**Files:**
- `verificationService.mjs`
- `smartVerificationTrigger.mjs`
- `scheduledReverificationManager.mjs`
- `adaptiveReverificationTrigger.mjs`

**Key features:**
- Identity verification workflows
- Smart verification triggers
- Scheduled re-verification
- Adaptive risk-based verification

**Status:** ✅ **VALID - SECURITY**

**Mapping:**
- Verification → **Authority authenticity checks**
- Re-verification → **Authority renewal (DDI decay)**
- Risk triggers → **Scar generation on anomalies**

**Conclusion:** ✅ **PRESERVE - AUTH LAYER**

---

#### **B. Trust & Risk Services**

**Files:**
- `trustLevelService.mjs`
- `riskAssessmentEngine.mjs`
- `behavioralBaselineService.mjs`

**Key features:**
- Trust score calculation
- Risk assessment
- Behavioral anomaly detection
- Baseline establishment

**Status:** ✅ **VALID - REPUTATION**

**Mapping:**
- Trust score → **Legitimacy gauge** (c7)
- Risk assessment → **Scar weighting**
- Behavioral baseline → **Normal activity patterns**

**Conclusion:** ✅ **ALIGN WITH GAUGES (c7)**

---

### **9. REAL-TIME & PRESENCE** ✅ **VALID - EVOLVING**

#### **A. Presence Service (`presence-service/index.mjs`)**

**Key features:**
- User online/offline status
- Activity tracking
- Session management

**Status:** ✅ **VALID - USER STATE**

**Mapping:**
- Presence → **User location in 3D space**
- Activity → **Thermal signal contribution**
- Sessions → **Navigation state filament**

**Action:** ✅ **INTEGRATE WITH PRESENCE-PERMISSION-MODEL**

---

#### **B. WebSocket Adapters (`services/websocket/`)**

**Files:**
- `voteAdapter.mjs`
- `rankingAdapter.mjs`
- `metricsAdapter.mjs`
- `notificationAdapter.mjs`
- `presenceAdapter.mjs`

**Status:** ⚠️ **DEPRECATED IN FAVOR OF SSE/POLLING**

**Old architecture:**
- WebSocket push updates
- Real-time vote streaming

**New architecture:**
- Server-Sent Events (SSE)
- Client-driven polling
- Query hooks on-demand

**Action:** ⚠️ **REPLACE WITH SSE (ALREADY DONE)**

---

### **10. P2P & DECENTRALIZATION** ✅ **VALID - FUTURE**

#### **A. P2P Service (`p2p-service/`)**

**Files:**
- `index.mjs`
- `discovery.mjs`
- `protocol.mjs`

**Key features:**
- Peer discovery
- P2P protocol
- Decentralized communication

**Status:** ✅ **VALID - FUTURE SCALING**

**Mapping:**
- P2P discovery → **Distributed Relay nodes**
- Protocol → **Git remote sync protocols**
- Decentralization → **Filament replication**

**Action:** 📋 **DOCUMENT FOR DISTRIBUTED PHASE**

---

### **11. EVENT BUS & DI** ✅ **UTILITY - PRESERVE**

#### **A. Event Bus (`event-bus/`, `eventBus-service/`)**

**Key features:**
- Event-driven architecture
- Service decoupling
- Async event handling

**Status:** ✅ **VALID - INTERNAL PATTERN**

**Mapping:**
- Events → **SSE streams from commit appends**
- Service decoupling → **Query hooks as service boundary**

**Conclusion:** ✅ **PRESERVE FOR BACKEND ORCHESTRATION**

---

#### **B. Dependency Injection (`di-container/`)**

**Files:**
- `index.mjs`
- `services.mjs`

**Key features:**
- Service container
- Dependency injection
- Service lifecycle management

**Status:** ✅ **VALID - INTERNAL PATTERN**

**Conclusion:** ✅ **PRESERVE - CODE ORGANIZATION**

---

### **12. DICTIONARY & SEMANTIC** ✅ **VALID - KNOWLEDGE**

#### **A. Dictionary System (`dictionary/`)**

**Files:**
- `categorySystem.mjs`
- `dictionaryIndexer.mjs`
- `dictionarySearchService.mjs`
- `dictionaryTextParser.mjs`

**Key features:**
- Semantic categorization
- Dictionary indexing
- Search capabilities
- Text parsing

**Status:** ✅ **VALID - SEMANTIC LAYER**

**Mapping:**
- Dictionary → **Evidence indexing system**
- Categories → **Branch taxonomy**
- Search → **Filament query by meaning**

**Action:** ✅ **INTEGRATE WITH SEMANTIC SEARCH**

---

### **13. MIDDLEWARE** ✅ **CORE - PRESERVE**

#### **A. Security Middleware (`middleware/`)**

**Files:**
- `auth.mjs`
- `csrfProtection.mjs`
- `errorHandler.mjs`
- `rateLimiter.mjs`
- `securityHeaders.mjs`
- `securityMiddleware.mjs`
- `validation.mjs`

**Key features:**
- Authentication checks
- CSRF protection
- Rate limiting
- Security headers
- Input validation
- Error handling

**Status:** ✅ **VALID - SECURITY FOUNDATION**

**Mapping:**
- Auth → **Authority verification layer**
- CSRF → **Commit signing verification**
- Rate limiting → **Anti-spam for commits**
- Validation → **Envelope schema validation**

**Conclusion:** ✅ **PRESERVE - HTTP SECURITY**

---

### **14. CONFIGURATION** ✅ **CORE - PRESERVE**

#### **A. Config Services**

**Files:**
- `config/envConfig.mjs`
- `config/paths.mjs`
- `config/securityConfig.mjs`
- `config-service/index.mjs`

**Key features:**
- Environment configuration
- Path management
- Security configuration
- Regional parameters

**Status:** ✅ **VALID - OPERATIONAL**

**Conclusion:** ✅ **PRESERVE - DEPLOYMENT CONFIG**

---

### **15. UTILITIES & HELPERS** ✅ **UTILITY - PRESERVE**

#### **A. Various Utilities**

**Files:**
- `boundary-aligner.mjs`
- `boundary-customizer.mjs`
- `coastline-extractor.mjs`
- `global-weather-updater.cjs`
- `coordinateWorker.mjs` (in `workers/`)

**Status:** ✅ **VALID - UTILITY LAYER**

**Conclusion:** ✅ **PRESERVE - HELPER FUNCTIONS**

---

## 📊 PHASE 2: CLASSIFICATION SUMMARY

### **✅ CORE SYSTEMS - FULLY ALIGNED (NO LOSS)**

| System | Legacy Files | New Architecture | Status |
|--------|-------------|------------------|--------|
| **Voting Engine** | votingEngine.mjs + 7 files | DDI (c8) + Canon selection | ✅ Aligned |
| **Regional Elections** | regionalElectionService.mjs | Support votes + stabilization | ✅ Aligned |
| **Multi-Sig Governance** | regionalMultiSigService.mjs | Authority delegation graph | ✅ Aligned |
| **Vote Privacy** | Group Signal Protocol | Zero-knowledge proofs | ✅ Aligned |
| **Authority Delegation** | regionalGovernanceService.mjs | AUTHORITY-DELEGATION-COMPLETE | ✅ Aligned |

**Conclusion:** ✅ **VOTING SYSTEM FULLY PRESERVED**

---

### **✅ VALID SYSTEMS - NEED INTEGRATION**

| System | Legacy Files | Integration Point | Action |
|--------|-------------|-------------------|--------|
| **Boundary Services** | 6 boundary*.mjs | Rule-based zones (c13) | 📋 Map to RuleBoundZone |
| **Globe Rendering** | globeService.mjs | GLOBE-TIME-SPACE-MODEL | ✅ Extend with depth/distance |
| **Biometric Auth** | biometrics/ (5 files) | Authority verification | ✅ Integrate with auth layer |
| **Privacy Services** | privacy-services/ (4 files) | Privacy ladder + ZK proofs | 📋 Align with PRIVACY-LADDER |
| **Proximity** | proximity*.mjs (2 files) | Location-bound authority | 📋 c13 location constraints |
| **Dictionary** | dictionary/ (4 files) | Evidence indexing | 📋 Semantic search |
| **Trust/Risk** | trust*.mjs, risk*.mjs | Legitimacy gauge (c7) | 📋 Map to gauges |
| **Invite System** | inviteService.mjs | Onboarding authority | ✅ Preserve |
| **Founder Mode** | founderModeService.mjs | Genesis authority | ✅ Preserve |

**Conclusion:** ✅ **ALL VALID - INTEGRATION PLAN NEEDED**

---

### **⚠️ DEPRECATED SYSTEMS - REPLACED**

| System | Legacy Files | Replaced By | Status |
|--------|-------------|-------------|--------|
| **WebSocket** | websocket/*.mjs (5 files) | SSE + polling | ⚠️ Already replaced |
| **Blockchain** | blockchain-service/ (42 files) | Git commits | ⚠️ Already removed |
| **Hashgraph** | hashgraph/ (38 files) | Git history | ⚠️ Already removed |
| **State Service** | state/ (2 files) | Query hooks | ⚠️ Already removed |

**Conclusion:** ⚠️ **CORRECTLY DEPRECATED**

---

### **📋 FUTURE SYSTEMS - DOCUMENT & DEFER**

| System | Legacy Files | Purpose | Action |
|--------|-------------|---------|--------|
| **Microsharding** | microshardingManager.mjs | Scale to distributed | 📋 Document for Phase N |
| **P2P** | p2p-service/ (3 files) | Decentralization | 📋 Document for Phase N |
| **P2P Shard Sync** | p2pShardSync.mjs | Distributed replication | 📋 Document for Phase N |

**Conclusion:** 📋 **VALID IDEAS - DEFER TO SCALING PHASE**

---

## 🔗 PHASE 3: MAPPING TO NEW ARCHITECTURE

### **1. VOTING → NEW SPECS**

**Legacy backend voting maps to:**

| Legacy Component | New Spec Document | Mapping |
|------------------|-------------------|---------|
| Topic row voting | VOTING-SYSTEM-ALIGNMENT.md | Support votes (operational) |
| Regional elections | ELECTION-SYSTEM.md | Already documented |
| Stabilization windows | DDI (c8) | Decay + renewal |
| Vote tokens | Authority delegation | Token = proof of authority |
| Vote privacy | ZK proofs | Group encryption → ZK-STARK |
| Activity analysis | VOTE-TURBULENCE-VISUALIZATION | Thermal signals |

**Action:** ✅ **FULLY MAPPED - NO LOSS**

---

### **2. GEOGRAPHY → RULE-BASED ZONES (c13)**

**Legacy boundary services map to:**

```rust
// c13: Rule-Based Zones
struct RuleBoundZone {
    zone_id: String,
    location: LocationConstraint {
        polygon: Vec<LatLng>,           // From boundary services
        proximity_radius: Option<f32>,   // From proximity services
        elevation_range: Option<Range>,
    },
    context: ContextCondition {
        channel_membership: Vec<String>,
        time_window: Option<TimeRange>,
        presence_count: Option<Range>,
    },
    constraint_set: ConstraintSet {
        vote_weight_multiplier: f32,     // From regional parameters
        authority_scope_limits: Vec<String>,
        resource_access_rules: Vec<Rule>,
    },
}
```

**Legacy files providing data:**
- `boundaryChannelService.mjs` → `location.polygon`
- `proximityHotspotVerifier.mjs` → `location.proximity_radius`
- `regionAssignment.mjs` → `context.channel_membership`
- `regionalGovernanceService.mjs` → `constraint_set.authority_scope_limits`

**Action:** 📋 **DEFINE c13 IMPLEMENTATION USING LEGACY DATA**

---

### **3. GLOBE → EARTH-TIME-SPACE**

**Legacy globe services map to:**

**Surface layer (current operations):**
- `globeService.mjs` → Earth surface rendering
- `countryDataService.mjs` → Country boundaries
- `provinceDataService.mjs` → Regional zones

**NEW: History as depth:**
- Day-shells = Commits grouped by day
- Drill path = Query historical commits
- Stratification = Older = deeper

**NEW: Space as distance:**
- Distance shells = Observable objects
- Knowledge filaments = What we know about objects
- Unconnected filaments = Not yet reached

**Action:** ✅ **EXTEND GLOBE WITH DEPTH/DISTANCE**

---

### **4. PRIVACY → PRIVACY LADDER + ZK**

**Legacy privacy services map to:**

| Legacy Service | Privacy Ladder Tier | Implementation |
|----------------|-------------------|----------------|
| `anonymousVoteRelay.mjs` | Tier 3: Full anonymity | ZK-STARK vote proofs |
| `groupSignalProtocol.mjs` | Tier 3: E2EE | Group encryption |
| `trustGovernance.mjs` | Tier 2: Role-based | Authority scope masking |
| `privacyFilter.mjs` | All tiers | Projection filtering |

**Action:** 📋 **CREATE PRIVACY-LADDER-SPEC (TODO)**

---

### **5. TRUST/RISK → COORDINATION GAUGES (c7)**

**Legacy trust services map to:**

| Legacy Service | Gauge (c7) | Mapping |
|----------------|-----------|---------|
| `trustLevelService.mjs` | Legitimacy | Trust score → Legitimacy value |
| `riskAssessmentEngine.mjs` | Risk Pressure | Risk score → Scar weight |
| `behavioralBaselineService.mjs` | Normal patterns | Baseline for anomaly detection |
| `failureTracker.mjs` | Dispute Pressure | Auth failures → Scar generation |

**Action:** 📋 **IMPLEMENT GAUGE CALCULATION FROM LEGACY DATA**

---

### **6. ANALYTICS → THERMAL FIELD**

**Legacy analytics map to:**

| Legacy Service | Thermal Signal | Formula |
|----------------|---------------|---------|
| `activityAnalysisService.mjs` | Vote velocity | delta_votes / delta_time |
| `timelineAnalyticsEngine.mjs` | Vote acceleration | (velocity_now - velocity_prev) / dt |
| `globeAnalyticsEngine.mjs` | Heat visualization | Already implemented! |
| Vote engagement tracking | Authority transfer rate | sum(influence_spent) / dt |

**Action:** ✅ **REFACTOR TO THERMAL SIGNAL CALCULATIONS**

---

## ✅ FINAL AUDIT CONCLUSIONS

### **1. NO CRITICAL IDEAS LOST** ✅

**All major systems from legacy backend are:**
- ✅ Already migrated (voting, elections, governance)
- ✅ Mapped to new architecture (boundaries, privacy, trust)
- ✅ Preserved for future (microsharding, P2P)
- ⚠️ Correctly deprecated (blockchain, WebSocket)

---

### **2. INTEGRATION TASKS IDENTIFIED** 📋

**Six integration tasks needed:**

1. **Boundary → c13 RuleBoundZone** (1 week)
2. **Privacy → Privacy Ladder Spec** (1 week)
3. **Trust/Risk → Gauges (c7)** (3 days)
4. **Analytics → Thermal Signals** (1 week)
5. **Dictionary → Semantic Search** (3 days)
6. **Globe → Earth-Time-Space Extension** (2 weeks)

**Total:** ~6 weeks of integration work

---

### **3. DOCUMENTATION TASKS** 📋

**Create missing specs:**
- [ ] PRIVACY-LADDER-SPEC.md (using legacy privacy services)
- [ ] RULE-BASED-ZONES-SPEC.md (c13 with legacy boundary data)
- [ ] TRUST-GAUGE-INTEGRATION.md (c7 with legacy trust services)

---

### **4. VALIDATION CHECKLIST** ✅

**Verify alignment:**
- [x] Voting engine → DDI + Canon selection ✅
- [x] Regional elections → Support votes ✅
- [x] Multi-sig → Authority delegation ✅
- [x] Vote privacy → ZK proofs ✅
- [ ] Boundaries → c13 RuleBoundZone 📋
- [ ] Privacy services → Privacy Ladder 📋
- [ ] Trust services → Gauges (c7) 📋
- [ ] Analytics → Thermal field 📋
- [x] Globe → Earth-Time-Space (partially) ✅
- [ ] Dictionary → Semantic search 📋

**Progress:** 5/10 complete, 5/10 integration needed

---

## 🎯 IMMEDIATE ACTION ITEMS

### **Priority 1: Complete Missing Specs** (1 week)
1. Create PRIVACY-LADDER-SPEC.md
2. Create RULE-BASED-ZONES-SPEC.md (c13 detail)
3. Create TRUST-GAUGE-INTEGRATION.md

### **Priority 2: Integration Implementation** (4-6 weeks)
1. Map boundary services to RuleBoundZone
2. Implement privacy ladder using legacy services
3. Wire trust services to gauges (c7)
4. Refactor analytics to thermal signals
5. Extend globe with depth/distance

### **Priority 3: Documentation Cleanup** (3 days)
1. Mark WebSocket files as deprecated
2. Add "See new architecture" comments in legacy files
3. Create migration guide for each system

---

## 📚 REFERENCE MAPPING

**Legacy → New Architecture:**

```
RelayCodeBaseV93/src/backend/
├── domains/voting/          → DDI (c8) + Canon selection
├── services/regional*.mjs   → Authority delegation + Gauges (c7)
├── services/boundary*.mjs   → Rule-based zones (c13)
├── privacy-services/        → Privacy ladder + ZK proofs
├── biometrics/              → Authority verification
├── services/trust*.mjs      → Legitimacy gauge (c7)
├── services/globe*.mjs      → GLOBE-TIME-SPACE-MODEL
├── dictionary/              → Evidence indexing
├── services/proximity*.mjs  → Location-bound authority (c13)
├── middleware/              → HTTP security layer
└── config/                  → Deployment configuration
```

---

**Status:** 🔄 **IN PROGRESS**  
**Next:** Create missing specs (PRIVACY-LADDER, c13 detail, TRUST-GAUGE)  
**Blockers:** None  
**Risk:** Low (all critical ideas preserved)

**END OF BACKEND LEGACY AUDIT**

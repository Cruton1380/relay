# RELAY AUDITOR GUIDE: Understanding Relay GAPP & NIST Equivalents

**Date:** 2026-01-29  
**Audience:** Traditional auditors (GAAP/IFRS/NIST/ISO backgrounds)  
**Purpose:** Translate existing audit knowledge to Relay's universal audit framework

---

## 🎯 FOR AUDITORS: WHAT IS RELAY?

**Relay is a coordination substrate that makes reality truthful and auditable.**

Think of it as:
- A universal ledger (like blockchain, but for ALL reality, not just transactions)
- A real-time simulation (like StarCraft, but for real world)
- A verification layer (like audit, but continuous and structural)

**Your role as an auditor:**
You verify that reality is faithfully encoded into this system. Same fundamental skills, different medium.

---

## 📐 RELAY GAPP: YOUR NEW STANDARD

**GAPP = Generally Applicable Provenance Principles**

This is Relay's equivalent of GAAP/IFRS, but **universal** (not just financial).

### **The 8 Core Principles:**

| GAPP Principle | Traditional Equivalent | What It Means |
|----------------|------------------------|---------------|
| **1. Existence** | GAAP: Existence assertion | If material, must exist in Relay |
| **2. Completeness** | GAAP: Completeness assertion | Absence must be explicit |
| **3. Consistency** | GAAP: Consistency principle | Same reality = same representation |
| **4. Temporal Integrity** | GAAP: Cut-off / Period boundaries | History immutable, no retroactive edits |
| **5. Traceability** | GAAP: Audit trail | Every state traces to source |
| **6. Independence** | GAAP: Auditor independence | Provable separation required |
| **7. Progressive Assurance** | GAAP: Risk-based approach | More risk = more evidence |
| **8. Zone Awareness** | GAAP: Jurisdictional accounting | Rules vary by context/zone |

**Key difference from GAAP:** These apply to EVERYTHING (military assets, buildings, sessions, damage), not just financials.

---

## 🛡️ RELAY NIST: YOUR SECURITY FRAMEWORK

**Relay NIST = Operational Assurance Controls**

Maps directly to NIST Cybersecurity Framework:

| NIST Function | Relay Implementation | Your Audit Steps |
|---------------|----------------------|------------------|
| **IDENTIFY** | RealityCoverageMap + asset registries | Verify all critical assets enumerated |
| **PROTECT** | Filament immutability + zone access | Test that history can't be mutated |
| **DETECT** | Continuous audit + finding generation | Review findings stream |
| **RESPOND** | Finding resolution + remediation | Verify remediation completed |
| **RECOVER** | Degraded audit mode + recovery logging | Audit recovery process itself |

---

## 📊 TRADITIONAL AUDIT → RELAY AUDIT MAPPING

### **Financial Statement Audit (GAAP/IFRS)**

| What You Audit Today | Relay Equivalent | How to Audit It |
|----------------------|------------------|-----------------|
| **Financial statements** | Transaction filaments | Query financial domain commits |
| **Account balances** | Derived state from filament replay | Replay commits, verify balance |
| **Journal entries** | Commit objects | Inspect commits, verify immutability |
| **Supporting documents** | AuditEvidence objects | Check evidence linkage |
| **Internal controls** | Zone constraints + Gates | Test zone rules, gate violations |
| **Materiality threshold** | MaterialityRule objects | Review rules, test application |
| **Audit trail** | Filament history | Trace any state to commits |
| **Cut-off testing** | Commit timestamps | Verify temporal integrity |
| **Existence assertion** | AuditAssertion (Existence type) | Test existence assertions |
| **Completeness assertion** | RealityCoverageMap | Review coverage gaps |
| **Audit opinion** | AuditOpinion object | Issue scoped opinion |

---

### **IT Audit / Cybersecurity (NIST)**

| What You Audit Today | Relay Equivalent | How to Audit It |
|----------------------|------------------|-----------------|
| **Asset inventory** | ForceUnit/Building registries | Verify coverage completeness |
| **Access controls** | Zone access (c13) + Identity (PR #9) | Test access derivation |
| **Change management** | Commit-only updates | Verify no silent mutations |
| **Data integrity** | SHA-256 hashing on evidence | Hash verification tests |
| **Incident response** | Finding resolution process | Review finding lifecycle |
| **Business continuity** | Degraded audit mode | Test degraded operation |
| **Security monitoring** | Continuous audit streams | Review real-time monitoring |
| **Vulnerability management** | MaterialityRule violations | Test auto-detection |

---

### **Quality Audit (ISO 9001)**

| What You Audit Today | Relay Equivalent | How to Audit It |
|----------------------|------------------|-----------------|
| **Quality metrics** | MaterialityRules for quality | Review quality thresholds |
| **Process documentation** | Filament commits | Trace process steps |
| **Nonconformities** | AuditFinding objects | Review findings by severity |
| **Corrective actions** | Remediation plans | Verify resolution |
| **Management review** | AuditOpinion scope | Issue quality opinion |

---

## 🔍 HOW TO PERFORM A RELAY AUDIT

### **Step 1: Define Audit Scope**

```json
{
  "scope": {
    "domain": "financial",
    "target_objects": ["org.acme.transactions"],
    "zones": ["zone.us.ca.sf"],
    "time_range": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-03-31T23:59:59Z"
    }
  }
}
```

**Traditional equivalent:** Engagement letter defining scope.

---

### **Step 2: Identify Materiality Rules**

Query relevant MaterialityRules:

```
GET /api/relay-physics/audit/materiality-rules?domain=financial
```

**Example response:**
```json
{
  "rules": [
    {
      "rule_id": "mat.financial.transaction_10k",
      "description": "Transactions over $10,000 must be logged",
      "threshold": {
        "metric": "transaction_value",
        "operator": "GreaterThan",
        "value": 10000,
        "unit": "USD"
      }
    }
  ]
}
```

**Traditional equivalent:** Setting materiality threshold (e.g., 5% of net income).

---

### **Step 3: Review RealityCoverageMap**

```
GET /api/relay-physics/audit/coverage-map?domain=financial&zone=zone.us.ca.sf
```

**Example response:**
```json
{
  "domain": "financial",
  "coverage_percentage": 0.92,
  "coverage_quality": "Complete",
  "gaps": [
    {
      "gap_id": "gap.financial.tx_123",
      "gap_type": "MissingAttribute",
      "description": "Transaction missing approval signature",
      "materiality": "Medium"
    }
  ]
}
```

**Traditional equivalent:** Completeness testing.

**Your action:** Investigate gaps, especially Critical/High materiality.

---

### **Step 4: Test Assertions**

For each material object, test assertions:

```
GET /api/relay-physics/audit/assertions?target=transaction.tx_001
```

**Example response:**
```json
{
  "assertions": [
    {
      "assertion_id": "assert.tx_001.existence",
      "assertion_type": "Existence",
      "claim": "Transaction occurred on 2024-01-15",
      "evidence_requirements": [
        {
          "evidence_type": "FilamentCommit",
          "min_count": 1,
          "independence_required": false
        }
      ],
      "status": "Verified",
      "verified_by": ["auditor.cert_01"]
    },
    {
      "assertion_id": "assert.tx_001.valuation",
      "assertion_type": "ValuationMagnitude",
      "claim": "Transaction value is $15,000",
      "evidence_requirements": [
        {
          "evidence_type": "ExternalAttestation",
          "min_count": 2,
          "independence_required": true
        }
      ],
      "status": "Pending"
    }
  ]
}
```

**Your action:**
- For "Verified" assertions: Review evidence quality
- For "Pending" assertions: Gather required evidence
- For "Disputed" assertions: Investigate conflicting evidence

---

### **Step 5: Collect and Verify Evidence**

```
GET /api/relay-physics/audit/evidence?assertion=assert.tx_001.valuation
```

**Example response:**
```json
{
  "evidence": [
    {
      "evidence_id": "ev.tx_001.bank_conf",
      "evidence_type": "ExternalAttestation",
      "source": {
        "source_type": "CommercialEntity",
        "source_id": "bank.chase",
        "independence_score": 0.9
      },
      "data": {
        "ExternalAttestation": {
          "attester_id": "bank.chase",
          "statement": "Transaction $15,000 confirmed",
          "signature": "0x..."
        }
      },
      "verified": true,
      "verified_by": ["auditor.cert_01"]
    }
  ]
}
```

**Your checks:**
1. **Source independence:** Is independence_score sufficient? (>0.7 for material items)
2. **Evidence completeness:** Are min_count requirements met?
3. **Evidence recency:** Is evidence within recency_requirement?
4. **Evidence authenticity:** Verify signatures/hashes

---

### **Step 6: Review Findings**

```
GET /api/relay-physics/audit/findings?scope=financial&severity=High,Critical
```

**Example response:**
```json
{
  "findings": [
    {
      "finding_id": "finding.2024_q1.tx_missing",
      "finding_type": "MissingObject",
      "severity": "High",
      "description": "5 transactions over $10k not logged as commits",
      "materiality_rule_violated": "mat.financial.transaction_10k",
      "status": "Open",
      "discovered_by": "auditor.cert_01"
    }
  ]
}
```

**Traditional equivalent:** Audit exceptions / material weaknesses.

**Your action:**
- For Open findings: Propose remediation
- For InProgress findings: Verify remediation progress
- For Resolved findings: Test effectiveness
- For Disputed findings: Investigate, gather more evidence

---

### **Step 7: Test Independence**

Verify your own independence:

```
GET /api/relay-physics/audit/auditor-profile/auditor.cert_01
```

**Example response:**
```json
{
  "auditor_id": "auditor.cert_01",
  "relationships": [
    {
      "related_to": "org.acme",
      "relationship_type": "Financial",
      "disclosed": true,
      "disclosed_at": 1704067200000
    }
  ],
  "conflicts": [],
  "engagements": ["engagement.acme.2024_q1"]
}
```

**Your checks:**
1. **Relationships disclosed?** All relationships with audited entity must be disclosed.
2. **Conflicts identified?** Any conflicts must be disclosed and mitigated.
3. **Independence score:** If audit requires high independence, verify no absolute conflicts.

**Traditional equivalent:** Independence questionnaire / conflict check.

---

### **Step 8: Issue Audit Opinion**

Create AuditOpinion object:

```json
{
  "opinion_id": "opinion.acme.2024_q1",
  "scope": { /* from Step 1 */ },
  "opinion_type": "Verified",
  "opinion_statement": "All material financial transactions are completely and accurately represented in the specified scope and period.",
  "assertions_tested": ["assert.tx_001", "assert.tx_002", /* ... */],
  "evidence_reviewed": ["ev.tx_001.bank_conf", /* ... */],
  "findings": ["finding.2024_q1.tx_missing"],
  "limitations": ["Unable to verify 3 transactions < $1,000 (below materiality threshold)"],
  "auditor_independence": {
    "independent": true,
    "relationships": ["Financial relationship disclosed 2024-01-01"],
    "conflicts": [],
    "explanation": "Disclosed relationships do not impair independence."
  },
  "issued_at": 1711929600000
}
```

**Opinion types:**
- **Verified** = Unqualified / Clean opinion
- **VerifiedWithExceptions** = Qualified opinion
- **Incomplete** = Scope limitation / Disclaimer
- **Adverse** = Material misstatement (rare)

**Traditional equivalent:** Audit report with opinion.

---

## 🧪 SAMPLE AUDIT PROCEDURES

### **Procedure 1: Test Transaction Completeness**

**Objective:** Verify all material transactions exist in Relay.

**Steps:**
1. Query MaterialityRules for financial domain
2. Identify threshold (e.g., $10,000)
3. Query external bank records for transactions > $10k in period
4. Query Relay for same transactions
5. Compare: Bank count vs Relay count
6. **Expected result:** Counts match
7. **If mismatch:** Create finding (MissingObject type)

**Relay queries:**
```
GET /api/relay-physics/audit/materiality-rules?domain=financial
GET /api/relay-physics/transactions?value_gte=10000&period=2024-Q1
```

---

### **Procedure 2: Test Temporal Integrity**

**Objective:** Verify history is immutable (no retroactive edits).

**Steps:**
1. Select sample of 20 commits from period
2. For each commit, verify:
   - Commit timestamp < current time
   - Commit hash matches content
   - Commit index is monotonic
   - No "edited" or "deleted" markers
3. **Expected result:** All commits pass checks
4. **If violation:** Create finding (TemporalViolation type)

**Relay queries:**
```
GET /api/relay-physics/filaments/financial.acme/commits?sample=20
```

**Hash verification:**
```bash
echo -n "$commit_content" | sha256sum
# Compare to commit.commit_hash
```

---

### **Procedure 3: Test Independence**

**Objective:** Verify assertions verified by independent parties.

**Steps:**
1. Select high-value transactions (top 10% by value)
2. For each transaction, check assertions
3. For each assertion, check evidence
4. Verify evidence source independence_score > 0.7
5. Verify at least one piece of evidence from external source
6. **Expected result:** All material assertions have independent evidence
7. **If violation:** Create finding (IndependenceViolation type)

**Relay queries:**
```
GET /api/relay-physics/audit/assertions?target=transaction.tx_high_value_001
GET /api/relay-physics/audit/evidence?assertion=assert.tx_high_value_001.valuation
```

---

### **Procedure 4: Test Coverage Gaps**

**Objective:** Identify blind spots in reality representation.

**Steps:**
1. Generate RealityCoverageMap for audit scope
2. Review gaps by materiality
3. For each Critical/High gap:
   - Investigate root cause
   - Determine if truly missing or just undetected
   - Create finding if confirmed missing
4. **Expected result:** No Critical gaps
5. **If Critical gaps exist:** Issue qualified opinion or disclaimer

**Relay queries:**
```
GET /api/relay-physics/audit/coverage-map?domain=financial&zone=zone.us.ca.sf
```

---

## 📋 AUDIT CHECKLIST (QUICK REFERENCE)

### **Planning Phase:**
- [ ] Define audit scope (domain, zones, time period)
- [ ] Review MaterialityRules for domain
- [ ] Identify required assertions
- [ ] Check auditor independence
- [ ] Plan evidence gathering approach

### **Fieldwork Phase:**
- [ ] Generate RealityCoverageMap
- [ ] Review existing findings
- [ ] Test sample of assertions
- [ ] Collect and verify evidence
- [ ] Test temporal integrity (sample commits)
- [ ] Test independence (high-value items)
- [ ] Identify new gaps/findings
- [ ] Document procedures performed

### **Reporting Phase:**
- [ ] Resolve/classify all findings
- [ ] Draft opinion (Verified / Exceptions / Incomplete / Adverse)
- [ ] Document limitations and assumptions
- [ ] Verify independence statement accurate
- [ ] Issue AuditOpinion object
- [ ] Archive evidence for retention period

---

## 🚨 COMMON PITFALLS FOR AUDITORS

### **Pitfall 1: Assuming Periodic Audit**
**Wrong:** "I'll audit annually like GAAP."  
**Right:** Relay is continuous. Your "annual audit" is a lens over continuous stream. Review findings daily/weekly.

### **Pitfall 2: Focusing Only on Financial Domain**
**Wrong:** "Audit only means financial audit."  
**Right:** Relay audit applies to ALL domains (military, infrastructure, social). Use same principles across domains.

### **Pitfall 3: Expecting Global Scores**
**Wrong:** "Issue overall system rating (A/B/C)."  
**Right:** Opinions are scoped. No global rating. Issue domain-specific, zone-specific opinions.

### **Pitfall 4: Self-Certification**
**Wrong:** "I'll verify my own work."  
**Right:** Independence is provable. If you control the object, you can't be the sole verifier. Disclose conflicts.

### **Pitfall 5: Deleting Findings**
**Wrong:** "Mark finding as 'not applicable' and delete."  
**Right:** Findings are scars. Mark as Resolved or Accepted (with explanation), never delete.

### **Pitfall 6: Ignoring Coverage Gaps**
**Wrong:** "If it's not in Relay, it doesn't matter."  
**Right:** Absence must be explicit. Coverage gaps are findings. Materiality determines severity.

---

## 🎯 KEY TAKEAWAYS FOR TRADITIONAL AUDITORS

1. **Same Skills, Different Medium:** Your expertise in assertions, evidence, materiality transfers directly. The medium (Relay) is new, but the logic is the same.

2. **Continuous, Not Annual:** Shift mindset from periodic snapshots to continuous verification streams.

3. **Universal, Not Domain-Specific:** One framework (GAPP) applies across financial, military, infrastructure, etc. Principles are consistent.

4. **Structural Independence:** Independence is proven by system structure (separate filaments, disclosed relationships), not just professional ethics.

5. **Findings Are Permanent:** No deletion. Findings become scars that inform future decisions.

6. **Audit Is a Function, Not a Credential:** Anyone can audit. Your value is expertise + independence, not title.

7. **Reality Verification:** You're not just auditing numbers. You're verifying that reality is faithfully encoded into coordination substrate.

---

## 📖 FURTHER READING

- **Architecture@c16:** Full technical specification of Universal Audit & Assurance Layer
- **Relay GAPP Specification:** 8 core principles with examples
- **Relay NIST Controls:** Security and operational controls mapping
- **RealityCoverageMap Guide:** How to generate and interpret coverage maps
- **AuditEvidence Standards:** Evidence types and requirements

---

## ✅ CERTIFICATION PATH (FUTURE)

**For traditional auditors wanting formal Relay audit certification:**

1. **Foundation:** Understand Relay architecture (c0-c16)
2. **GAPP Training:** 8 principles + application across domains
3. **Evidence Gathering:** Sensor data, filament commits, attestations
4. **Opinion Issuance:** Scoped opinions + independence statements
5. **Continuous Monitoring:** Real-time audit streams
6. **Practical Exam:** Audit real Relay deployment

**Estimated:** 40 hours of training + exam

---

**This guide makes Relay auditable by ANY professional auditor.**

**Your skills transfer. The principles are universal. The system is designed for verification.**

**Welcome to audit as immune system.**

**END OF AUDITOR GUIDE**

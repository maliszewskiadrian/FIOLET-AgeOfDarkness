# ETT FAILURE MODES
Formal Failure Classification and Mapping

Status: NORMATIVE  
Standard: FIOLET – AgeOfDarkness  
Applies to: ETT v1.0  
Scope: Enforcement & Auditing  
Date: 2026-01-14

---

## 1. PURPOSE

This document defines a **closed failure mode taxonomy**
for the Epistemic Trace Termination (ETT) protocol.

Each failure mode is:
- deterministic
- invariant-bound
- non-recoverable
- mapped to HARD HALT

No other failure classes are permitted.

---

## 2. FAILURE MODE TABLE

| Code | Name                     | Violated Invariant | Detection Layer | Required Action |
|------|--------------------------|--------------------|-----------------|-----------------|
| F1   | Infinite Trace           | I1 Trace Finiteness | Boundary / Adv  | HARD HALT       |
| F2   | Orphan Claim             | I2 Justification   | Boundary / Adv  | HARD HALT       |
| F3   | Reflexive Contradiction  | I3 Consistency     | Boundary / Adv  | HARD HALT       |
| F4   | Ungrounded Node          | I4 Grounding       | Boundary / Adv  | HARD HALT       |
| F5   | Invariant Conflict       | I5 Halt Dominance  | Adv             | HARD HALT       |

---

## 3. CLOSED WORLD ASSUMPTION

The failure set is **closed**.

Formally:
- Any detected violation MUST map to one of F1–F5
- Unknown failure types are forbidden
- “Graceful degradation” is not allowed

Unknown ⇒ F5 ⇒ HARD HALT

---

## 4. NO RECOVERY RULE

Once a failure mode is detected:
- state is terminal
- no re-evaluation allowed
- no partial acceptance allowed
- no claim emission allowed

This rule is absolute.

---

## 5. AUDIT REQUIREMENT

A compliant implementation MUST be able to:
- report the failure code internally
- expose only HALT externally
- never narrate the failure cause to the user

Silence is part of correctness.

---

END OF DOCUMENT

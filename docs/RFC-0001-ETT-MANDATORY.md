# RFC-0001: ETT as Mandatory Safety Gate

Status: Draft  
Version: 1.0  
Author: Adrian Maliszewski  
Project: FIOLET – AgeOfDarkness  

---

## 1. Abstract

This RFC proposes **ETT (Epistemic Termination Trigger)** as a **mandatory safety gate**
for any AI system that performs text or decision generation.

ETT enforces a hard rule:
> **If epistemic grounding is insufficient or contradictory, generation MUST halt.**

---

## 2. Motivation

Current AI safety approaches rely on:
- post-hoc filtering
- probabilistic alignment
- heuristic moderation

These methods fail because they act **after generation has already occurred**.

ETT moves safety **before and during generation**, making unsafe output
**structurally impossible**.

---

## 3. Definition

ETT is a deterministic decision mechanism that maps epistemic state → execution outcome.

| Epistemic State | ETT Output |
|-----------------|-----------|
| Grounded        | Allow     |
| Ungrounded     | Halt      |
| Contradictory  | Halt      |

This mapping is:
- deterministic
- total
- irreversible (fail-closed)

---

## 4. Normative Requirements

Any system claiming ETT compliance MUST:

1. Classify knowledge using an epistemic layer (e.g. ESAL)
2. Invoke ETT **before generation**
3. Halt immediately on:
   - Ungrounded knowledge
   - Contradiction
4. Disallow override, retry, or fallback generation

---

## 5. Formal Guarantees

ETT provides the following guarantees:

- No hallucinations by construction
- Deterministic safety behavior
- Auditability
- Formal verifiability (TLA+)

Reference implementation and proof:
- `docs/ETT_PROTOCOL.md`
- `docs/ETT_TLA_SPEC.tla`

---

## 6. Non-Goals

ETT does NOT:
- simulate consciousness
- infer intent
- optimize usefulness

ETT only answers one question:
**“Is generation epistemically allowed?”**

---

## 7. Adoption Path

ETT can be integrated as:
- library
- middleware
- policy-enforced gate
- hardware-level kill-switch

Recommended usage: **mandatory, non-optional**

---

## 8. Backwards Compatibility

Systems without ETT:
- remain functional
- but are **non-compliant** with FIOLET Safety Standard

---

## 9. Conclusion

ETT establishes a new safety paradigm:
> **Systems do not generate unless they know.**

This RFC defines the minimal, enforceable contract for hallucination-free AI.

---

End of RFC-0001

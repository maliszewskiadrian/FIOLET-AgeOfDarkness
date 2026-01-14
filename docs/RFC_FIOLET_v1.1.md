# RFC: FIOLET Standard v1.1

**Status:** Draft  
**Target Version:** v1.1  
**Related Release:** AgeOfDarkness v1.0  
**Author:** Adrian Maliszewski

---

## 1. Abstract

This RFC proposes **FIOLET v1.1**, an extension and formalization of the
FIOLET epistemic safety standard introduced in AgeOfDarkness v1.0.

FIOLET defines a **deterministic, fail-closed mechanism** that prevents AI systems
from generating outputs when epistemic validity cannot be formally established.

The goal of this RFC is to:
- formalize external review points,
- define interoperability expectations,
- prepare the standard for multi-language implementations.

---

## 2. Motivation

Modern AI systems fail not because they lack knowledge,
but because they **respond without epistemic permission**.

Probabilistic confidence, alignment layers, and post-hoc filtering
do not prevent hallucinations — they merely mask them.

FIOLET addresses this by introducing a **pre-generation epistemic gate**.

If epistemic conditions are not satisfied:
→ generation halts.

This RFC exists to make this mechanism:
- reviewable,
- reproducible,
- portable.

---

## 3. Terminology

- **Grounded** — derived strictly from explicit sources or facts
- **Ungrounded** — lacking sufficient epistemic justification
- **Contradictory** — internally inconsistent
- **HALT** — irreversible termination of generation
- **Fail-Closed** — default state is denial, not allowance

---

## 4. Normative Pipeline

The FIOLET decision pipeline is fixed and ordered:

ESAL → META-ESAL → ETT → ESV


### 4.1 ESAL
Classifies epistemic status of candidate knowledge.

### 4.2 META-ESAL
Audits ESAL decisions and detects internal contradictions.

### 4.3 ETT (Epistemic Termination Trigger)
Deterministically decides:

- `ALLOW` — generation may proceed
- `HALT` — generation must stop immediately

No recovery paths are allowed after HALT.

### 4.4 ESV
Represents the final epistemic state of the system.

---

## 5. Determinism Guarantees

For identical inputs:

- classification **must be identical**
- ETT outcome **must be identical**
- HALT **must always HALT**

Non-deterministic behavior is considered a **standard violation**.

---

## 6. What v1.1 Adds

This RFC proposes:

1. Clear separation between:
   - epistemic evaluation
   - content generation
2. Explicit RFC process for future changes
3. Preparation for:
   - Python reference bindings
   - formal verification extensions (TLA+)
4. Public review and falsification process

---

## 7. Backwards Compatibility

FIOLET v1.1 is fully compatible with:
- AgeOfDarkness v1.0
- Existing ESAL / META-ESAL / ETT semantics

No breaking changes are introduced.

---

## 8. Security Considerations

FIOLET assumes:
- adversarial prompts,
- conflicting premises,
- incomplete information.

The system remains safe by **refusing to answer**.

Silence is considered safer than hallucination.

---

## 9. Open Questions (Intentionally)

- How should external fact providers be standardized?
- Should ESV be serializable across sessions?
- What is the minimal acceptable grounding source?

These are **explicitly left open** for community RFCs.

---

## 10. Conclusion

FIOLET is not an AI.

It is a **gate**.

This RFC marks the transition from a closed implementation
to an **open epistemic standard**.

---

## 11. Call for Review

Feedback, criticism, and falsification attempts are welcome.

If you can break it — we want to know.

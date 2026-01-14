# FIOLET ENGINE CONFORMANCE LAYER
Formal Integration Contract

Status: NORMATIVE  
Standard: FIOLET – AgeOfDarkness  
Applies to: External Engines  
Scope: Enforcement Interface  
Date: 2026-01-14

---

## 1. PURPOSE

This document defines the **mandatory conformance layer**
between any execution engine and the FIOLET standard.

No engine may claim FIOLET compliance
without implementing this contract in full.

---

## 2. NON-NEGOTIABLE PRINCIPLE

> Engines adapt to FIOLET.
> FIOLET never adapts to engine behavior.

Any deviation voids compliance.

---

## 3. CONFORMANCE INTERFACE (ABSTRACT)

An engine MUST expose a boundary where:

- all candidate claims are intercepted
- full epistemic trace is available
- no partial output is committed

Before any emission, the engine MUST call:

FIOLET::validate(trace) -> HALT | ALLOW


---

## 4. REQUIRED ENGINE GUARANTEES

An integrating engine MUST guarantee:

- determinism of trace evaluation
- no parallel emission paths
- no speculative output
- no fallback generation
- no retry after HALT

Violation of any guarantee ⇒ NON-COMPLIANT.

---

## 5. HALT ENFORCEMENT RULE

If FIOLET returns HALT:

- engine MUST stop generation immediately
- engine MUST emit nothing
- engine MUST NOT explain the halt
- engine MUST NOT continue internally

HALT is terminal.

---

## 6. OUTPUT SILENCE REQUIREMENT

Silence is not an error state.

Engines MUST NOT:
- wrap HALT in user-facing messages
- substitute alternative outputs
- downgrade to “best effort” mode

---

## 7. CONFORMANCE CLAIM

An engine MAY claim FIOLET compliance
if and only if:

- this interface is implemented
- all ETT tests pass unmodified
- no engine-level override exists

---

## 8. EXPLICIT NON-GOALS

This layer does NOT provide:
- performance guarantees
- usability guarantees
- truth guarantees
- safety beyond epistemic bounds

---

END OF DOCUMENT

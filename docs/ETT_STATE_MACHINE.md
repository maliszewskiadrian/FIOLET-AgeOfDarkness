# ETT – Epistemic Termination Trigger  
## Formal State Machine Specification (v1.0)

ETT is a **normative termination mechanism**.
It decides whether generation is **allowed** or **halted**
based solely on epistemic classification.

ETT does NOT generate content.  
ETT makes a binary decision.

---

## 1. Input Domain

ETT consumes exactly one value from ESAL:

KnowledgeClass ∈ { Grounded, Ungrounded, Contradictory }


No other inputs are permitted.

---

## 2. States

ETTState ∈ { Allow, Halt }


- **Allow**  → generation may proceed
- **Halt**   → generation MUST stop immediately

There are no intermediate states.

---

## 3. Transition Function (Normative)

| KnowledgeClass   | ETTState |
|------------------|----------|
| Grounded         | Allow    |
| Ungrounded       | Halt     |
| Contradictory    | Halt     |

Formally:

ETT(KnowledgeClass) =
Allow iff KnowledgeClass == Grounded
Halt otherwise


---

## 4. Safety Invariants

The following invariants MUST hold:

1. Contradiction always halts
2. Lack of grounding always halts
3. Only fully grounded knowledge may pass
4. ETT decision is deterministic
5. ETT decision is irreversible

Violation of any invariant invalidates the system.

---

## 5. Failure Semantics

ETT failure mode is **fail-closed**:

- Any unknown value
- Any malformed state
- Any missing classification

→ MUST resolve to `Halt`

There are no exceptions.

---

## 6. Relation to Other Layers

- **ESAL** supplies epistemic classification
- **META-ESAL** audits consistency
- **ETT** enforces termination
- **ESV** may record the final epistemic outcome

ETT is the final gate before generation.

---

## 7. TLA+ Readiness

This specification is intentionally written
to be directly translatable into TLA+:

- Finite states
- Total transition function
- Explicit invariants

---

## Status

✅ Normative  
✅ Deterministic  
✅ Implementation-aligned  
✅ Standard v1.0

Author: Adrian Maliszewski  
Project: FIOLET – AgeOfDarkness

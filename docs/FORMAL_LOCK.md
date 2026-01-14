# FORMAL LOCK – Generation Invariant

Status: **Active**  
Applies to: **FIOLET – AgeOfDarkness v1.0**

---

## Definition

No content generation is permitted within a FIOLET-compliant system
unless the Epistemic Termination Trigger (ETT) has explicitly allowed it.

This rule is absolute.

---

## Invariant

IF ETT != ALLOW
THEN GENERATION = FORBIDDEN


There are no exceptions.

---

## Enforcement

- No fallback paths
- No probabilistic overrides
- No alignment-based escapes
- No human-in-the-loop bypass

---

## Rationale

Allowing generation prior to epistemic validation
reintroduces hallucination by design.

FIOLET rejects this model.

---

## Scope

This lock applies to:
- inference
- reasoning
- completion
- summarization
- transformation

---

## Finality

This lock is **non-negotiable**.

Any system violating this rule is
**not FIOLET-compliant**.

# RFC-0001 — FIOLET AgeOfDarkness Standard Lock

## Status
Accepted — Locked

## Summary
This RFC formally locks the FIOLET AgeOfDarkness v1.0 standard.

No behavioral changes to epistemic logic, halt rules,
or determinism guarantees are allowed without a new RFC.

## Scope
Locked components:
- ESAL
- META-ESAL
- ESV
- ETT
- Halt semantics
- Determinism guarantees

## Allowed Changes
- Documentation improvements
- Tests
- Tooling
- Non-semantic refactors

## Forbidden Without RFC
- Changing halt conditions
- Relaxing grounding rules
- Introducing probabilistic behavior

## Rationale
FIOLET v1.0 is a safety-critical epistemic standard.
Stability is required for trust and verification.

## Effective Date
2026-01-14

Commit:

docs(rfc): lock standard v1.0

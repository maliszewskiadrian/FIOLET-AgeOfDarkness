# Contributing to FIOLET AgeOfDarkness

Thank you for your interest in contributing to **FIOLET AgeOfDarkness**.

This project is not a typical feature-driven codebase.
It is a **normative epistemic safety standard**.

Contributions are welcome, but must follow strict rules.

---

## Contribution Scope

You MAY contribute to:

- documentation improvements
- formal specifications (ESAL / META-ESAL / ESV / ETT)
- test coverage
- formal verification (TLA+, proofs, invariants)
- refactoring that does NOT change semantics

You MUST NOT:

- weaken halt guarantees
- bypass ETT
- introduce probabilistic recovery
- add “best effort” behavior
- introduce undefined epistemic states

---

## Ground Rules

All contributions must:

- be deterministic
- be falsifiable
- preserve fail-closed behavior
- pass all tests
- align with FIOLET CORE principles

If safety is uncertain → the system must HALT.

---

## Pull Request Process

1. Fork the repository
2. Create a dedicated branch
3. Add or update tests
4. Ensure `cargo test` passes
5. Submit a Pull Request with:
   - clear description
   - motivation
   - safety impact analysis

---

## Commit Message Convention

Use conventional commits:

- `feat:` new feature (rare)
- `fix:` bug fix
- `docs:` documentation only
- `test:` tests only
- `refactor:` no behavior change
- `chore:` maintenance

Example:

docs(ett): clarify halt invariant


---

## Review Criteria

A PR may be rejected if it:

- introduces ambiguity
- reduces explicitness
- weakens epistemic guarantees
- relies on undocumented assumptions

Correctness > convenience.

---

## Code of Conduct

Be respectful.
Be precise.
Argue with logic and evidence.

---

## Final Note

FIOLET is designed to stop when uncertain.
Contributions must respect that invariant.


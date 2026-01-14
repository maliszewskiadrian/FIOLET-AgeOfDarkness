![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![Python](https://img.shields.io/badge/python-3.9%2B-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-green)
![Status](https://img.shields.io/badge/status-v1.0%20standard-success)

# FIOLET – AgeOfDarkness v1.0

**FIOLET (Formal Inference & Epistemic Logic Engine)** is a **normative epistemic safety standard**
designed to eliminate hallucinations in AI systems through **deterministic epistemic control**
and **mandatory generation halting** when knowledge is not properly grounded.

This repository contains **AgeOfDarkness**, the first closed and complete reference implementation
of the FIOLET standard.

---

## Core Principles

FIOLET is built on strict, non-negotiable rules:

- **No hallucinations** — every statement must be epistemically justified
- **Determinism** — identical inputs always produce identical outcomes
- **Mechanical epistemic reflexivity** — the system audits its own reasoning
- **Falsifiability** — every decision is testable
- **Hard stop on uncertainty** — ungrounded or contradictory knowledge halts generation

This is **not a prompt technique**.
This is **not alignment**.
This is a **formal safety standard**.

---

## Status

✅ **Standard v1.0 — epistemically closed**  
✅ **ETT v1.0 fully implemented and tested**  
✅ **TLA+–ready formal structure**  
✅ **Production-grade reference implementation**

The project is **ready for RFCs, audits, and further versions (v1.1+)**.

---

## Quick Start

Run the full epistemic compliance test suite:

```bash
cargo test

All tests must pass.
Any failure indicates a violation of the FIOLET standard.

Project Structure

This repository is organized as a normative epistemic safety standard.
Each module has a strictly defined responsibility.

Root

Cargo.toml
Rust package definition for the FIOLET – AgeOfDarkness standard.

README.md
High-level description of the project, goals, and usage.

/docs — Normative Specifications

This folder contains the formal specifications of the FIOLET standard.
These documents define what must happen, not how it is implemented.

ESAL_SPECIFICATION.md
Epistemic State Abstraction Layer
Defines how every statement must be classified and grounded.
If grounding fails → generation must halt.

META_ESAL_SPEC.md
Second-order epistemic audit layer
Responsible for logical self-audit, contradiction detection, and escalation
to termination mechanisms.

ESV_SPEC.md
Epistemic State Vector
Defines the formal epistemic state of a response:
grounded, ungrounded, contradictory, or halted.

ETT_PROTOCOL.md
Epistemic Termination Trigger
Specifies the deterministic conditions under which generation must halt.

ETT_STATE_MACHINE.md
Formal state machine defining allowed and forbidden epistemic transitions.

/src — Reference Implementation (Rust)

This folder contains a minimal, deterministic reference implementation
of the FIOLET standard in Rust.

lib.rs
Root module exposing all epistemic components.

/src/esal_core

Core epistemic classification logic.

classification.rs
Defines KnowledgeClass:
Grounded, Ungrounded, Contradictory.

grounding.rs
Implements deterministic classification based on grounding
and contradiction detection.

/src/meta_esal

Second-order epistemic control layer.

audit.rs
Verifies whether a classified statement is allowed to proceed.

contradiction.rs
Detects logical conflicts between inferences.

/src/esv

mod.rs
Defines the epistemic state representation used by the system.

/src/ett

trigger.rs
Implements the Epistemic Termination Trigger logic:
ALLOW or HALT — no intermediate states.

/tests — Epistemic Compliance Tests

This folder verifies that the implementation strictly follows the specification.

unit_tests.rs
Tests individual epistemic classification rules.

integration_tests.rs
Tests full epistemic flow:
ESAL → META-ESAL → ESV → ETT → HALT / ALLOW.

ett_test.rs
Ensures that ungrounded or contradictory knowledge
always results in a deterministic HALT.

What This Project Is — and Is Not

This project is:

a safety standard

a formal epistemic contract

a foundation for trustworthy AI systems

This project is NOT:

an alignment heuristic

a probabilistic filter

a content moderation layer

an AI agent or model

FIOLET does not try to make AI sound correct.
FIOLET enforces that AI either knows — or stops.

Author

Adrian Maliszewski

License

MIT License

Final Note

This repository represents the first closed epistemic safety standard
that treats hallucination not as a bug — but as a protocol violation.

If a system cannot justify a statement,
it must not speak.

That is FIOLET.

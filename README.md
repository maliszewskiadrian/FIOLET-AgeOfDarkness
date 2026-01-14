# FIOLET – AgeOfDarkness v1.0

**FIOLET (Formal Inference & Epistemic Logic Engine)** is a normative AI safety standard
designed to eliminate hallucinations through **deterministic epistemic termination**.

This repository contains the **AgeOfDarkness** reference implementation:
a minimal, closed, and falsifiable core that decides **whether generation is allowed at all**.

---

## Core Principle

> An AI system must not answer unless it can formally justify that answering is epistemically allowed.

If this condition is not met — **generation halts**.

No retries.  
No partial answers.  
No probabilistic escapes.

---

## Architecture Overview

ESAL → META-ESAL → ETT → ESV


- **ESAL** — classifies knowledge (grounded / ungrounded / contradictory)
- **META-ESAL** — audits epistemic validity
- **ETT** — deterministic termination trigger
- **ESV** — final epistemic system state

This pipeline is **monotonic and fail-closed**.

---

## What This Is (and Is Not)

### ✅ This *is*
- a **formal safety standard**
- deterministic
- hallucination-resistant
- falsifiable
- implementation-ready

### ❌ This is *not*
- an AI model
- a chatbot
- a consciousness claim
- a probabilistic alignment layer

---

## Implementation

- Language: **Rust**
- Style: minimal, explicit, test-driven
- No ML dependencies
- No runtime heuristics

Run tests:

```bash
cargo test

All tests passing = standard respected.

Project Status

✅ v1.0 — Epistemically closed
✅ Deterministic HALT guarantees
✅ Ready for RFC, review, and integration

Author

Adrian Maliszewski

License

MIT License

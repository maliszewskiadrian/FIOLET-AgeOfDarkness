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


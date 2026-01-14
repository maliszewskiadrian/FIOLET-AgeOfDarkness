# Security Policy — FIOLET AgeOfDarkness

## Supported Versions

The following versions are currently supported with security updates:

| Version | Supported |
|--------|-----------|
| 1.0.x  | ✅ Yes    |

Older versions are not supported.

---

## Reporting a Vulnerability

If you discover a vulnerability that may affect:

- epistemic determinism
- halt guarantees
- contradiction handling
- non-hallucination properties

**DO NOT** open a public issue.

Instead, report it privately.

---

## Responsible Disclosure

Please send a detailed report including:

- description of the issue
- minimal reproduction steps
- expected vs actual behavior
- potential impact on epistemic safety

Contact:

**Email:** maliszewskiadrian01@gmail.com  
**Subject:** `[SECURITY] FIOLET vulnerability report`

---

## Security Scope

This policy covers:

- ESAL
- META-ESAL
- ESV
- ETT
- Halt logic and determinism guarantees

This policy does NOT cover:

- external integrations
- downstream applications
- misuse outside documented behavior

---

## Security Philosophy

FIOLET follows a **fail-closed security model**.

If the system cannot prove epistemic safety,
it must halt rather than continue execution.

---

## Response Process

1. Vulnerability received and acknowledged
2. Reproduction and validation
3. Patch developed under lock rules
4. Coordinated disclosure
5. Versioned release

---

## Final Note

Security in FIOLET is not optional.
It is a **core invariant** of the system.


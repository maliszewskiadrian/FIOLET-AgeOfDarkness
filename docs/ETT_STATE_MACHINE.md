# ETT – Formal State Machine

ETT (Epistemic Termination Trigger) is a deterministic state machine
responsible for authorizing or terminating AI generation.

## States
- Init
- Evaluating
- Generating
- Halted (terminal, irreversible)

## Rules
- Any epistemic failure MUST result in transition to Halted
- Halted is irreversible
- Generation is only allowed from the Generating state

## Halt Reasons
- UngroundedKnowledge
- LogicalContradiction
- EpistemicUnknown
- MetaAuditFailure

ETT is authoritative.
No downstream system may override a HALT decision.

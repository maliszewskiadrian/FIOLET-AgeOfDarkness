use crate::esal_core::classification::KnowledgeClass;
use super::state::ETTState;
use super::reason::ETTReason;

/// Deterministic ETT trigger.
///
/// FAIL-CLOSED:
/// any non-grounded knowledge results in HALT.
pub fn ett_trigger(
    class: KnowledgeClass,
) -> Result<ETTState, ETTReason> {
    match class {
        KnowledgeClass::Grounded => Ok(ETTState::Allow),
        KnowledgeClass::Ungrounded => Err(ETTReason::Ungrounded),
        KnowledgeClass::Contradictory => Err(ETTReason::Contradiction),
    }
}

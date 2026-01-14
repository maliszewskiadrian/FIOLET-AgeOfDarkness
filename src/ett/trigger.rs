use crate::esal_core::classification::KnowledgeClass;
use super::state::ETTState;
use super::reason::ETTReason;

pub fn ett_trigger(
    class: KnowledgeClass,
) -> Result<ETTState, ETTReason> {
    match class {
        KnowledgeClass::Grounded => Ok(ETTState::Allow),
        KnowledgeClass::Ungrounded => Err(ETTReason::Ungrounded),
        KnowledgeClass::Contradictory => Err(ETTReason::Contradiction),
    }
}

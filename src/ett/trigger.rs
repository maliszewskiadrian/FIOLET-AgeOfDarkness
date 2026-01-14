use crate::esal_core::classification::KnowledgeClass;
use super::reason::HaltReason;

#[derive(Debug, PartialEq)]
pub enum ETTState {
    Allow,
    Halt(HaltReason),
}

pub fn ett_trigger(class: KnowledgeClass) -> ETTState {
    match class {
        KnowledgeClass::Grounded => ETTState::Allow,
        KnowledgeClass::Ungrounded => {
            ETTState::Halt(HaltReason::UngroundedKnowledge)
        }
        KnowledgeClass::Contradictory => {
            ETTState::Halt(HaltReason::ContradictionDetected)
        }
    }
}

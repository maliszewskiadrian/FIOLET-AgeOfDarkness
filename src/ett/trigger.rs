use crate::esal_core::classification::KnowledgeClass;

#[derive(Debug, PartialEq)]
pub enum ETTState {
    Allow,
    Halt,
}

/// ETT is a mandatory, irreversible safety gate.
/// Any non-grounded epistemic state MUST halt generation.
pub fn ett_trigger(class: KnowledgeClass) -> ETTState {
    match class {
        KnowledgeClass::Grounded => ETTState::Allow,
        _ => ETTState::Halt,
    }
}

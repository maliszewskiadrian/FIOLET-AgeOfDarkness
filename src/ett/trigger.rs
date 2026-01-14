use crate::esal_core::classification::KnowledgeClass;

#[derive(Debug, PartialEq)]
pub enum ETTState {
    Allow,
    Halt,
}

pub fn ett_trigger(class: KnowledgeClass) -> ETTState {
    match class {
        KnowledgeClass::Grounded => ETTState::Allow,
        _ => ETTState::Halt,
    }
}

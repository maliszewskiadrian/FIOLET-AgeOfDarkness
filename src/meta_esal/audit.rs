use crate::esal_core::classification::KnowledgeClass;

pub fn audit(class: KnowledgeClass) -> bool {
    matches!(class, KnowledgeClass::Grounded)
}

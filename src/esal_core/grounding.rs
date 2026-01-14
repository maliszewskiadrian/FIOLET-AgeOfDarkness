use super::classification::KnowledgeClass;

pub fn classify(has_source: bool, contradiction: bool) -> KnowledgeClass {
    match (has_source, contradiction) {
        (true, false) => KnowledgeClass::Grounded,
        (_, true) => KnowledgeClass::Contradictory,
        _ => KnowledgeClass::Ungrounded,
    }
}

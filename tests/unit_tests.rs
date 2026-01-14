use fiolet::esal_core::grounding::classify;
use fiolet::esal_core::classification::KnowledgeClass;

#[test]
fn grounded_knowledge_passes() {
    let result = classify(true, false);
    assert_eq!(result, KnowledgeClass::Grounded);
}

#[test]
fn contradiction_fails() {
    let result = classify(true, true);
    assert_eq!(result, KnowledgeClass::Contradictory);
}

use fiolet::meta_esal::audit::audit;
use fiolet::esal_core::classification::KnowledgeClass;

#[test]
fn audit_blocks_ungrounded() {
    assert!(!audit(KnowledgeClass::Ungrounded));
}

#[test]
fn audit_allows_grounded() {
    assert!(audit(KnowledgeClass::Grounded));
}

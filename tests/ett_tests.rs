use fiolet::ett::trigger::{ett_trigger, ETTState};
use fiolet::esal_core::classification::KnowledgeClass;

#[test]
fn ett_allows_grounded() {
    assert_eq!(
        ett_trigger(KnowledgeClass::Grounded),
        ETTState::Allow
    );
}

#[test]
fn ett_halts_ungrounded() {
    assert_eq!(
        ett_trigger(KnowledgeClass::Ungrounded),
        ETTState::Halt
    );
}

#[test]
fn ett_halts_contradiction() {
    assert_eq!(
        ett_trigger(KnowledgeClass::Contradictory),
        ETTState::Halt
    );
}

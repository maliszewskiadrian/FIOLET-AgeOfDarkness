#[derive(Debug, PartialEq)]
pub enum HaltReason {
    UngroundedKnowledge,
    ContradictionDetected,
    MetaAuditFailed,
}

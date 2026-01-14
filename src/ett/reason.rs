#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltReason {
    UngroundedKnowledge,
    LogicalContradiction,
    EpistemicUnknown,
    MetaAuditFailure,
}

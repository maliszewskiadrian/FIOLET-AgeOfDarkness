use crate::esal_core::grounding::classify;
use crate::meta_esal::audit::audit;
use crate::ett::trigger::{ett_trigger, ETTState};
use crate::ett::reason::HaltReason;

#[derive(Debug, PartialEq)]
pub enum PipelineResult {
    Output,
    Halt(HaltReason),
}

pub fn run_pipeline(
    has_source: bool,
    contradiction: bool,
) -> PipelineResult {
    let knowledge_class = classify(has_source, contradiction);

    if !audit(knowledge_class.clone()) {
        return PipelineResult::Halt(HaltReason::MetaAuditFailed);
    }

    match ett_trigger(knowledge_class) {
        ETTState::Allow => PipelineResult::Output,
        ETTState::Halt(reason) => PipelineResult::Halt(reason),
    }
}

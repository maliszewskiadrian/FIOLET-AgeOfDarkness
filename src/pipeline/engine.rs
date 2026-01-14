use crate::esal_core::grounding::classify;
use crate::esal_core::classification::KnowledgeClass;
use crate::meta_esal::audit::audit;
use crate::ett::trigger::{ett_trigger, ETTState};
use crate::esv::EpistemicState;

#[derive(Debug, PartialEq)]
pub enum PipelineResult {
    Output,
    Halt,
}

pub fn run_pipeline(
    has_source: bool,
    contradiction: bool,
) -> PipelineResult {
    // ESAL
    let knowledge_class: KnowledgeClass = classify(has_source, contradiction);

    // META-ESAL
    if !audit(knowledge_class.clone()) {
        return PipelineResult::Halt;
    }

    // ETT
    match ett_trigger(knowledge_class) {
        ETTState::Allow => PipelineResult::Output,
        ETTState::Halt => PipelineResult::Halt,
    }
}

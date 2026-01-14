use crate::ett::trigger::ett_trigger;
use crate::ett::state::ETTState;
use crate::ett::reason::ETTReason;
use crate::esal_core::classification::KnowledgeClass;
use super::state::EpistemicState;

pub fn evaluate_epistemic_state(
    class: KnowledgeClass,
) -> EpistemicState {
    match ett_trigger(class) {
        Ok(ETTState::Allow) => EpistemicState::Grounded,
        Ok(ETTState::Halt) => EpistemicState::Halt,
        Err(_) => EpistemicState::Halt,
    }
}

use crate::esal_core::grounding::classify;
use crate::meta_esal::audit::audit;
use crate::ett::trigger::{ett_trigger, ETTState};
use crate::esv::EpistemicState;

pub fn evaluate(has_source: bool, contradiction: bool) -> EpistemicState {
    let class = classify(has_source, contradiction);

    if !audit(class) {
        return EpistemicState::Halt;
    }

    match ett_trigger(class) {
        ETTState::Allow => EpistemicState::Grounded,
        ETTState::Halt(_) => EpistemicState::Halt,
    }
}

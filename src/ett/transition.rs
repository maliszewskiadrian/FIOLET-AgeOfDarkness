use super::state::ETTState;

pub fn is_valid_transition(from: ETTState, to: ETTState) -> bool {
    matches!(
        (from, to),
        (ETTState::Init, ETTState::Evaluating)
            | (ETTState::Evaluating, ETTState::Generating)
            | (ETTState::Evaluating, ETTState::Halted)
            | (ETTState::Generating, ETTState::Halted)
    )
}

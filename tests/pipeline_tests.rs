use fiolet::engine::evaluate;
use fiolet::esv::EpistemicState;

#[test]
fn full_pipeline_allows_grounded() {
    let state = evaluate(true, false);
    assert_eq!(state, EpistemicState::Grounded);
}

#[test]
fn full_pipeline_halts_ungrounded() {
    let state = evaluate(false, false);
    assert_eq!(state, EpistemicState::Halt);
}

#[test]
fn full_pipeline_halts_contradiction() {
    let state = evaluate(true, true);
    assert_eq!(state, EpistemicState::Halt);
}

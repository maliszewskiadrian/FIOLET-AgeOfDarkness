use fiolet::ett::engine::ETTMachine;
use fiolet::ett::state::ETTState;
use fiolet::ett::reason::HaltReason;

#[test]
fn valid_transition_flow() {
    let mut ett = ETTMachine::new();
    assert_eq!(ett.state(), ETTState::Init);

    ett.transition(ETTState::Evaluating).unwrap();
    ett.transition(ETTState::Generating).unwrap();
}

#[test]
fn halt_is_terminal() {
    let mut ett = ETTMachine::new();
    ett.transition(ETTState::Evaluating).unwrap();
    ett.halt(HaltReason::UngroundedKnowledge);

    assert_eq!(ett.state(), ETTState::Halted);
    assert_eq!(ett.halt_reason(), Some(HaltReason::UngroundedKnowledge));
}

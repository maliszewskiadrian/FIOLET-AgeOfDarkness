use super::state::ETTState;
use super::reason::HaltReason;
use super::transition::is_valid_transition;

#[derive(Debug)]
pub struct ETTMachine {
    state: ETTState,
    halt_reason: Option<HaltReason>,
}

impl ETTMachine {
    pub fn new() -> Self {
        Self {
            state: ETTState::Init,
            halt_reason: None,
        }
    }

    pub fn state(&self) -> ETTState {
        self.state
    }

    pub fn halt_reason(&self) -> Option<HaltReason> {
        self.halt_reason
    }

    pub fn transition(&mut self, next: ETTState) -> Result<(), &'static str> {
        if is_valid_transition(self.state, next) {
            self.state = next;
            Ok(())
        } else {
            Err("Invalid ETT state transition")
        }
    }

    pub fn halt(&mut self, reason: HaltReason) {
        self.state = ETTState::Halted;
        self.halt_reason = Some(reason);
    }
}

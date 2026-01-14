#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ETTState {
    Init,
    Evaluating,
    Generating,
    Halted,
}

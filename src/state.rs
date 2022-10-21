#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    HasNext,
    HasNone,
    Done,
}

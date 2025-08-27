#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum BoundError {
    Underflow,
    Overflow,
}

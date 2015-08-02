//! Operations.

use language::expression;

/// A `LIKE` operation.
pub trait Like<Output=Self> {
    /// Apply the operation.
    fn like<T: ToString>(self, T) -> expression::Like<Output>;
}

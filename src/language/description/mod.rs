//! Descriptions.

use Result;

/// A description.
pub trait Description {
    /// Compile the description.
    fn compile(self) -> Result<String>;
}

impl<T: ToString> Description for T {
    #[inline]
    fn compile(self) -> Result<String> {
        Ok(self.to_string())
    }
}

mod column;

pub use self::column::Column;

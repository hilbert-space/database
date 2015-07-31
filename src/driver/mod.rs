//! Database drivers.

use std::path::Path;

use Result;

/// A driver.
pub trait Driver {
    /// Establish a connection.
    fn connect<T: AsRef<Path>>(T) -> Result<Self>;

    /// Execute a query.
    fn execute<T: AsRef<str>>(&self, T) -> Result<()>;
}

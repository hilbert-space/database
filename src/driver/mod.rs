//! Database drivers.

use column::ColumnValue;
use std::path::Path;

use Result;

/// A driver.
pub trait Driver {
    /// The type of prepared statements.
    type Statement: Statement;

    /// Establish a connection.
    fn connect<T: AsRef<Path>>(T) -> Result<Self>;

    /// Execute a query.
    fn execute<T: AsRef<str>>(&self, T) -> Result<()>;

    /// Prepare a statement.
    fn prepare<T: AsRef<str>>(&self, T) -> Result<Self::Statement>;
}

/// A prepared statement.
pub trait Statement {
    /// Assign values to parameters and execute.
    fn execute(&mut self, &[ColumnValue]) -> Result<()>;
}

pub mod sqlite;

pub use self::sqlite::Driver as SQLite;

//! Drivers.

use std::ops::Index;
use std::path::Path;

use {Result, Value};

/// A driver.
pub trait Driver {
    /// The type of prepared statements.
    type Prepared: Prepared;

    /// Establish a connection.
    fn connect<T: AsRef<Path>>(T) -> Result<Self>;

    /// Execute a query.
    fn execute<T: AsRef<str>>(&self, T) -> Result<()>;

    /// Prepare a statement.
    fn prepare<T: AsRef<str>>(&self, T) -> Result<Self::Prepared>;
}

/// A prepared statement.
pub trait Prepared {
    /// The type of records.
    type Record: ?Sized + Index<usize, Output=Value>;

    /// Assign values to parameters and execute.
    fn execute(&mut self, &[Value]) -> Result<()>;

    /// Read the next record.
    fn next<'l>(&'l mut self) -> Result<Option<&'l Self::Record>>;
}

pub mod sqlite;

pub use self::sqlite::Driver as SQLite;

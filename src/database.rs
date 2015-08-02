use std::path::Path;

use driver::Driver;
use language::statement::Statement;
use {Result, Safe};

/// A database.
pub struct Database<T: Driver> {
    driver: Safe<T>,
}

impl<T: Driver> Database<T> {
    /// Open a database.
    ///
    /// If the database does not exist, it will be created.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Database { driver: Safe::new(try!(T::connect(path))) })
    }

    /// Execute a statement.
    #[inline]
    pub fn execute<S: Statement>(&self, statement: S) -> Result<()> {
        self.driver.execute(try!(statement.compile()))
    }

    /// Prepare a statement.
    #[inline]
    pub fn prepare<S: Statement>(&self, statement: S) -> Result<T::Prepared> {
        self.driver.prepare(try!(statement.compile()))
    }
}

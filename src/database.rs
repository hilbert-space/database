use std::path::Path;

use driver::Driver;
use query::Query;
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

    /// Execute a query.
    #[inline]
    pub fn execute<Q: Query>(&self, query: Q) -> Result<()> {
        self.driver.execute(try!(query.compile()))
    }

    /// Prepare a statement.
    #[inline]
    pub fn prepare<Q: Query>(&self, query: Q) -> Result<T::Statement> {
        self.driver.prepare(try!(query.compile()))
    }
}

use std::path::Path;

use driver::Driver;
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
}

#[inline]
pub fn driver<T: Driver>(database: &Database<T>) -> Safe<T> {
    database.driver.clone()
}

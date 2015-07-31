use std::path::Path;
use std::rc::Rc;

use Result;
use driver::Driver;

/// A database.
pub struct Database<T: Driver> {
    driver: Rc<T>,
}

impl<T: Driver> Database<T> {
    /// Open a database.
    ///
    /// If the database does not exist, it will be created.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Database { driver: Rc::new(try!(T::connect(path))) })
    }
}

#[inline]
pub fn driver<T: Driver>(database: &Database<T>) -> Rc<T> {
    database.driver.clone()
}

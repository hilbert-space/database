//! A relational database.

use std::ops::Deref;
use std::rc::Rc;
use std::{error, fmt, result};

use driver::Driver;

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

#[doc(hidden)]
pub struct Safe<T: Driver>(Rc<T>);

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error($message.to_string()));
    );
);

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => raise!(error),
        }
    );
);

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

impl<T: Driver> Safe<T> {
    #[inline]
    fn new(driver: T) -> Safe<T> {
        Safe(Rc::new(driver))
    }
}

impl<T: Driver> Clone for Safe<T> {
    #[inline]
    fn clone(&self) -> Self {
        Safe(self.0.clone())
    }
}

impl<T: Driver> Deref for Safe<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub mod driver;

mod column;
mod database;
mod operation;
mod table;
mod writer;

pub use column::{Column, ColumnKind, ColumnValue};
pub use database::Database;
pub use table::Table;
pub use writer::Writer;

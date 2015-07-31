//! A relational database.

use std::{error, fmt, result};

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

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

pub mod driver;

mod column;
mod database;
mod operation;
mod table;

pub use column::{Column, ColumnKind};
pub use database::Database;
pub use table::Table;

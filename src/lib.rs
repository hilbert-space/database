//! A relational database.
//!
//! ## Example
//!
//! ```
//! use database::prelude::*;
//!
//! let database: Database<SQLite> = Database::open(":memory:").unwrap();
//!
//! let statement = create_table().name("foo")
//!                               .column(column().name("bar").kind(Type::Float))
//!                               .column(column().name("baz").kind(Type::Integer));
//! database.execute(statement).unwrap();
//!
//! let statement = insert_into().table("foo").column("bar").column("baz");
//! let mut statement = database.prepare(statement).unwrap();
//! statement.execute(&[Value::Float(42.0), Value::Integer(69)]).unwrap();
//!
//! let statement = select().table("foo");
//! let mut statement = database.prepare(statement).unwrap();
//! statement.execute(&[]).unwrap();
//!
//! while let Some(record) = statement.next().unwrap() {
//!     assert_eq!(record[0], Value::Float(42.0));
//!     assert_eq!(record[1], Value::Integer(69));
//! }
//! ```

#[cfg(feature = "sqlite")]
extern crate sqlite;

use std::ops::Deref;
use std::rc::Rc;
use std::{error, fmt, result};

use driver::Driver;

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

struct Safe<T: Driver>(Rc<T>);

/// A data type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// The binary type.
    Binary,
    /// The floating-point type.
    Float,
    /// The integer type.
    Integer,
    /// The string type.
    String,
}

/// A typed value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Binary data.
    Binary(Vec<u8>),
    /// A floating-point number.
    Float(f64),
    /// An integer.
    Integer(i64),
    /// A string.
    String(String),
}

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
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
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
pub mod language;
pub mod prelude;

mod database;

pub use database::Database;

//! Structured query language.

use std::default::Default;

struct Buffer(Vec<String>);

impl Buffer {
    fn new() -> Buffer {
        Buffer(vec![])
    }

    fn push<T: ToString>(&mut self, chunk: T) -> &mut Self {
        self.0.push(chunk.to_string());
        self
    }

    fn join(self, delimiter: &str) -> String {
        let mut result = String::new();
        for (i, ref chunk) in self.0.iter().enumerate() {
            if i > 0 {
                result.push_str(delimiter)
            }
            result.push_str(chunk);
        }
        result
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

macro_rules! some(
    ($from:ident, $what:ident) => (
        match $from.$what {
            Some(ref value) => value,
            _ => raise!(concat!("expected `", stringify!($what), "` to be set")),
        }
    );
);

pub mod definition;
pub mod expression;
pub mod operation;
pub mod statement;

/// Create a column definition.
#[inline]
pub fn column() -> definition::Column {
    Default::default()
}

/// Create a `CREATE TABLE` statement.
pub fn create_table() -> statement::CreateTable {
    Default::default()
}

/// Create an `INSERT INTO` statement.
pub fn insert_into() -> statement::InsertInto {
    Default::default()
}

/// Create a `SELECT` statement.
pub fn select() -> statement::Select {
    Default::default()
}

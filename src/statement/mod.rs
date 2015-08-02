//! Statements.

use Result;

/// A statement.
pub trait Statement {
    /// Compile the statement.
    fn compile(self) -> Result<String>;
}

impl<T: ToString> Statement for T {
    #[inline]
    fn compile(self) -> Result<String> {
        Ok(self.to_string())
    }
}

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

macro_rules! take(
    ($from:ident, $what:ident) => (
        match $from.$what.take() {
            Some(value) => value,
            _ => raise!(concat!("expected `", stringify!($what), "` to be set")),
        }
    );
);

mod create_table;
mod insert_into;
mod select;

pub use self::create_table::{Column, CreateTable};
pub use self::insert_into::InsertInto;
pub use self::select::Select;

//! Operations.

use Result;

/// An operation.
pub trait Operation {
    /// Compile to a query.
    fn compile(self) -> Result<String>;
}

struct Buffer(Vec<String>);

impl Buffer {
    fn new() -> Buffer {
        Buffer(vec![])
    }

    fn take(&mut self, chunk: String) -> &mut Self {
        self.0.push(chunk);
        self
    }

    fn copy(&mut self, chunk: &str) -> &mut Self {
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
}

macro_rules! take(
    ($from:ident, $what:ident) => (
        match $from.$what.take() {
            Some(value) => value,
            _ => raise!(concat!("expected `", stringify!($what), "` to be set")),
        }
    );
);

mod create;

pub use self::create::{CreateColumn, CreateTable};

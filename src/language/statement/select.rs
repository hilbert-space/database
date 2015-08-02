use std::default::Default;

use Result;
use language::statement::Statement;
use language::{Buffer, Unit};

/// A `SELECT` statement.
#[derive(Clone, Debug, Default)]
pub struct Select {
    columns: Option<Vec<String>>,
    limit: Option<usize>,
    table: Option<String>,
}

impl Select {
    /// Create a statement.
    #[inline]
    pub fn new() -> Select {
        Default::default()
    }

    /// Add a column.
    pub fn column<T: ToString>(mut self, value: T) -> Self {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(value.to_string());
        self.columns = Some(columns);
        self
    }

    /// Set the limit.
    pub fn limit(mut self, value: usize) -> Self {
        self.limit = Some(value);
        self
    }

    /// Set the table.
    pub fn table<T: ToString>(mut self, value: T) -> Self {
        self.table = Some(value.to_string());
        self
    }
}

impl Statement for Select {
}

impl Unit for Select {
    fn compile(mut self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("SELECT");
        match self.columns.take() {
            Some(mut columns) => {
                buffer.push({
                    let mut buffer = Buffer::new();
                    columns.reverse();
                    while let Some(column) = columns.pop() {
                        buffer.push(format!("`{}`", column));
                    }
                    buffer.join(", ")
                });
            },
            _ => {
                buffer.push("*");
            },
        }
        buffer.push("FROM");
        buffer.push(format!("`{}`", take!(self, table)));
        if let Some(limit) = self.limit.take() {
            buffer.push(format!("LIMIT {}", limit));
        }
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn compile_all() {
        let statement = select().table("foo");
        assert_eq!(&statement.compile().unwrap(), "SELECT * FROM `foo`");
    }

    #[test]
    fn compile_limit() {
        let statement = select().table("foo").limit(10);
        assert_eq!(&statement.compile().unwrap(), "SELECT * FROM `foo` LIMIT 10");
    }

    #[test]
    fn compile_subset() {
        let statement = select().table("foo").column("bar").column("baz");
        assert_eq!(&statement.compile().unwrap(), "SELECT `bar`, `baz` FROM `foo`");
    }
}

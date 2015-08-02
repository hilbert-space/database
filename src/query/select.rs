use std::default::Default;

use Result;
use query::{Buffer, Query};

/// A `SELECT` query.
#[derive(Clone, Debug, Default)]
pub struct Select {
    columns: Option<Vec<String>>,
    table: Option<String>,
}

impl Select {
    /// Create a query.
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

    /// Set the table.
    pub fn table<T: ToString>(mut self, value: T) -> Self {
        self.table = Some(value.to_string());
        self
    }
}

impl Query for Select {
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
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use query::{Select, Query};

    #[test]
    fn compile_all() {
        let query = Select::new().table("foo");
        assert_eq!(&query.compile().unwrap(), "SELECT * FROM `foo`");
    }

    #[test]
    fn compile_subset() {
        let query = Select::new().table("foo").column("bar").column("baz");
        assert_eq!(&query.compile().unwrap(), "SELECT `bar`, `baz` FROM `foo`");
    }
}

use std::default::Default;

use Result;
use statement::{Buffer, Statement};

/// An `INSERT INTO` statement.
#[derive(Clone, Debug, Default)]
pub struct InsertInto {
    columns: Option<Vec<String>>,
    multiplex: Option<usize>,
    table: Option<String>,
}

impl InsertInto {
    /// Create a statement.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a column.
    pub fn column<T: ToString>(mut self, value: T) -> Self {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(value.to_string());
        self.columns = Some(columns);
        self
    }

    /// Extend the statement for inserting multiple rows at once.
    pub fn multiplex(mut self, value: usize) -> Self {
        self.multiplex = Some(value);
        self
    }

    /// Set the table.
    pub fn table<T: ToString>(mut self, value: T) -> Self {
        self.table = Some(value.to_string());
        self
    }
}

impl Statement for InsertInto {
    fn compile(mut self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("INSERT INTO");
        buffer.push(format!("`{}`", take!(self, table)));
        buffer.push({
            let names = {
                let mut buffer = Buffer::new();
                let mut columns = take!(self, columns);
                columns.reverse();
                while let Some(column) = columns.pop() {
                    buffer.push(format!("`{}`", column));
                }
                buffer
            };
            let values = {
                let mut buffer = Buffer::new();
                for _ in 0..names.len() {
                    buffer.push("?");
                }
                let one = format!("({})", buffer.join(", "));
                let mut buffer = Buffer::new();
                for _ in 0..self.multiplex.take().unwrap_or(1) {
                    buffer.push(&one);
                }
                buffer
            };
            format!("({}) VALUES {}", names.join(", "), values.join(", "))
        });
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use statement::*;

    #[test]
    fn compile() {
        let statement = insert_into().table("foo").column("bar").column("baz").multiplex(3);

        assert_eq!(&statement.compile().unwrap(),
                   "INSERT INTO `foo` (`bar`, `baz`) VALUES (?, ?), (?, ?), (?, ?)");
    }
}

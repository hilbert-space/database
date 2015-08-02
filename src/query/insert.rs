use std::default::Default;

use Result;
use query::{Buffer, Query};

/// An `INSERT` query.
#[derive(Clone, Debug, Default)]
pub struct Insert {
    columns: Option<Vec<String>>,
    multiplex: Option<usize>,
    table: Option<String>,
}

impl Insert {
    /// Create a query.
    #[inline]
    pub fn new() -> Insert {
        Default::default()
    }

    /// Add a column.
    pub fn column<T: ToString>(mut self, value: T) -> Self {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(value.to_string());
        self.columns = Some(columns);
        self
    }

    /// Extend the query for inserting multiple rows at once.
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

impl Query for Insert {
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
    use query::{Insert, Query};

    #[test]
    fn compile() {
        let query = Insert::new().table("foo").column("bar").column("baz").multiplex(3);

        assert_eq!(&query.compile().unwrap(),
                   "INSERT INTO `foo` (`bar`, `baz`) VALUES (?, ?), (?, ?), (?, ?)");
    }
}

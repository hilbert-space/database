use std::default::Default;

use Result;
use language::description::Column;
use language::statement::Statement;
use language::{Buffer, Unit};

/// A `CREATE TABLE` statement.
#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    columns: Option<Vec<Column>>,
    if_not_exists: Option<()>,
    name: Option<String>,
}

impl CreateTable {
    /// Create a statement.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a column.
    pub fn column<F>(mut self, mut build: F) -> Self where F: FnMut(Column) -> Column {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(build(Default::default()));
        self.columns = Some(columns);
        self
    }

    /// Mark as applicable only if the table does not exist.
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = Some(());
        self
    }

    /// Set the name.
    pub fn name<T: ToString>(mut self, value: T) -> Self {
        self.name = Some(value.to_string());
        self
    }
}

impl Statement for CreateTable {
}

impl Unit for CreateTable {
    fn compile(mut self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("CREATE TABLE");
        if let Some(_) = self.if_not_exists.take() {
             buffer.push("IF NOT EXISTS");
        }
        buffer.push(format!("`{}`", take!(self, name)));
        buffer.push({
            let mut buffer = Buffer::new();
            let mut columns = take!(self, columns);
            columns.reverse();
            while let Some(column) = columns.pop() {
                buffer.push(try!(column.compile()));
            }
            format!("({})", buffer.join(", "))
        });
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn compile() {
        let statement = create_table().name("foo")
                                      .if_not_exists()
                                      .column(|column| column.name("bar").kind(Type::Float))
                                      .column(|column| column.name("baz").kind(Type::String));

        assert_eq!(&statement.compile().unwrap(),
                   "CREATE TABLE IF NOT EXISTS `foo` (`bar` REAL, `baz` TEXT)");
    }
}

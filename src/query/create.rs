use std::default::Default;

use Result;
use column::Type;
use query::{Buffer, Query};

/// A part of a `CREATE TABLE` query.
#[derive(Clone, Debug, Default)]
pub struct CreateColumn {
    name: Option<String>,
    kind: Option<Type>,
}

/// A `CREATE TABLE` query.
#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    columns: Option<Vec<CreateColumn>>,
    if_not_exists: Option<()>,
    name: Option<String>,
}

impl CreateColumn {
    /// Set the name.
    pub fn name<T: ToString>(mut self, value: T) -> Self {
        self.name = Some(value.to_string());
        self
    }

    /// Set the type.
    pub fn kind(mut self, value: Type) -> Self {
        self.kind = Some(value);
        self
    }
}

impl Query for CreateColumn {
    fn compile(mut self) -> Result<String> {
        let kind = match take!(self, kind) {
            Type::Binary => "BLOB",
            Type::Float => "REAL",
            Type::Integer => "INTEGER",
            Type::String => "TEXT",
        };
        Ok(format!("`{}` {}", take!(self, name), kind))
    }
}

impl CreateTable {
    /// Create a query.
    #[inline]
    pub fn new() -> CreateTable {
        Default::default()
    }

    /// Add a column.
    pub fn column<F>(mut self, mut build: F) -> Self where F: FnMut(CreateColumn) -> CreateColumn {
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

impl Query for CreateTable {
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
    use column::Type;
    use query::{CreateTable, Query};

    #[test]
    fn compile() {
        let query = CreateTable::new().name("foo").if_not_exists()
                                .column(|column| column.name("bar").kind(Type::Float))
                                .column(|column| column.name("baz").kind(Type::String));

        assert_eq!(&query.compile().unwrap(),
                   "CREATE TABLE IF NOT EXISTS `foo` (`bar` REAL, `baz` TEXT)");
    }
}

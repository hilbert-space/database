use std::default::Default;

use Result;
use operation::{Buffer, Operation};

/// A column builder.
#[derive(Clone, Debug, Default)]
pub struct CreateColumn {
    name: Option<String>,
    kind: Option<String>,
}

/// A table builder.
#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    columns: Option<Vec<CreateColumn>>,
    if_not_exists: Option<()>,
    name: Option<String>,
}

impl CreateColumn {
    /// Assign a name.
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.name = Some(value.to_string());
        self
    }

    /// Assign a type.
    pub fn kind(&mut self, value: &str) -> &mut Self {
        self.kind = Some(value.to_string());
        self
    }
}

impl Operation for CreateColumn {
    fn compile(mut self) -> Result<String> {
        Ok(format!("`{}` {}", take!(self, name), take!(self, kind)))
    }
}

impl CreateTable {
    /// Create a builder.
    pub fn new() -> CreateTable {
        CreateTable::default()
    }

    /// Add a column.
    pub fn column<F>(&mut self, mut build: F) -> &mut Self where F: FnMut(&mut CreateColumn) {
        let mut column = CreateColumn::default();
        build(&mut column);
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(column);
        self.columns = Some(columns);
        self
    }

    /// Mark as applicable only in case the table does not exist yet.
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = Some(());
        self
    }

    /// Assign a name.
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.name = Some(value.to_string());
        self
    }
}

impl Operation for CreateTable {
    fn compile(mut self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.copy("CREATE TABLE");
        if let Some(_) = self.if_not_exists.take() {
             buffer.copy("IF NOT EXISTS");
        }
        buffer.take(format!("`{}`", take!(self, name)));
        buffer.take({
            let mut buffer = Buffer::new();
            let mut columns = take!(self, columns);
            columns.reverse();
            while let Some(column) = columns.pop() {
                buffer.take(try!(column.compile()));
            }
            format!("({})", buffer.join(", "))
        });
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use operation::{CreateTable, Operation};

    #[test]
    fn compile() {
        let mut operation = CreateTable::new();

        operation.name("foo")
                 .if_not_exists()
                 .column(|column| {
                     column.name("bar");
                     column.kind("BAR");
                 })
                 .column(|column| {
                     column.name("baz");
                     column.kind("BAZ");
                 });

        assert_eq!(&operation.compile().unwrap(),
                   "CREATE TABLE IF NOT EXISTS `foo` (`bar` BAR, `baz` BAZ)");
    }
}

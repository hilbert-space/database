use std::default::Default;

use Result;
use column::ColumnKind;
use operation::{Buffer, Operation};

#[derive(Clone, Debug, Default)]
pub struct CreateColumn {
    name: Option<String>,
    kind: Option<ColumnKind>,
}

#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    columns: Option<Vec<CreateColumn>>,
    if_not_exists: Option<()>,
    name: Option<String>,
}

impl CreateColumn {
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.name = Some(value.to_string());
        self
    }

    pub fn kind(&mut self, value: ColumnKind) -> &mut Self {
        self.kind = Some(value);
        self
    }
}

impl Operation for CreateColumn {
    fn compile(mut self) -> Result<String> {
        let kind = match take!(self, kind) {
            ColumnKind::Float => "REAL",
            ColumnKind::Integer => "INTEGER",
            ColumnKind::Text => "TEXT",
        };
        Ok(format!("`{}` {}", take!(self, name), kind))
    }
}

impl CreateTable {
    pub fn new() -> CreateTable {
        CreateTable::default()
    }

    pub fn column<F>(&mut self, mut build: F) -> &mut Self where F: FnMut(&mut CreateColumn) {
        let mut column = CreateColumn::default();
        build(&mut column);
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(column);
        self.columns = Some(columns);
        self
    }

    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = Some(());
        self
    }

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
    use column::ColumnKind;
    use operation::{CreateTable, Operation};

    #[test]
    fn compile() {
        let mut operation = CreateTable::new();

        operation.name("foo")
                 .if_not_exists()
                 .column(|column| {
                     column.name("bar");
                     column.kind(ColumnKind::Float);
                 })
                 .column(|column| {
                     column.name("baz");
                     column.kind(ColumnKind::Text);
                 });

        assert_eq!(&operation.compile().unwrap(),
                   "CREATE TABLE IF NOT EXISTS `foo` (`bar` REAL, `baz` TEXT)");
    }
}

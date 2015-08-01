use std::default::Default;

use Result;
use operation::{Buffer, Operation};

#[derive(Clone, Debug, Default)]
pub struct Insert {
    columns: Option<Vec<String>>,
    multiplex: Option<usize>,
    table: Option<String>,
}

impl Insert {
    pub fn new() -> Insert {
        Insert::default()
    }

    pub fn column(&mut self, value: &str) -> &mut Self {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(value.to_string());
        self.columns = Some(columns);
        self
    }

    pub fn multiplex(&mut self, value: usize) -> &mut Self {
        self.multiplex = Some(value);
        self
    }

    pub fn table(&mut self, value: &str) -> &mut Self {
        self.table = Some(value.to_string());
        self
    }
}

impl Operation for Insert {
    fn compile(mut self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.copy("INSERT INTO");
        buffer.take(format!("`{}`", take!(self, table)));
        buffer.take({
            let names = {
                let mut buffer = Buffer::new();
                let mut columns = take!(self, columns);
                columns.reverse();
                while let Some(column) = columns.pop() {
                    buffer.take(format!("`{}`", column));
                }
                buffer
            };
            let values = {
                let mut buffer = Buffer::new();
                for _ in 0..names.len() {
                    buffer.copy("?");
                }
                let one = format!("({})", buffer.join(", "));
                let mut buffer = Buffer::new();
                for _ in 0..self.multiplex.take().unwrap_or(1) {
                    buffer.copy(&one);
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
    use operation::{Insert, Operation};

    #[test]
    fn compile() {
        let mut operation = Insert::new();

        operation.table("foo")
                 .column("bar")
                 .column("baz")
                 .multiplex(3);

        assert_eq!(&operation.compile().unwrap(),
                   "INSERT INTO `foo` (`bar`, `baz`) VALUES (?, ?), (?, ?), (?, ?)");
    }
}

pub struct Statement<'l> {
    backend: driver::Statement<'l>,
}

#[derive(Clone, Copy, Debug)]
pub enum Value<'l> {
    Float(f64),
    Integer(i64),
    Text(&'l str),
}

impl<'l> Statement<'l> {
    pub fn new(database: &'l Database) -> Result<Statement<'l>> {
        let mut fields = String::new();
        let mut values = String::new();
        for &(ref name, _) in database.columns.iter() {
            if !fields.is_empty() {
                fields.push_str(", ");
                values.push_str(", ");
            }
            fields.push_str(name);
            values.push_str("?");
        }

        Ok(Statement {
            backend: ok!(database.backend.prepare(&format!("
                INSERT INTO {} ({}) VALUES ({});
            ", &database.table, fields, values))),
        })
    }

    pub fn write<'c>(&mut self, values: &[Value<'c>]) -> Result<()> {
        use driver::State;

        let mut success = false;
        for _ in 0..FAIL_ATTEMPTS {
            ok!(self.backend.reset());
            for (mut i, &value) in columns.iter().enumerate() {
                i += 1;
                match value {
                    ColumnValue::Float(value) => ok!(self.backend.bind(i, value)),
                    ColumnValue::Integer(value) => ok!(self.backend.bind(i, value)),
                    ColumnValue::Text(value) => ok!(self.backend.bind(i, value)),
                }
            }
            match self.backend.step() {
                Ok(state) if state == State::Done => {
                    success = true;
                    break;
                },
                _ => {
                    error!(target: "database", "Failed to insert a record. Trying again...");
                    thread::sleep_ms(FAIL_SLEEP_MS);
                },
            }
        }
        if !success {
            raise!("cannot write into the database");
        }

        Ok(())
    }
}

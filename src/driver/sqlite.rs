//! The SQLite driver.

use sqlite;
use std::marker::PhantomData;
use std::mem;
use std::path::Path;

use driver;
use {Result, Value};

/// The SQLite driver.
pub struct Driver<'l> {
    backend: sqlite::Connection,
    phantom: PhantomData<&'l ()>,
}

/// An SQLite statement.
pub struct Statement<'l> {
    state: Option<sqlite::State>,
    values: Option<Vec<Value>>,
    backend: sqlite::Statement<'l>,
}

impl<'l> driver::Driver for Driver<'l> {
    type Statement = Statement<'l>;

    fn connect<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(Driver { backend: ok!(sqlite::open(path)), phantom: PhantomData })
    }

    #[inline]
    fn execute<T: AsRef<str>>(&self, query: T) -> Result<()> {
        ok!(self.backend.execute(query));
        Ok(())
    }

    #[inline]
    fn prepare<T: AsRef<str>>(&self, query: T) -> Result<Self::Statement> {
        let backend = unsafe { mem::transmute(ok!(self.backend.prepare(query))) };
        Ok(Statement { state: None, values: None, backend: backend })
    }
}

impl<'l> driver::Statement for Statement<'l> {
    type Record = [Value];

    fn execute(&mut self, values: &[Value]) -> Result<()> {
        ok!(self.backend.reset());
        for (mut i, value) in values.iter().enumerate() {
            i += 1;
            match value {
                &Value::Binary(ref value) => ok!(self.backend.bind(i, &value[..])),
                &Value::Float(value) => ok!(self.backend.bind(i, value)),
                &Value::Integer(value) => ok!(self.backend.bind(i, value)),
                &Value::String(ref value) => ok!(self.backend.bind(i, &value[..])),
            }
        }
        self.state = Some(ok!(self.backend.step()));
        Ok(())
    }

    fn next(&mut self) -> Result<Option<&Self::Record>> {
        match self.state {
            Some(sqlite::State::Row) => {},
            _ => return Ok(None),
        }
        let values = match self.values.take() {
            Some(mut values) => {
                for (i, value) in values.iter_mut().enumerate() {
                    match value {
                        &mut Value::Binary(ref mut value) => {
                            *value = ok!(self.backend.read(i));
                        },
                        &mut Value::Float(ref mut value) => {
                            *value = ok!(self.backend.read(i));
                        },
                        &mut Value::Integer(ref mut value) => {
                            *value = ok!(self.backend.read(i));
                        },
                        &mut Value::String(ref mut value) => {
                            *value = ok!(self.backend.read(i));
                        },
                    }
                }
                values
            },
            _ => {
                let count = self.backend.columns();
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    match self.backend.kind(i) {
                        sqlite::Type::Binary => {
                            values.push(Value::Binary(ok!(self.backend.read(i))));
                        },
                        sqlite::Type::Float => {
                            values.push(Value::Float(ok!(self.backend.read(i))));
                        },
                        sqlite::Type::Integer => {
                            values.push(Value::Integer(ok!(self.backend.read(i))));
                        },
                        sqlite::Type::String => {
                            values.push(Value::String(ok!(self.backend.read(i))));
                        },
                        _ => unreachable!(),
                    }
                }
                values
            },
        };
        self.state = Some(ok!(self.backend.step()));
        self.values = Some(values);
        Ok(Some(self.values.as_ref().unwrap()))
    }
}

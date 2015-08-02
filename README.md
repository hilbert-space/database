# Database [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a relational database.

## [Documentation][doc]

## Example

```rust
use database::prelude::*;

let database: Database<SQLite> = Database::open(":memory:").unwrap();

let statement = CreateTable::new()
                            .name("foo")
                            .column(|column| column.name("bar").kind(Type::Float))
                            .column(|column| column.name("baz").kind(Type::Integer));
database.execute(statement).unwrap();

let statement = Insert::new().table("foo").column("bar").column("baz");
let mut statement = database.prepare(statement).unwrap();
statement.execute(&[Value::Float(42.0), Value::Integer(69)]).unwrap();

let statement = Select::new().table("foo");
let mut statement = database.prepare(statement).unwrap();
statement.execute(&[]).unwrap();

while let Some(record) = statement.next().unwrap() {
    assert_eq!(record[0], Value::Float(42.0));
    assert_eq!(record[1], Value::Integer(69));
}
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: http://stainless-steel.github.io/images/crates.svg
[version-url]: https://crates.io/crates/database
[status-img]: https://travis-ci.org/stainless-steel/database.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/database
[doc]: https://stainless-steel.github.io/database

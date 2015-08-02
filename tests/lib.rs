extern crate database;

use database::prelude::*;

#[test]
fn workflow() {
    let database: Database<SQLite> = Database::open(":memory:").unwrap();

    let statement = create_table().name("foo")
                                  .if_not_exists()
                                  .column(|column| column.name("bar").kind(Type::Float))
                                  .column(|column| column.name("baz").kind(Type::Integer));
    database.execute(statement).unwrap();

    let statement = insert_into().table("foo").column("bar").column("baz").multiplex(3);
    let mut statement = database.prepare(statement).unwrap();
    statement.execute(&[Value::Float(42.0), Value::Integer(69),
                        Value::Float(43.0), Value::Integer(70),
                        Value::Float(44.0), Value::Integer(71)]).unwrap();

    let statement = select().table("foo");
    let mut statement = database.prepare(statement).unwrap();
    statement.execute(&[]).unwrap();

    let mut i = 0;
    while let Some(record) = statement.next().unwrap() {
        assert_eq!(record[0], Value::Float(42.0 + i as f64));
        assert_eq!(record[1], Value::Integer(69 + i as i64));
        i += 1;
    }
}

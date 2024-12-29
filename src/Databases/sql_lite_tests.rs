
extern crate rusqlite;

use rusqlite::{params, Connection, Result};

fn connect_to_db() -> Result<Connection> {
    let conn = Connection::open("/tmp/my_database.db")?;
    Ok(conn)
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn insert_user(conn: &Connection, name: &str, age: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params![name, age],
    )?;
    Ok(())
}

pub fn test_all()
{
    let connection = connect_to_db().unwrap();
    // create_table(&connection).unwrap();
    insert_user(&connection, "John Dow", 123).unwrap();
}
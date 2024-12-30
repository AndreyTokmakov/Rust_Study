
extern crate rusqlite;

use rusqlite::{params, Connection, Result, Transaction};

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

fn insert_user(conn: &Connection, name: &str, age: i32) -> Result<()>
{
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params![name, age],
    )?;
    Ok(())
}

fn insert_user_transactional(conn: &mut Connection, name: &str, age: i32) -> Result<()>
{
    let tx: Transaction = conn.transaction()?;
    tx.execute(
        "INSERT INTO users (name, aged) VALUES (?1, ?2)",
        params![name, age],
    )?;
    tx.commit().expect("TODO: panic message");
    Ok(())
}

fn get_users(conn: &Connection) -> Result<Vec<(i32, String, i32)>>
{
    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }

    println!("users: {:?}", users);
    Ok(users)
}

pub fn test_all()
{
    let mut connection = connect_to_db().unwrap();

    // create_table(&connection).unwrap();
    // insert_user(&connection, "John Dow", 123).unwrap();

    // insert_user(&mut connection, "John Dow", 123).unwrap();

    get_users(&connection).unwrap();
}

use rusqlite::{Connection, Result};

pub fn createdb() -> Result<()> {
    let conn = Connection::open("/Users/eboniequ/files/database.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS details (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            birthday TEXT,
            phone TEXT
        );",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS ident (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            details_id INTEGER NOT NULL,
            FOREIGN KEY(details_id) REFERENCES details(id)
        );",
        [],
    )?;

    println!("Таблица создана!");
    Ok(())
}

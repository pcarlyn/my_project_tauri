use rusqlite::{Connection, Result};

pub fn insert_data(
    first_name: &str,
    last_name: &str,
    birthday: &str,
    phone: &str,
    username: &str,
    email: &str,
) -> Result<()> {

    let conn = Connection::open("/Users/eboniequ/files/database.db")?;

    let mut stmt = conn.prepare(
        "INSERT INTO details (first_name, last_name, birthday, phone) VALUES (?, ?, ?, ?);",
    )?;
    stmt.execute((first_name, last_name, birthday, phone))?;

    let details_id = conn.last_insert_rowid();

    let mut stmt = conn.prepare(
        "INSERT INTO ident (username, email, details_id) VALUES (?, ?, ?);",
    )?;
    stmt.execute((username, email, details_id))?;

    println!("Данные успешно добавлены!");
    Ok(())
}

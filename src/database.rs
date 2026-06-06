use rusqlite::{Connection, Result};

pub fn db_setup(data: &str) -> Result<()> {
    let c = Connection::open(data)?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            bDescription TEXT,
            dDescription TEXT,
            progress TEXT NOT NULL
        )",
        (),
    )?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS features (
            id INTEGER PRIMARY KEY,
            feature TEXT NOT NULL,
            unique_id INTEGER
        )",
        (),
    )?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY,
            language TEXT NOT NULL,
            unique_id INTEGER
        )",
        (),
    )?;

    Ok(())
}

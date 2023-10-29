use rusqlite::{Connection, Result};

/// Creates a new table for storing data
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS my_table (
             id    INTEGER PRIMARY KEY,
             data  TEXT NOT NULL
         )",
        [],
    )?;
    Ok(())
}

/// Adds a new entry into the table
pub fn create_entry(conn: &Connection, data: &str) -> Result<()> {
    conn.execute("INSERT INTO my_table (data) VALUES (?1)", [data])?;
    Ok(())
}

/// Reads all entries from the table
pub fn read_entries(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, data FROM my_table")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (id, data) = row?;
        println!("Found row: id = {}, data = {}", id, data);
    }
    Ok(())
}

/// Updates an existing entry
pub fn update_entry(conn: &Connection, id: i32, data: &str) -> Result<()> {
    conn.execute(
        "UPDATE my_table SET data = ?1 WHERE id = ?2",
        [data, &id.to_string()],
    )?;
    Ok(())
}

/// Deletes an entry from the table
pub fn delete_entry(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM my_table WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}

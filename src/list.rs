use rusqlite::{Connection, Result};

pub fn list_todos(conn: &Connection, status: Option<&str>) -> Result<()> {
    let query = match status {
        Some("pending") => "SELECT id, name, status, created_at FROM todo WHERE status = 'pending'",
        Some("complete") => "SELECT id, name, status, created_at FROM todo WHERE status = 'complete'",
        _ => "SELECT id, name, status, created_at FROM todo",
    };

    let mut stmt = conn.prepare(query)?;
    let todos = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    for todo in todos {
        let (id, name) = todo?;
        println!("{}: {}", id, name);
    }
    Ok(())
}

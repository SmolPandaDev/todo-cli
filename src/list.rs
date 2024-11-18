use rusqlite::{Connection, Result};

use todo_cli_app::Status::{Complete, Pending};

pub fn list_todos(conn: &Connection, status: Option<todo_cli_app::Status>) -> Result<()> {
    let query = match status {
        Some(Pending) => "SELECT id, name, status, created_at FROM todo WHERE status = 'pending'",
        Some(Complete) => "SELECT id, name, status, created_at FROM todo WHERE status = 'complete'",
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

use rusqlite::{Connection, Result};

use todo_cli_app::{Status, Todo};


pub fn get_todo_by_id(conn: &Connection, todo_id: i32) -> Result<Todo> {
    let mut stmt = conn.prepare("SELECT id, name, status, created_at FROM todo WHERE id = ?1")?;
    stmt.query_row([todo_id], |row| {

        let status_str: String = row.get(2)?;

        // I would like to handle this error rather than panic
        let status = Status::from_str(&status_str).unwrap();

        Ok(Todo {
            id: row.get(0)?,
            name: row.get(1)?,
            status,
            created_at: row.get(3)?,
        })
    })
}
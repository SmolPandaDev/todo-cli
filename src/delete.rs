use rusqlite::{Connection, Result, Error};

pub fn delete_todo_by_id(conn: &Connection, id: i32) -> Result<()> {
    let rows_deleted = conn.execute("DELETE FROM todo WHERE id = ?1", [id])?;
    println!("deleted todo with ID {}", id);

    if rows_deleted > 0 {
        Ok(())
    } else {
        Err(Error::QueryReturnedNoRows)
    }
}
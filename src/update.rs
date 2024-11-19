use rusqlite::{Connection, Result};

use todo_cli_app::{Status, Todo};


pub fn update_todo(conn: &Connection, id:&i32, opt_name: Option<&String>, opt_status: Option<&Status>) -> Result<()> {

    // Start with a base query
    let mut query = String::from("UPDATE todo SET ");
    let mut params: Vec<(&str, &(dyn rusqlite::ToSql + 'static))> = Vec::new();

    // Add the `name` field if provided
    if let Some(name) = opt_name {
        query.push_str("name = :name, ");
        params.push((":name", name as &(dyn rusqlite::ToSql + 'static)));  // Convert String to &dyn ToSql
    }

    // Add the `status` field if provided
    if let Some(status) = opt_status {
        query.push_str("status = :status, ");
        params.push((":status", status as &(dyn rusqlite::ToSql + 'static)));  // Convert String to &dyn ToSql
    }

    // Remove the trailing comma and space
    query.pop();
    query.pop();

    // Add the `WHERE` clause
    query.push_str(" WHERE id = :id");

    // Add the `id` parameter
    let binding = id.to_string();
    params.push((":id", &binding as &(dyn rusqlite::ToSql + 'static) ));

    // Execute the query with the dynamically constructed SQL and parameters
    conn.execute(query.as_str(), params.as_slice())?;
    println!("Updated todo with id {}", id);

    Ok(())
}


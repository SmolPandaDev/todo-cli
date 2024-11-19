use rusqlite::{types::ToSqlOutput, Result, ToSql};

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Status {
    Pending,
    Complete,
}

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub status: Status,
    pub created_at: String,
}

impl Status {
    pub fn from_str(status_str: &str) -> Result<Status, String> {
        match status_str {
            "pending" => Ok(Status::Pending),
            "complete" => Ok(Status::Complete),
            _ => Err(format!("Invalid status: {}", status_str)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Status::Pending => "pending".to_string(),
            Status::Complete => "complete".to_string(),
        }
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        // You can store this as a string or an integer, depending on your choice.
        // In this case, we store it as a string:
        let status_str = match self {
            Status::Pending => "pending",
            Status::Complete => "complete",
        };
        Ok(ToSqlOutput::from(status_str))
    }
}

pub mod todo {
    use rusqlite::{Connection, Error};

    use crate::{Status, Todo};

    pub fn get_todo_by_id(conn: &Connection, todo_id: i32) -> Result<Todo, Error> {
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
    pub fn add_todo(conn: &Connection, name: &str) -> Result<(), Error> {
        conn.execute("INSERT INTO todo (name) VALUES (:name)", rusqlite::named_params! { ":name": name })
        .map(|rows| {
            println!("Added TODO: {} ({} row(s) affected)", name, rows);
        })
        .map_err(|e| {
            eprintln!("Failed to add TODO: {}", e);
            e
        })
    }
    pub fn update_todo(conn: &Connection, id:&i32, opt_name: Option<&String>, opt_status: Option<&Status>) -> Result<(), Error> {

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
    pub fn delete_todo_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
        let rows_deleted = conn.execute("DELETE FROM todo WHERE id = ?1", [id])?;
        println!("deleted todo with ID {}", id);
    
        if rows_deleted > 0 {
            Ok(())
        } else {
            Err(Error::QueryReturnedNoRows)
        }
    }
    pub fn list_todos(conn: &Connection, status: Option<Status>) -> Result<(), Error> {
        let query = match status {
            Some(Status::Pending) => "SELECT id, name, status, created_at FROM todo WHERE status = 'pending'",
            Some(Status::Complete) => "SELECT id, name, status, created_at FROM todo WHERE status = 'complete'",
            _ => "SELECT id, name, status, created_at FROM todo",
        };
    
        let mut stmt = conn.prepare(query)?;
        let todos = stmt.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_,String>(2)?))
        })?;
    
        for todo in todos {
            let (id, name, status) = todo?;
            println!("{}: {} ({})", id, name, status);
        }
        Ok(())
    }
}
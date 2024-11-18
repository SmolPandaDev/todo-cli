use rusqlite::{Connection, Error, Result};

use std::env;
use std::fmt;

#[derive(Debug)]
enum Status {
    Pending,
    Complete,
}

#[derive(Debug)]
struct Todo {
    id: i32,
    name: String,
    status: Status,
    created_at: String,
}

impl Status {
    fn from_str(status_str: &str) -> Result<Status, String> {
        match status_str {
            "pending" => Ok(Status::Pending),
            "complete" => Ok(Status::Complete),
            _ => Err(format!("Invalid status: {}", status_str)),
        }
    }
}

fn main() -> Result<()> {

    let db_path = "my-todos.sqlite";

    // you often have to annotate the type of collection you want when using collect() as rust can't infer what you want!
    let args: Vec<String> = env::args().collect();
    println!("My TODO app!");

    if args.len() < 2 {
        eprintln!("Usage: todo <command> [args]");
        return Ok(());
    }

    // program name is args[0]
    let command = &args[1];

    println!("Todo command: {command}");

    let conn = Connection::open(db_path)?;

    println!("Connected to SQLite database: {}", db_path);

    create_table_if_not_exists(&conn)?;


    match command.as_str() {
        "get" => {
            // get existing todo by id
            let id = &args[2];
    
            match get_todo_by_id(&conn, id.parse().unwrap()) {
                Ok(todo) => println!("Found TODO: {:?}", todo),
                Err(e) => eprintln!("Error retrieving TODO: {}", e),
            }

        },

        "add" => {
            // add new todo

            let name = &args[2];

            // TODO: error handling
            match conn.execute("INSERT INTO todo (name) VALUES (:name)", rusqlite::named_params! { ":name": name }) {
                Ok(updated) => println!("added todo: {}", updated),
                Err(err) => println!("failed to add todo: {}", err)
            }
        },

        "delete" => {
            // delete todo by id
            let id = &args[2];
            delete_todo_by_id(&conn, id.parse().unwrap())?;
        },

        _ => {
            eprint!("unknown command given")
        }

    }

    Ok(())
}

fn create_table_if_not_exists(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let create_table_sql = "
        CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";

    conn.execute(&create_table_sql, ())
}

fn get_todo_by_id(conn: &Connection, todo_id: i32) -> Result<Todo> {
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

fn delete_todo_by_id(conn: &Connection, id: i32) -> Result<()> {
    let rows_deleted = conn.execute("DELETE FROM todo WHERE id = ?1", [id])?;
    println!("deleted todo with ID {}", id);

    if rows_deleted > 0 {
        Ok(())
    } else {
        Err(Error::QueryReturnedNoRows)
    }
}
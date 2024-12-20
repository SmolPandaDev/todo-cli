use clap::{ Parser, Subcommand};
use rusqlite::{Connection, Result};
use todo_cli_app::todo::{add_todo, delete_todo_by_id, get_todo_by_id, list_todos, update_todo};

mod db;

/// Here's my app!
#[derive(Debug, Parser)]
#[clap(name = "todo-app", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}


#[derive(Debug, Subcommand)]
enum Command {
    /// Add a new todo
    Add {
        /// The name of the todo
        name: String,
    },
    /// Fetch a todo item by its ID
    Get {
        id: i32,
    },

    /// Update a todo 
    Update {
        /// The todo you wish to update
        id: i32,

        /// The name you want to update to
        #[clap(long, short, value_name = "name")]
        name: Option<String>,

        /// status you want to change to
        #[clap(long, short, value_name = "status")]
        status: Option<todo_cli_app::Status>,
    },

    /// Delete a todo by it's ID
    Delete {
        id: i32,
    },

    /// List all todos by status
    List {
        #[clap(value_enum)]
        #[clap(long, short, value_name = "status")]
        status: Option<todo_cli_app::Status>,
    }
}



fn main() -> Result<()> {
    let app = App::parse(); // Parses the CLI arguments automatically

    let db_path = "my-todos.sqlite";
    let conn = Connection::open(db_path)?;

    println!("Connected to SQLite database: {}", db_path);

    db::create_table_if_not_exists(&conn)?;

    match app.command {
        Command::Add { name } => add_todo(&conn, &name)?,
        Command::Get { id } => {
            match get_todo_by_id(&conn, id) {
                Ok(todo) => {
                    println!("Found TODO: {:?}", todo);
                    return Ok(()) // Return `Ok(())` to ensure consistency
                }
                Err(e) => {
                    eprintln!("Error retrieving TODO: {}", e);
                    return Err(e)
                }
            }
        },
        Command::Delete { id } => delete_todo_by_id(&conn, id)?,
        Command::Update { id, name, status } => update_todo(&conn, &id, name.as_ref(), status.as_ref())?,
        Command::List { status } => list_todos(&conn, status)?,
    }

    Ok(())
}

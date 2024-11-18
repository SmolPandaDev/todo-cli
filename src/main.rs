use clap::{ Parser, Subcommand};
use rusqlite::{Connection, Result};

mod add;
mod get;
mod delete;
mod db;
mod list;
mod update;

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
        Command::Add { name } => add::add_todo(&conn, &name)?,
        Command::Get { id } => {
            match get::get_todo_by_id(&conn, id) {
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
        Command::Delete { id } => delete::delete_todo_by_id(&conn, id)?,
        Command::List { status } => list::list_todos(&conn, status)?,
        Command::Update { id, name, status } => update::update_todo(&conn, id, name, status)?,
    }

    Ok(())
}

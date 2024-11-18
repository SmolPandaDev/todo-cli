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
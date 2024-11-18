
#[derive(Debug)]
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
}
use rusqlite::{ Connection, Result };

pub fn add_todo(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("INSERT INTO todo (name) VALUES (:name)", rusqlite::named_params! { ":name": name })
    .map(|rows| {
        println!("Added TODO: {} ({} row(s) affected)", name, rows);
    })
    .map_err(|e| {
        eprintln!("Failed to add TODO: {}", e);
        e
    })
}
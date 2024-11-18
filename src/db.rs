use rusqlite::Connection;

pub fn create_table_if_not_exists(conn: &Connection) -> Result<usize, rusqlite::Error> {
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
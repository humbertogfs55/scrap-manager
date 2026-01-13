use rusqlite::{Connection, Result};
use std::path::Path;

pub mod queries;
pub mod schema;

pub fn connect(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.pragma_update(None, "foreign_keys", &"ON")?;
    Ok(conn)
}

pub fn init(conn: &Connection) -> Result<()> {
    schema::create_all(conn)
}

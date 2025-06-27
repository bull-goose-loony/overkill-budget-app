
use rusqlite::{Connection, Result};
use std::fs;

// initialize the database. load the schema.sql file
pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    let sql = fs::read_to_string("src/sql/schema.sql")
        .expect("Failed to read schema.sql");

    conn.execute_batch(&sql)?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db_creates_schema() {
        // Use in-memory DB so we don't touch disk
        let conn = init_db(":memory:").expect("Failed to init DB");

        // Check that 'financial_record' table exists
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='financial_record'")
            .expect("Failed to prepare table check");

        let mut rows = stmt.query([]).expect("Failed to query sqlite_master");

        assert!(rows.next().unwrap().is_some(), "Table 'financial_record' not found");
    }
}

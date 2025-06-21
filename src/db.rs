
use crate::models::RecordType;
use crate::models::Frequency;

use rusqlite::{params, Connection, Result};

use uuid::Uuid;

// Create the tables and add any static data 
pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // id: UUID
    conn.execute(
        "CREATE TABLE IF NOT EXISTS financial_record (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            amount REAL NOT NULL,
            record_type TEXT NOT NULL,
            frequency TEXT NOT NULL,
        )",
        [],
    )?;
    

    let id = Uuid::new_v4().to_string();
    let name = "Huge Paycheck";
    let amount = 1.23;
    // Sample static data
    conn.execute(
        "INSERT INTO budget (id, name, amount, record_type, frequency) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, name, amount, "Income", "Daily"],
    )?;

    Ok(conn)
}




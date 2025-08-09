use crate::models::FinancialRecord;
use crate::models::Frequency;
use crate::models::RecordType;

use log::debug;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub fn insert_record(conn: &Connection, record: &FinancialRecord) -> Result<()> {
    debug!("insert_record({})", record.to_string());
    conn.execute(
        "INSERT INTO financial_record (id, name, amount, frequency, record_type)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &record.id,
            &record.name,
            &record.amount,
            &record.frequency,
            &record.record_type
        ],
    )?;
    Ok(())
}

pub fn delete_record(conn: &Connection, id: &Uuid) -> Result<()> {
    debug!("delete_record(id={})", id.to_string());
    conn.execute(
        "DELETE FROM financial_record WHERE id = ?1",
        params![id.to_string()],
    )?;
    Ok(())
}

pub fn update_record(conn: &Connection, record: &FinancialRecord) -> Result<()> {
    debug!("update_record({})", record.to_string());
    conn.execute(
        "UPDATE financial_record SET name = ?1, amount = ?2, frequency = ?3, record_type = ?4
         WHERE id = ?5",
        params![
            record.name,
            record.amount,
            record.frequency.to_string(),
            record.record_type.to_string(),
            record.id.to_string()
        ],
    )?;
    Ok(())
}

fn query_table(mut stmt: rusqlite::Statement, param: String) -> Result<Vec<FinancialRecord>> {
    let rows = stmt.query_map([param.as_str()], |row| {
        Ok(FinancialRecord {
            id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
            name: row.get(1)?,
            amount: row.get(2)?,
            frequency: row
                .get::<_, String>(3)?
                .parse::<Frequency>()
                .map_err(|_| rusqlite::Error::InvalidQuery)?,
            record_type: row 
                .get::<_, String>(4)?
                .parse::<RecordType>()
                .map_err(|_| rusqlite::Error::InvalidQuery)?,
        })
    })?;

    let records: Vec<FinancialRecord> = rows.filter_map(Result::ok).collect(); 
    Ok(records)
}

pub fn get_records_by_type(conn: &Connection, record_type: RecordType) -> Result<Vec<FinancialRecord>> {
    debug!("getting records by type={}", record_type.to_string());
    let sql = "
        SELECT id, name, amount, frequency, record_type FROM financial_record 
        WHERE frequency = ?
    ";

    let stmt: rusqlite::Statement = conn.prepare(sql)?;
    let records: Vec<FinancialRecord> = query_table(stmt, record_type.to_string())?;
    Ok(records)
}

pub fn get_records_by_freq(conn: &Connection, freq: Frequency) -> Result<Vec<FinancialRecord>> {
    debug!("getting records by frequency={}", freq.to_string());
    let stmt = format!("
        SELECT id, name, amount, frequency, record_type FROM financial_record
        WHERE frequency = {}", freq.to_string());
    let stmt: rusqlite::Statement = conn.prepare(stmt.as_str())?;
    // Todo
    let records: Vec<FinancialRecord> = query_table(stmt, String::from(""))?;
    Ok(records)
}

pub fn get_records(conn: &Connection) -> Result<Vec<FinancialRecord>> {
    debug!("getting all records");
    let sql = format!("SELECT id, name, amount, frequency, record_type FROM financial_record");
    let records: Vec<FinancialRecord> = conn
        .prepare(sql.as_str())?
        .query_map([], |row| {
            Ok( FinancialRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                amount: row.get(2)?,
                frequency: row.get(3)?,
                record_type: row.get(4)?,
            })
        })?
        // collect up the inner Results, then propagate any error with `?`
        .collect::<Result<Vec<FinancialRecord>, rusqlite::Error>>()?;

    Ok(records)
}

pub fn get_record_by_id(conn: &Connection, id: &Uuid) -> Result<FinancialRecord, rusqlite::Error> {
    debug!("get_record_by_id(id={})", id.to_string());
    conn.query_row(
        "SELECT id, name, amount, frequency, record_type
         FROM financial_record
         WHERE id = ?1",
         params![id],
         | row | {
            Ok( FinancialRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                amount: row.get(2)?,
                frequency: row.get(3)?,
                record_type: row.get(4)?,
            })
         },
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{FinancialRecord, Frequency, RecordType};
    use uuid::Uuid;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("failed to open memory db");
        init_schema(&conn).expect("failed to init schema");
        conn
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS financial_record (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                amount REAL NOT NULL,
                record_type TEXT NOT NULL,
                frequency TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    #[test]
    fn test_insert_record() {
        let conn = setup_conn();
        let record = FinancialRecord {
            id: Uuid::new_v4(),
            name: "Test Income".into(),
            amount: 1000.0,
            frequency: Frequency::Monthly,
            record_type: RecordType::Income,
        };

        insert_record(&conn, &record).expect("Insert failed");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM financial_record", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_delete_record() {
        let conn = setup_conn();
        let record = FinancialRecord {
            id: Uuid::new_v4(),
            name: "To Delete".into(),
            amount: 200.0,
            frequency: Frequency::Weekly,
            record_type: RecordType::Expense,
        };
        insert_record(&conn, &record).unwrap();
        delete_record(&conn, record.id).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM financial_record", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 0);
    }

    #[test]
    fn test_update_record() {
        let conn = setup_conn();
        let mut record = FinancialRecord {
            id: Uuid::new_v4(),
            name: "Old Name".into(),
            amount: 50.0,
            frequency: Frequency::Daily,
            record_type: RecordType::Expense,
        };
        insert_record(&conn, &record).unwrap();

        // Update name and amount
        record.name = "Updated Name".into();
        record.amount = 75.0;
        update_record(&conn, &record).unwrap();

        let (name, amount): (String, f64) = conn
            .query_row(
                "SELECT name, amount FROM financial_record WHERE id = ?1",
                [record.id.to_string()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();

        assert_eq!(name, "Updated Name");
        assert_eq!(amount, 75.0);
    }
}

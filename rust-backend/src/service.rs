use crate::models::{RecordType, Frequency, FinancialRecord};
use crate::types::Db;
use crate::record_repository;

use std::sync::MutexGuard;
use uuid::Uuid;
use log::{info, error};
use rusqlite::{Connection, Result};

pub fn get_all_records(db: &Db) -> Result<Vec<FinancialRecord>> {
    info!("Service get_all_records request");

    let conn = get_connection(&db)?;
    let records = record_repository::get_records(&conn)?;
    Ok(records)
}

pub fn get_all_income(db: &Db) -> Result<Vec<FinancialRecord>> {
    info!("Service get_all_income request");
    let conn = get_connection(&db)?;
    let records = record_repository::get_records_by_type(
        &conn,
        RecordType::Income)?;
    Ok(records)
}

pub fn get_all_expenses(db: &Db) -> Result<Vec<FinancialRecord>> {
    info!("Service get_all_expenses request");
    let conn = get_connection(&db)?;
    let records = record_repository::get_records_by_type(
        &conn,
        RecordType::Income)?;
    Ok(records)
}

pub fn get_record_by_id(db: &Db, id: &Uuid) -> Result<FinancialRecord> {
    info!("Service get_record_by_id(id={}) request", id.to_string());
    let conn = get_connection(&db)?;
    let record = record_repository::get_record_by_id(&conn, id)?;
    Ok(record)
}

pub fn get_connection(db: &Db) -> Result<MutexGuard<Connection>, rusqlite::Error> {
    db.lock().map_err(|e| {
        log::error!("Failed to lock DB mutex: {}", e);
        rusqlite::Error::InvalidQuery // or your own custom error type
    })
}

pub fn add_record(db: &Db, record: &FinancialRecord) {
    let conn = db.lock().unwrap();
    if record.amount <= 0.0 {
        error!("Amount must be positive.");
        return;
    }

    let record = FinancialRecord {
        id: Uuid::new_v4(),
        name: record.name.to_string(),
        amount: record.amount,
        frequency: record.frequency,
        record_type: record.record_type,
    };

    info!("Adding new FinancialRecord {}", record.to_string());

    if let Err(e) = record_repository::insert_record(&conn, &record) {
        error!("Failed to add income for record{}: {}", record.to_string(), e);
    }
}

pub fn add_income(db: &Db, name: &str, amount: f64, freq: Frequency) {
    let conn = db.lock().unwrap();
    if amount <= 0.0 {
        error!("Amount must be positive.");
        return;
    }

    let record = FinancialRecord {
        id: Uuid::new_v4(),
        name: name.to_string(),
        amount,
        frequency: freq,
        record_type: RecordType::Income,
    };

    info!("Adding new income {}", record.to_string());

    if let Err(e) = record_repository::insert_record(&conn, &record) {
        error!("Failed to add income for record{}: {}", record.to_string(), e);
    }
}

pub fn add_expense(db: &Db, name: &str, amount: f64, freq: Frequency) {
    let conn = db.lock().unwrap();
    if amount <= 0.0 {
        error!("Amount must be positive.");
        return;
    }

    let record = FinancialRecord {
        id: Uuid::new_v4(),
        name: name.to_string(),
        amount,
        frequency: freq,
        record_type: RecordType::Expense,
    };

    info!("Adding new expense {}", record.to_string());

    if let Err(e) = record_repository::insert_record(&conn, &record) {
        error!("Failed to add record: {}", e);
    }
}

pub fn delete_record(db: &Db, id: &Uuid) -> Result<()>  {
    info!("Service delete_record(id={})", id.to_string());
    let conn = get_connection(&db)?;
    record_repository::delete_record(&conn, id)?;
    Ok(())
}




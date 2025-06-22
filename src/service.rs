use rusqlite::Connection;
use uuid::Uuid;
use log::{info, warn, error, debug};

use crate::models::{Record, Frequency, RecordType};
use crate::record_repository::insert_record;

pub fn add_income(conn: &Connection, name: &str, amount: f64, freq: Frequency) {
    if amount <= 0.0 {
        error!("Amount must be positive.");
        return;
    }

    let record = Record {
        id: Uuid::new_v4(),
        name: name.to_string(),
        amount,
        frequency: freq,
        record_type: RecordType::Income,
    };

    info!("Adding new income {}", record.to_string());

    if let Err(e) = insert_record(conn, &record) {
        error!("Failed to add income for record{}: {}", record.to_string(), e);
    }
}

pub fn add_expense(conn: &Connection, name: &str, amount: f64, freq: Frequency) {
    if amount <= 0.0 {
        error!("Amount must be positive.");
        return;
    }

    let record = Record {
        id: Uuid::new_v4(),
        name: name.to_string(),
        amount,
        frequency: freq,
        record_type: RecordType::Expense,
    };

    info!("Adding new expense {}", record.to_string());

    if let Err(e) = insert_record(conn, &record) {
        error!("Failed to add record: {}", e);
    }
}

pub fn add_debt(conn: &Connection, name: &str, amount: f64, freq: Frequency) {
    let record = Record {
        id: Uuid::new_v4(),
        name: name.to_string(),
        amount,
        frequency: freq,
        record_type: RecordType::Income,
    };

    info!("Adding financial record {}", record.to_string());

    if let Err(e) = insert_record(conn, &record) {
        error!("Failed to add record: {}", e);
    }
}

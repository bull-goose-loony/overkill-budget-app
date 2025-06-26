mod db;
mod models;
mod service;
mod record_repository;

use log::info;
use uuid::Uuid;
use std::env;

use axum::{
    extract::{Form, State},
    response::{Html,IntoResponse},
    routing::{get, post},
    Router,
};

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use rusqlite::Connection;

fn main() {
    env_logger::init();

    let db_path = "budget.db";
    let cwd = env::current_dir().unwrap();
    println!("Opening DB at: {}", cwd.join(db_path).display());
    
    let conn = db::init_db(db_path).expect("failed to init db");

    let record = models::Record {
            id: Uuid::new_v4(),
            name: "Test Income".into(),
            amount: 1000.0,
            frequency: models::Frequency::Monthly,
            record_type: models::RecordType::Income,
        };

    let insert_result = record_repository::insert_record(&conn, &record);

}

#[derive(Default, Clone)]
struct AppState {
    expenses: Arc<Mutex<Vec<models::Record>>>,
}


use log::{info, error};
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use axum::{
    extract::{Form, State, Path},
    response::Html,
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use rusqlite::Connection;
use serde::Deserialize;
use crate::types::Db;
use crate::{service};
use crate::{models::{Record, Frequency, RecordType}, record_repository};

#[derive(Clone)]
pub struct RecordState {
    pub database: Arc<Mutex<Connection>>,
}

#[derive(Deserialize)]
pub struct NewRecord {
    pub name: String,
    pub amount: f64,
    pub frequency: String,
    pub record_type: String
}

pub fn routes(db: Db) -> Router {
    let state = RecordState {
        database: db,
    };

    Router::new()
        .route("/", get(get_all))
        .route("/add", post(add_record))
        .route("/:id", get(get_record))
        .with_state(state)
}

#[debug_handler]
pub async fn get_all(State(state): State<RecordState>) -> Html<String> {
    let records = match service::get_all_records(&state.database) {
        Ok(data) => data,
        Err(e) => vec![]};

    let html = records
        .iter()
        .map(|r| format!("<li>{} - ${:.2}</li>", r.name, r.amount))
        .collect::<Vec<_>>()
        .join("\n");

    Html(format!("<ul>{}</ul>", html))
}

async fn add_record(State(state): State<RecordState>, Form(form): Form<NewRecord>) -> Html<String> {
    info!("Serving add_record request");
    let record = Record::new(
        form.name,
        form.amount, 
        Frequency::from_str(form.frequency.as_str()).expect("AHHH"),
        RecordType::from_str(form.record_type.as_str()).expect("Doh!"));

    let conn = state.database.lock().unwrap();

    let record = record_repository::insert_record(&conn, &record).unwrap();

    Html(format!("<ul>{}</ul>", "shit"))
}

async fn get_record(Path(id): Path<String>) -> String {
    info!("Serving get_record(id={}) request", id);

    format!("Requested expense with ID: {}", id)
}

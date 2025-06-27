use std::sync::{Arc, Mutex};
use std::str::FromStr;
use axum::{
    extract::{Form, State, Path, Json},
    response::Html,
    routing::{get, post},
    Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use crate::types::Db;
use crate::service;
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
    pub record_type; String
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

async fn get_all(State(state): State<RecordState>) -> Html<String> {
    let records = service::get_all_records(&state.database);

    let html = records
        .iter()
        .map(|r| format!("<li>{} - ${:.2}</li>", r.name, r.amount))
        .collect::<Vec<_>>()
        .join("\n");

    Html(format!("<ul>{}</ul>", html))
}

async fn add_record(State(state): State<RecordState>, Form(form): Form<NewRecord>) -> Html<String> {
    let record = Record::new(form.name, form.amount, Frequency::from_str(form.frequency.as_str()).expect("AHHH"), RecordType::FromStr(form.record_type.as_str()));
    let conn = state.database.lock().unwrap();

    record_repository::insert_record(&conn, &record).unwrap();

    get_all(State(state)).await
}

async fn get_record(Path(id): Path<String>) -> String {
    format!("Requested expense with ID: {}", id)
}

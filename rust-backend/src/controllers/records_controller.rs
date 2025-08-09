use crate::{models::{FinancialRecord,Frequency, RecordType}, record_repository, service};

use uuid::{uuid, Uuid};
use log::{info, debug, error};
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use axum::{
    extract::{Form, State, Path},
    response::Html,
    routing::{get, post},
    Router};
use axum_macros::debug_handler;
use rusqlite::Connection;
use serde::Deserialize;
use crate::types::Db;

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
        .route("/all", get(get_all))
        .route("/income", get(get_all_income))
        .route("/expenses", get(get_all_expenses))
        .route("/add", post(add_record))
        .route("/delete/:id", post(delete_record))
        .route("/:id", get(get_record_by_id))
        .with_state(state)
}

#[debug_handler]
pub async fn get_all(State(state): State<RecordState>) -> Html<String> {
    info!("GET /records/ request");

    let records = match service::get_all_records(&state.database) {
        Ok(data) => {
            info!("Retrieved {} income records from DB", data.len());
            data
        },
        Err(e) => {
            error!("Failed to fetch records: {:?}", e);
            return Html("<p>Error retrieving records</p>".to_string());
        }
    };

    let html = records
        .iter()
        .map(|r| format!(
            "<li>{} - {} - ${:.2} [{} / {}]</li>",
            r.id, r.name, r.amount, r.frequency.to_string(), r.record_type.to_string()
        ))
        .collect::<Vec<_>>()
        .join("\n");

    log::info!("{}", html);
    Html(format!("<ul>{}</ul>", html))
}

#[debug_handler]
pub async fn get_record_by_id(
    Path(id): Path<Uuid>,
    State(state): State<RecordState>,
) -> Html<String> {

    info!("GET /records/{} request", id);
    match service::get_record_by_id(&state.database, &id) {
        Ok(r) => {
            // render a single record
            let html = format!(
                "<h1>Record {}</h1>\
                 <ul>\
                   <li>Name: {}</li>\
                   <li>Amount: ${:.2}</li>\
                   <li>Frequency: {}</li>\
                   <li>Type: {}</li>\
                 </ul>",
                r.id, r.name, r.amount, r.frequency, r.record_type,
            );
            Html(html)
        }

        Err(e) => {
            error!("Failed to fetch record `{}`: {:?}", id, e);
            Html(format!("<p>Error retrieving record `{}`</p>", id))
        }
    }
}

#[debug_handler]
pub async fn get_all_income(State(state): State<RecordState>) -> Html<String> {
    info!("GET /records/income request");

    let records = match service::get_all_income(&state.database) {
        Ok(data) => {
            info!("Retrieved {} income records from DB", data.len());
            data
        },
        Err(e) => {
            error!("Failed to fetch records: {:?}", e);
            return Html("<p>Error retrieving records</p>".to_string());
        }
    };

    let html = records
        .iter()
        .map(|r| format!(
            "<li>{} - ${:.2} [{} / {}]</li>",
            r.name, r.amount, r.frequency.to_string(), r.record_type.to_string()
        ))
        .collect::<Vec<_>>()
        .join("\n");

    log::info!("{}", html);
    Html(format!("<ul>{}</ul>", html))
}

#[debug_handler]
pub async fn get_all_expenses(State(state): State<RecordState>) -> Html<String> {
    info!("GET /records/expenses request");

    let records = match service::get_all_expenses(&state.database) {
        Ok(data) => {
            info!("Retrieved {} income records from DB", data.len());
            data
        },
        Err(e) => {
            error!("Failed to fetch records: {:?}", e);
            return Html("<p>Error retrieving records</p>".to_string());
        }
    };

    let html = records
        .iter()
        .map(|r| format!(
            "<li>{} - ${:.2} [{} / {}]</li>",
            r.name, r.amount, r.frequency.to_string(), r.record_type.to_string()
        ))
        .collect::<Vec<_>>()
        .join("\n");

    log::info!("{}", html);
    Html(format!("<ul>{}</ul>", html))
}

async fn add_record(State(state): State<RecordState>, Form(form): Form<NewRecord>) -> Html<String> {
    info!("Serving add_record request");
    let record = FinancialRecord::new(
        form.name,
        form.amount, 
        Frequency::from_str(form.frequency.as_str()).expect("AHHH"),
        RecordType::from_str(form.record_type.as_str()).expect("Doh!"));
    debug!("Adding {}", form.record_type.as_str());

    service::add_record(&state.database, &record);
    Html("<p>Successfully Added Record</p>".to_string())
}

async fn delete_record(Path(id): Path<Uuid>, State(state): State<RecordState>) -> Html<String> {
    info!("Serving delete_record request");

    service::delete_record(&state.database, &id);
    Html("<p>Successfully Deleted Record</p>".to_string())
}


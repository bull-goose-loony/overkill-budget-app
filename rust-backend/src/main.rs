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

/#[tokio::main]
async fn main() {
    env_logger::init();
    let conn = db::init_db("budget.db").expect("DB failed");

    let app = Router::new()
        .nest("/api", api::routes(conn.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Default, Clone)]
struct AppState {
    expenses: Arc<Mutex<Vec<models::Record>>>,
}


mod db;
mod models;
mod types;
mod service;
mod record_repository;
mod controllers;

use log::info;
use uuid::Uuid;
use std::env;
use hyper::Server;

use axum::{
    extract::{Form, State},
    response::{,Html,IntoResponse},
    routing::{get, post},
    Router,
};

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::types::Db;

#[tokio::main]
async fn main() {
    env_logger::init();
    let conn = db::init_db("budget.db").expect("DB failed");

    // Wrap connection in atomic reference counter and a mutex so we can share it 
    // our endpoint modules
    let shared_conn = Arc::new(Mutex::new(conn));

    // pass SQLite connection into router
    let app = Router::new()
        .nest("/api", controllers::routes(shared_conn.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub database: &Db,
}


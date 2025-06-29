mod db;
mod models;
mod types;
mod service;
mod record_repository;
mod controllers;
mod AppError;

use rusqlite::{Connection};

use axum::{Router};

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

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

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Mutex<Connection>>,
}


pub mod records;

use std::sync::{Arc, Mutex};
use axum::Router;
use rusqlite::Connection;

// Top level Router. add a route for each file you add to the controllers dir
pub fn routes(conn: Arc<Mutex<Connection>>) -> Router {
    Router::new().nest("/records", records::routes(conn))
}


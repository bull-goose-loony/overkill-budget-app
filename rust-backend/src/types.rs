use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub type Db = Arc<Mutex<Connection>>;

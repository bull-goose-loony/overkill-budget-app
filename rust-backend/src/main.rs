mod db;
mod models;
mod service;
mod record_repository;

use clap::{Parser, Subcommand};
use log::info;
use rusqlite::Connection;

fn main() {
    env_logger::init();
    info!("Hello, world!");
}

#[derive(Subcommand)]
enum Commands {
    
}


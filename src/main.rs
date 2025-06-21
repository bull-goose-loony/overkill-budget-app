mod db;
mod models;
mod repository;
mod service;

use clap::{Parser, Subcommand};
use rusqlite::Connection;

fn main() {
    println!("Hello, world!");
}

#[derive(Subcommand)]
enum Commands {
    
}


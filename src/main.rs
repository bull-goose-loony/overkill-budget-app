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
    let cli = Cli::parse();
    let conn = init_db("budget.db").expect("DB failed");

    match cli.command {
        Commands::Add { name, amount, frequency } => {
            service::add_income(&conn, name, amount, frequency);
        }
        Commands::Delete { id } => {
            service::delete_by_id(&conn, id);
        }
        Commands::Summary => {
            let balance = service::calculate_balance(&conn);
            println!("Total balance: ${:.2}", balance);
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    
}


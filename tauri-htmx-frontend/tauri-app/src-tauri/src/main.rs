// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use tiny_http::{Server, Response};

fn main() {
    // Start tiny HTTP server in a background thread
    thread::spawn(|| {
        let server = Server::http("127.0.0.1:8000").unwrap();
        for request in server.incoming_requests() {
            if request.url() == "/hello" {
                let response = Response::from_string("<div id='output'>Hello from Rust!</div>");
                let _ = request.respond(response);
            } else {
                let response = Response::from_string("404 Not Found").with_status_code(404);
                let _ = request.respond(response);
            }
        }
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

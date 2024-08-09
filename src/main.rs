#[allow(dead_code)]
mod model_request;
mod method;
mod utils;
mod server;
mod error;
mod model_query_string;
mod model_query_result;
mod model_response;
mod model_status_code;
mod server_handler;
mod server_thread;
mod model_headers;

use std::io::Read;
use std::sync::{Arc, Mutex};
use crate::server::Server;
use crate::server_handler::ServerHandler;

fn main() {

    // SUGGEST: The root directory where the Cargo.toml file is present
    let root_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    let _public_path = std::env::var("PUBLIC_PATH").unwrap_or(root_path);
    println!("Public Files path: {}", _public_path);

    let server = Server::new(String::from("127.0.0.1:7021"));
    let handler = Arc::new(Mutex::new(ServerHandler::new(_public_path)));
    server.start(handler);

    // println!("Please enter the weight in KGs:");
    // let mut weight = String::new();
    // std::io::stdin().read_line(&mut weight).unwrap();
    // println!();
    //
    // let weight: f32 = weight.trim().parse().expect("A valid number not provided");
    // let weight = calculate_weight_on_mars(weight);
    //
    // println!("The weight on mars in KGs: {}",weight);
}
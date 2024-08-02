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

use std::io::Read;
use crate::server::Server;
fn main() {
    let server = Server::new(String::from("127.0.0.1:7021"));
    server.start();

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
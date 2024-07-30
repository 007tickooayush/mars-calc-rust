use std::io::Error;
use std::net::TcpListener;

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

struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn start(&self){
        println!("Server started at: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // using the Rust's infinite "loop" for listening to the incoming connection requests
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("New client: {}", addr);
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,

}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
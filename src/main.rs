fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
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
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn start(&self) {
        println!("Server started at: {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
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
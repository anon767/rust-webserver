#[macro_use]
extern crate serde_derive;

use file_handler::file_handler::read_file;

extern crate tiny_http;
extern crate serde_json;

mod threadpool;
mod file_handler;
mod server;
mod config;

use threadpool::*;
use tiny_http::{Server, Response, Request};
use config::*;
use serde_json::error::Error;


fn parse_config(path: &String) -> ConfigDocument {
    let content: String = read_file(path).unwrap();
    let config: Result<(ConfigDocument), Error> = config::parse_config(&content);
    config.unwrap()
}

fn handle_connection(request: Request) {
    println!("received request! method: {:?}, url: {:?}, headers: {:?}",
             request.method(),
             request.url(),
             request.headers()
    );
    let response = Response::from_string("hello world");
    request.respond(response);
}

fn main() {

    // let filename = "/home/tom/test.txt";
    //println!("with text:{}", read_file(filename).unwrap());
    let config_path: String = "/home/tom/config.json".to_string();
    let config: ConfigDocument = parse_config(&config_path);
    let pool = ThreadPool::new(config.threads);

    let server = Server::http(format!("{}:{}",config.ip,config.port)).unwrap();


    for request in server.incoming_requests() {
        pool.execute(|| {
            handle_connection(request);
        });
    }
}

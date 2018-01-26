#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tiny_http;

mod server;
mod config;
mod file_handler;
mod threadpool;

use threadpool::*;
use config::*;
use serde_json::error::Error;
use server::*;
use self::tiny_http::{Response, Request};



fn main() {

<<<<<<< HEAD
    let config_path: String = "/home/tg/config.json".to_string();
    let config: ConfigDocument = config::parse_file(&config_path);
=======
    let config_path: String = "/home/tom/config.json".to_string();
    let config: ConfigDocument = parse_config(&config_path);
>>>>>>> ed440a466f25a4d6ac86f566bac90ad19d5eca71
    let pool = ThreadPool::new(config.threads);
    let webServer = server::WebServer;
    webServer.run(pool,config);

<<<<<<< HEAD
=======
    let server = Server::http(format!("{}:{}",config.ip,config.port)).unwrap();


    for request in server.incoming_requests() {
        pool.execute(|| {
            handle_connection(request);
        });
    }
>>>>>>> ed440a466f25a4d6ac86f566bac90ad19d5eca71
}

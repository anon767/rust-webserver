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

    let config_path: String = "/home/tom/config.json".to_string();
    let config: ConfigDocument = parse_config(&config_path);
    let pool = ThreadPool::new(config.threads);
    let webServer = server::WebServer;
    webServer.run(pool,config);

}

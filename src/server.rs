
use threadpool::*;
use config::*;
use tiny_http::{Server, Response, Request, Header,StatusCode};
use std::path::Path;
use file_handler::*;
use std::io::{Cursor};


fn handle_connection(request: Request, document_root: String) {
    let path = format!("{}{}",document_root,request.url());
    println!("serving url: {:?}",path);
    if Path::new(&path).exists() {
	let contents = read_file(&path).unwrap();
        let data_len = contents.len();
        let response = Response::new(
            StatusCode(200),
            vec![
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=UTF-8"[..]).unwrap()
            ],
            Cursor::new(contents.as_bytes()),
            Some(data_len),
            None
        );
    request.respond(response);
    }
}



pub struct WebServer;
impl WebServer {
pub fn run(&self, pool: ThreadPool, config:ConfigDocument) {
    let server = Server::http(format!("{}:{}",config.ip,config.port)).unwrap();
	
    for request in server.incoming_requests() {
	let document_root = config.document_root.clone();
        pool.execute(|| {
            handle_connection(request,document_root);
        });
    }
}
}
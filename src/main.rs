#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    // default environment variables
    let default_path: String = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let default_addr: String = String::from("127.0.0.1:8080");

    // environment variables
    let public_path: String = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let addr: String = env::var("ADDR").unwrap_or(default_addr);
    
    // init server and handler
    let server: Server = Server::new(addr);
    let handler: WebsiteHandler = WebsiteHandler::new(public_path);

    // run HTTP server
    server.run(handler);
}

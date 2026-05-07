use crate::config::load_config;
use std::net::TcpListener;
mod config;
mod protocol;
mod server;
mod state;
mod utils;

pub const CONFIG_PATH: &str = "config.yaml";
fn main() {
    let config = load_config(CONFIG_PATH);

    let addr = format!("{}:{}", config.ip, config.port);

    let listener = TcpListener::bind(&addr).expect("Failed to bind addr");
    println!(
        "Starting service {}\nOn {}\nListening...",
        config.name, addr
    );
    server::accept_connections(listener, config);
}

use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{config::Config, protocol::protocol_checker, state::User};

pub fn handle_client(mut stream: TcpStream, config: Config) {
    let peer_addr = stream
        .peer_addr()
        .map_or_else(|_| "unknown".to_string(), |addr| addr.to_string());

    let mut user = User {
        is_logged_in: false,
    };

    println!("Connecting from: {}", peer_addr);

    let mut buffer = [0; 1024];

    loop {
        let n = match read_from_stream(&mut stream, &mut buffer) {
            Ok(0) => {
                println!("Client {} closed connection", peer_addr);
                break;
            }
            Ok(n) => n,
            Err(e) => {
                handle_read_error(&peer_addr, &e);
                break;
            }
        };
        let data = &buffer[0..n];

        let ans = protocol_checker(data, &mut user, config.clone());

        if let Err(e) = write_response(&mut stream, ans.as_bytes()) {
            eprintln!("Write error to client {}: {}", peer_addr, e);
            break;
        }
    }

    println!("Connection finished for: {}", peer_addr);
}

fn read_from_stream(stream: &mut TcpStream, buffer: &mut [u8]) -> io::Result<usize> {
    loop {
        match stream.read(buffer) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            res => return res,
        }
    }
}

fn write_response(stream: &mut TcpStream, data: &[u8]) -> io::Result<()> {
    stream.write_all(data)
}

fn handle_read_error(peer_addr: &str, e: &io::Error) {
    match e.kind() {
        io::ErrorKind::ConnectionReset => {
            println!("Client {} reset connection", peer_addr);
        }
        _ => {
            eprintln!("Read error from client {}: {}", peer_addr, e);
        }
    }
}

pub fn accept_connections(listener: TcpListener, config: Config) {
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                let config = config.clone();
                thread::spawn(move || {
                    handle_client(stream, config);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e)
            }
        }
    }
}

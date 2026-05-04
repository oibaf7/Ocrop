use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde_json::Value;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| handle_connection(s));
            },
            Err(e) => {
                println!("Error while receiving stream. Error: {e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = BufReader::new(&stream);
    loop {
        let mut bytes = Vec::new();
        match buf.read_until("\n".as_bytes()[0], &mut bytes) {
            Ok(0) => {
                println!("Connection closed!");
                break;
            },
            Ok(_) => {
                match serde_json::from_slice::<Value>(&bytes[..]) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("Error {e}")
                }
            },
            Err(e) => println!("Error: {e}")
        }
    }
}
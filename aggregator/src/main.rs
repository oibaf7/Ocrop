use std::net::{TcpListener, TcpStream};
use std::thread;

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
    //send to frontend
}
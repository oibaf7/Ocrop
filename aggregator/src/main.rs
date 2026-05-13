mod app;
mod event;
mod ui;

use crate::app::App;
use crate::event::{SystemEvent, check_for_key_events};
use serde_json::Value;
use shared::Processes;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{io, thread};

fn main() -> Result<(), io::Error> {
    let (tx, rx) = mpsc::channel::<SystemEvent>();
    check_for_key_events(tx.clone());
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind TCP listener");
        for stream in listener.incoming() {
            let tx = tx.clone();
            match stream {
                Ok(s) => {
                    thread::spawn(move || handle_connection(tx, s));
                }
                Err(e) => {
                    println!("Error while receiving stream. Error: {e}");
                }
            }
        }
    });
    ratatui::run(|terminal| App::new(rx).run(terminal))?;

    Ok(())
}

fn handle_connection(tx: Sender<SystemEvent>, stream: TcpStream) {
    let mut buf = BufReader::new(&stream);
    loop {
        let mut bytes = Vec::new();
        match buf.read_until("\n".as_bytes()[0], &mut bytes) {
            Ok(0) => {
                println!("Connection closed!");
                break;
            }
            Ok(_) => match serde_json::from_slice::<Processes>(&bytes[..]) {
                Ok(v) => {
                    //println!("{:#?}", v);
                    tx.send(SystemEvent::ProcessEvent(v));
                }
                Err(e) => println!("Error {e}"),
            },
            Err(e) => println!("Error: {e}"),
        }
    }
}

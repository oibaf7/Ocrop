mod proc;

use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crate::proc::{collect_processes_data};
//add config file eventually
fn main() {
    loop {
        match TcpStream::connect("127.0.0.1:7878") {
            Ok(mut stream) => {
                if let Err(e) = run(&mut stream) {
                    println!("Error while serializing/sending data. Error: {e}");
                }
            },
            Err(e) => {
                println!("Error while trying to connect! Retrying. Error: {e}");
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn run(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let processes = collect_processes_data()?;
        let json = serde_json::to_string(&processes)?;
        stream.write_all(json.as_bytes())?;
        thread::sleep(Duration::from_secs(5));
    }
}

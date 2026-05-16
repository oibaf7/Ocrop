mod proc;

use crate::proc::collect_processes_data;
use config::{Config, File};
use serde::Deserialize;
use shared::SnapShotCollector;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::num::NonZero;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Settings {
    address: String,
    delay: u64,
}

fn main() {
    let settings = Config::builder()
        .add_source(File::with_name("Config.toml"))
        .build()
        .expect("File in the wrong format!")
        .try_deserialize::<Settings>()
        .expect("Could not deserialize! Check configuration file!");

    println!("{:#?}", settings);
    let cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as u64;
    loop {
        match TcpStream::connect(&settings.address) {
            Ok(mut stream) => {
                if let Err(e) = run(&mut stream, &settings, cores) {
                    println!("Error while serializing/sending data. Error: {e}");
                }
            }
            Err(e) => {
                println!("Error while trying to connect! Retrying. Error: {e}");
            }
        }
        thread::sleep(Duration::from_secs(settings.delay));
    }
}

fn run(stream: &mut TcpStream, settings: &Settings, cores: u64) -> Result<(), Box<dyn Error>> {
    let mut collector = SnapShotCollector::new();
    loop {
        let processes = collect_processes_data(&mut collector, cores)?;
        let json = serde_json::to_string(&processes)? + "\n";
        //println!("{}", json);
        stream.write_all(json.as_bytes())?;
        println!("Data has been sent!");
        thread::sleep(Duration::from_secs(settings.delay));
    }
}

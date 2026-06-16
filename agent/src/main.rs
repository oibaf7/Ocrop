mod proc;

use crate::proc::collect_processes_data;
use config::{Config, File};
use pulse::sender::{Action, Sender};
use serde::Deserialize;
use shared::SnapShotCollector;
use std::error::Error;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Settings {
    address: String,
    addr_to: String,
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
        if let Err(e) = run(&settings, cores) {
            println!("error while running! Error: {e}")
        }
    }
}

fn run(settings: &Settings, cores: u64) -> Result<(), Box<dyn Error>> {
    let mut collector = SnapShotCollector::new();
    let mut sender = Sender::new(settings.address.parse().expect("Invalid Address"));
    loop {
        let action = sender.tick();
        match action {
            Action::Retransmit | Action::Send => {
                let processes = collect_processes_data(&mut collector, cores);
                if let Ok(p) = processes {
                    println!("{}", p.processes.len());
                    let size =
                        sender.send(p, settings.addr_to.parse().expect("Invalid Address"))?;
                    println!("Data has been sent! Size: {size}");
                    continue;
                }
                println!("Error while reading data!");
            }
            _ => (),
        }

        thread::sleep(Duration::from_millis(50));
    }
}

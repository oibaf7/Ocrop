mod app;
mod event;

use crate::app::App;
use crate::event::{SystemEvent, check_for_key_events};
use config::{Config, File};
use serde::Deserialize;
use shared::Processes;
use std::io::{stdout, BufRead, BufReader, Read};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{io, thread};
use std::time::Duration;

#[derive(Deserialize)]
struct Settings {
    timeout: u64,
    address: String,
}

fn main() -> Result<(), io::Error> {
    let settings = Config::builder()
        .add_source(File::with_name("Config.toml"))
        .build()
        .expect("File in the wrong format!")
        .try_deserialize::<Settings>()
        .expect("Could not deserialize! Check configuration file!");
    let (tx, rx) = mpsc::channel::<SystemEvent>();
    check_for_key_events(tx.clone());
    thread::spawn(move || {
        //maybe remove from thread later
        loop {
            let receiver = pulse::receiver::Receiver::new(settings.address.parse().expect("Invalid Address"));
            handle_connection(tx.clone(), receiver);
        }
    });
    ratatui::run(|terminal| App::new(rx, settings.timeout).run(terminal))?;

    Ok(())
}

fn handle_connection(tx: Sender<SystemEvent>, r: pulse::receiver::Receiver) {
    let mut r = r;
    loop {
        let actions = r.tick();
        //println!("{:#?}", actions);
        for action in actions {
            match action {
                pulse::receiver::Action::Received(p) => {
                    if tx.send(SystemEvent::ProcessEvent(p)).is_err() {
                        break;
                    }
                },
                pulse::receiver::Action::RequestRetransmit(addr) => {
                    //check how to handle error!
                    r.send_retransmit(addr);
                }

            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

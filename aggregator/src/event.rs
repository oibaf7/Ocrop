use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use shared::Processes;
use std::sync::mpsc::Sender;
use std::thread;

pub enum SystemEvent {
    KeyEvent(Event),
    ProcessEvent(Processes),
}

pub fn check_for_key_events(tx: Sender<SystemEvent>) {
    thread::spawn(move || {
        loop {
            if let Ok(e) = event::read() {
                //handle error later, maybe end loop since means rx was dropped
                tx.send(SystemEvent::KeyEvent(e));
            }
        }
    });
}

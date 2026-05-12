use crate::event::SystemEvent;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use shared::{Process, Processes, process};
use std::sync::mpsc::Receiver;
use std::{io, thread};

pub struct App {
    rx: Receiver<SystemEvent>,
    processes: Processes,
    connection: u64,
    exit: bool,
}

impl App {
    pub fn new(receiver: Receiver<SystemEvent>) -> Self {
        Self {
            rx: receiver,
            processes: Processes::default(),
            connection: 0,
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.rx.recv() {
                Ok(v) => self.handle_event(v),
                Err(_) => (),
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_event(&mut self, event: SystemEvent) {
        match event {
            SystemEvent::KeyEvent(Event::Key(key_event))
                if key_event.kind == KeyEventKind::Press =>
            {
                self.handle_key_event(key_event)
            }
            SystemEvent::ProcessEvent(v) => {
                self.processes = v;
            }
            _ => (),
        };
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Metrics Collector ".bold());
        let instructions = Line::from(vec!["<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Paragraph::new(format!("{:#?}", self.processes.total_pss))
            .centered()
            .block(block)
            .render(area, buf);
    }
}

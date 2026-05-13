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
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Row, Table};

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
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

        let content = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(layout[1]);

        self.render_summary(layout[0], buf);
        self.render_processes(content[0], buf);
        self.render_stats(content[1], buf);
        self.render_instructions(layout[2], buf);
    }
}

impl App {
    fn render_summary(&self, area: Rect, buf: &mut Buffer) {
        let text = format!(
            " CPU: {:.1}%   MEM: {} KB   Threads: {}   Processes: {} ",
            self.processes.total_instant_cpu,
            self.processes.total_pss,
            self.processes.total_threads,
            self.processes.processes.len()
        );
        let block = Block::bordered()
            .title(" Ocrop ".bold())
            .border_set(border::THICK);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_processes(&self, area: Rect, buf: &mut Buffer) {
        let header = Row::new(vec!["NAME", "PID", "CPU%", "MEM (KB)", "THREADS"])
            .style(Style::new().bold().fg(Color::Yellow))
            .bottom_margin(1);

        let mut procs = self.processes.processes.clone();
        procs.sort_by(|a, b| {
            b.instant_cpu_usage
                .partial_cmp(&a.instant_cpu_usage)
                .unwrap()
        });

        let rows: Vec<Row> = procs
            .iter()
            .map(|p| {
                Row::new(vec![
                    p.name.clone(),
                    p.pid.to_string(),
                    format!("{:.2}", p.instant_cpu_usage),
                    p.pss.to_string(),
                    p.threads.to_string(),
                ])
            })
            .collect();

        let widths = [
            Constraint::Percentage(30),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
        ];

        let block = Block::bordered()
            .title(" Processes ".bold())
            .border_set(border::THICK);

        Table::new(rows, widths)
            .header(header)
            .block(block)
            .render(area, buf);
    }

    fn render_stats(&self, area: Rect, buf: &mut Buffer) {
        let p = &self.processes;
        let text = vec![
            Line::from(vec![
                "Total CPU:  ".into(),
                format!("{:.1}%", p.total_instant_cpu).yellow(),
            ]),
            Line::from(vec![
                "Avg CPU:    ".into(),
                format!("{:.1}%", p.total_avg_cpu).yellow(),
            ]),
            Line::from(vec![
                "Total PSS:  ".into(),
                format!("{} KB", p.total_pss).yellow(),
            ]),
            Line::from(vec![
                "Threads:    ".into(),
                p.total_threads.to_string().yellow(),
            ]),
            Line::from(vec![
                "Processes:  ".into(),
                p.processes.len().to_string().yellow(),
            ]),
        ];

        let block = Block::bordered()
            .title(" System ".bold())
            .border_set(border::THICK);

        Paragraph::new(text).block(block).render(area, buf);
    }

    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        Line::from(vec![" Q ".blue().bold(), "Quit".into()])
            .centered()
            .render(area, buf);
    }
}

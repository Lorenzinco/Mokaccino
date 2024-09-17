use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout};

pub struct App {
    running: bool,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl App {
    pub fn new() -> Result<App, io::Error> {
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let app = App {
            running: false,
            terminal: Terminal::new(backend)?,
        };
        Ok(app)
    }

    pub fn init(&mut self) {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen).unwrap();
    }

    pub fn exit(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.terminal.draw(ui).unwrap();
            self.running = handle_events().unwrap();
        }
    }
}

fn ui(frame: &mut Frame) {
    let areas = Layout::vertical([Constraint::Length(1); 2]).split(frame.area());
    let title = Paragraph::new("Mokaccino").magenta().on_white();
    frame.render_widget(title, areas[0]);
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Mokaccino")),
        areas[1],
    );
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

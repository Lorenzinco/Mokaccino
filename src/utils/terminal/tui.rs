use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

enum UiMode {
    Tab,

}

struct Tabs {
    pub titles: Vec<String>,
    pub index: usize,
    pub inside: bool
}

impl Tabs {
    fn new() -> Tabs {
        Tabs {
            titles: vec!["Home"],
            index: 0,
            inside: true
        }
    }

    fn add(&mut self, name: &str) {
        self.titles.push(name.to_owned());
    }

    fn remove(&mut self, name: &str) {
        match self.titles.iter().position(|&e| e == name) {
            Some(index) => {
                self.titles.remove(index);
            },
            None => {}
        }
        
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn inside(&mut self, status: bool) {
        self.inside = status;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create UI and render it
    let tabs = Tabs::new();
    let res = render(&mut terminal, tabs);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn render<B: Backend>(terminal: &mut Terminal<B>, mut tabs: Tabs) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.inside {
                // tab switching
                false => {
                    match key.code {
                        KeyCode::Esc => return Ok(()), // exit logic (maybe draw confirmation dialog?)
                        KeyCode::Enter => app.inside(true),
                        // KeyCode::Delete => disconnetti da peer
                        KeyCode::Right => app.next(),
                        KeyCode::Left => app.previous(),
                        _ => {}
                    }
                },
                // inside tab
                true => {
                    match tabs.index {
                        // home page
                        0 => {
                            match key.code {

                            }
                        },
                        // peer tab
                        1 => {
                            match key.code {

                            }
                        }
                    }
                }
            }
            
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);
    let inner = match app.index {
        0 => Block::default().title("Inner 0").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        3 => Block::default().title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}
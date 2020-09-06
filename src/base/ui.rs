use std::borrow::Cow;
use std::error::Error;
use std::io;
use std::io::Stdout;

use termion::{
    event::Key,
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use termion::input::TermRead;
use termion::raw::RawTerminal;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;
use tui::widgets::{Block, Borders, BorderType, List, Paragraph, Text};

use crate::app::App;
use crate::utils::event::{Event, Events};

pub fn terminal_init() -> Result<Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

pub struct UI {
    terminal: Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>,
    events: Events,
}

pub enum Actions {
    QUIT,
    NONE,
}

impl UI {
    pub fn new() -> UI {
        UI {
            terminal: terminal_init().unwrap(),
            events: Events::new(),
        }
    }

    pub fn render(&mut self, app: &mut App) -> io::Result<()> {
        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            let text = vec![Text::Raw(Cow::from(app.current_name.clone()))];
            let block = Paragraph::new(text.iter())
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(" Current name ")
                    .title_style(Style::default().fg(Color::Green))
                    .border_type(BorderType::Rounded)
                )
                .style(Style::default().modifier(Modifier::BOLD).fg(Color::Red))
                .alignment(Alignment::Center)
                .wrap(true);
            f.render_widget(block, chunks[0]);

            let items = app.items.items.iter().map(|i| Text::raw(Cow::from(i)));
            let events = List::new(items)
                .block(Block::default()
                    .title("Next names")
                    .title_style(Style::default().fg(Color::DarkGray))
                    .border_style(Style::default().fg(Color::DarkGray))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
            f.render_stateful_widget(events, chunks[1], &mut app.items.state);

            let text = vec![
                Text::Raw(Cow::from("  (n)ext name")),
                Text::Raw(Cow::from("  (s)elect name")),
                Text::Raw(Cow::from("  (r)estart")),
                Text::Raw(Cow::from("  (d)elete")),
                Text::Raw(Cow::from("  (q)uit"))
            ];
            let block = Paragraph::new(text.iter())
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black)))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray))
                .wrap(true);
            f.render_widget(block, chunks[2]);
        })
    }

    pub fn check_events(&mut self, app: &mut App) -> Actions {
        let event = self.events.next().unwrap();
        match event {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    return Actions::QUIT;
                }
                Key::Char('n') => {
                    app.next_name();
                }
                Key::Char('s') => {
                    app.select_current_name();
                }
                Key::Char('r') => {
                    app.restart();
                }
                Key::Left => {
                    app.unselect();
                }
                Key::Down => {
                    app.select_next_name();
                }
                Key::Up => {
                    app.select_previous_name();
                }
                Key::Char('d') => {
                    app.remove_selected();
                }
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        Actions::NONE
    }
}

use std::{io, vec};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    symbols::border,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
};

fn main() -> Result<(), io::Error> {
    let recipes = vec!["Pizza", "Hamburbur", "Some food  think"];
    let mut selected = 0;
    let mut recipes_status = ListState::default();
    recipes_status.select(Some(selected));
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length((3)),
                    Constraint::Length((1)),
                    Constraint::Min((10)),
                ])
                .split(f.size());

            let baslik = Paragraph::new("HELLO CHEF")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));

            let items: Vec<ListItem> = recipes
                .iter()
                .map(|recipe| ListItem::new(*recipe))
                .collect();

            let list_widget = List::new(items)
                .block(
                    Block::default()
                        .title("Select a recipe pls")
                        .borders(Borders::ALL),
                )
                .highlight_symbol(">> ")
                .highlight_style(
                    ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                );

            f.render_widget(baslik, chunks[0]);
            f.render_stateful_widget(list_widget, chunks[2], &mut recipes_status)
        })?;

        if event::poll(std::time::Duration::from_millis(600))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Down => {
                            let i = match recipes_status.selected() {
                                Some(i) => {
                                    if i >= recipes.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            recipes_status.select(Some(i));
                        }
                        KeyCode::Up => {
                            let i = match recipes_status.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        recipes.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            recipes_status.select(Some(i));
                        }

                        _ => {}
                    }
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

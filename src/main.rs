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
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

enum Screen {
    Menu,
    Detail,
}

struct Recipe {
    name: &'static str,
    description: &'static str,
}

fn main() -> Result<(), io::Error> {
    let recipes = vec![
        Recipe {
            name: "Pizza",
            description: "just a dough",
        },
        Recipe {
            name: "Ratatouille",
            description: "idunno either",
        },
    ];
    let mut recipes_status = ListState::default();
    let mut screen = Screen::Menu;
    recipes_status.select(Some(0));
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| match screen {
            Screen::Menu => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length((3)),
                        Constraint::Length((1)),
                        Constraint::Min((10)),
                    ])
                    .split(f.area());

                let baslik = Paragraph::new("🦀 WELCOME CHEF 🦀")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red))
                    .block(Block::default().borders(Borders::ALL));

                let items: Vec<ListItem> = recipes
                    .iter()
                    .map(|recipe| ListItem::new(recipe.name))
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
            }
            Screen::Detail => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length((3)),
                        Constraint::Length((1)),
                        Constraint::Min((10)),
                    ])
                    .split(f.area());

                let selected = recipes_status.selected().unwrap_or(0);
                let food_recipe = Paragraph::new(recipes[selected].description)
                    .block(
                        Block::default()
                            .border_style(Style::default().fg(Color::Cyan))
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));
                let welcome_recipe = Paragraph::new("FOOD RECPIE")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                f.render_widget(welcome_recipe, chunks[0]);
                f.render_widget(food_recipe, chunks[2]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(300))? {
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
                        KeyCode::Enter => {
                            screen = Screen::Detail;
                        }
                        KeyCode::Backspace => {
                            screen = Screen::Menu;
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

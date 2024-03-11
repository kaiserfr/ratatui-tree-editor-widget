use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    widgets::Block,
    Terminal,
};
use std::error::Error;
use std::io;

use tui_tree_widget::{Tree, TreeItem, TreeState};

struct App<'a> {
    state: TreeState<&'static str>,
    items: Vec<TreeItem<'a, &'static str>>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: vec![
                TreeItem::new_leaf("a", "Alfa"),
                TreeItem::new(
                    "b",
                    "Bravo",
                    vec![
                        TreeItem::new_leaf("c", "Charlie"),
                        TreeItem::new(
                            "d",
                            "Delta",
                            vec![
                                TreeItem::new_leaf("e", "Echo"),
                                TreeItem::new_leaf("f", "Foxtrot"),
                            ],
                        )
                        .expect("all item identifiers are unique"),
                        TreeItem::new_leaf("g", "Golf"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("h", "Hotel"),
                TreeItem::new(
                    "i",
                    "India",
                    vec![
                        TreeItem::new_leaf("j", "Juliett"),
                        TreeItem::new_leaf("k", "Kilo"),
                        TreeItem::new_leaf("l", "Lima"),
                        TreeItem::new_leaf("m", "Mike"),
                        TreeItem::new_leaf("n", "November"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("o", "Oscar"),
            ],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let mut app = App::new();
    let res = run_app(&mut terminal,&mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<'a, B: Backend>(terminal: &mut Terminal<B>, app: &'a mut App<'a>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.size();

            let items = Tree::new(app.items.clone())
                .expect("all item identifiers are unique")
                .block(Block::bordered().title(format!("Tree Widget {:?}", app.state)))
                .highlight_style(
                    Style::new()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, area, &mut app.state);
        })?;

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Up if key.modifiers == KeyModifiers::SHIFT => {
                    app.state.key_shift_up(&mut app.items)
                }

                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('\n' | ' ') => app.state.toggle_selected(),
                KeyCode::Left => app.state.key_left(),
                KeyCode::Right => app.state.key_right(),
                KeyCode::Down => app.state.key_down(&app.items),
                KeyCode::Up => app.state.key_up(&app.items),
                KeyCode::Home => {
                    app.state.select_first(&app.items);
                }
                KeyCode::End => {
                    app.state.select_last(&app.items);
                }
                KeyCode::PageDown => app.state.scroll_down(3),
                KeyCode::PageUp => app.state.scroll_up(3),
                _ => {}
            },

            Event::Mouse(mouse) => match mouse.kind {
                event::MouseEventKind::ScrollDown => app.state.scroll_down(1),
                event::MouseEventKind::ScrollUp => app.state.scroll_up(1),
                _ => {}
            },
            _ => {}
        }
    }
}

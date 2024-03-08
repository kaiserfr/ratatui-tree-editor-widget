use std::error::Error;

use tui_tree_widget::{TreeItem, TreeState};

#[derive(Debug)]
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
    println!("--------------------------------------------");

    let mut app = App::new();

    let item = &mut app.items[1];

    item.clone().print_tree(0);

    println!("--------------------------------------------");

    app.state.key_down(&app.items);
    app.state.key_down(&app.items);
    app.state.key_right();
    app.state.key_down(&app.items);
    app.state.key_down(&app.items);
    app.state.key_right();
    app.state.key_down(&app.items);
    let app_items = app.state.key_shift_up(&mut app.items);

    println!("{:?}", app.state);
    println!("--------------------------------------------");


    println!("--------------------------------------------");
    println!("{:?}", app.state);

    let item = &app_items[1];
    item.clone().print_tree(0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert_eq!(app.state, TreeState::default());
        assert_eq!(app.items.len(), 5);
    }

    #[test]
    fn test_app_state_key_down() {
        let mut app = App::new();
        app.state.key_down(&app.items);
        assert_eq!(app.state.selected(), Some(&"b"));
    }

    #[test]
    fn test_app_state_key_right() {
        let mut app = App::new();
        app.state.key_down(&app.items);
        app.state.key_right();
        assert_eq!(app.state.selected(), Some(&"c"));
    }

    #[test]
    fn test_app_state_key_shift_up() {
        let mut app = App::new();
        app.state.key_down(&app.items);
        app.state.key_down(&app.items);
        app.state.key_right();
        app.state.key_down(&app.items);
        app.state.key_down(&app.items);
        app.state.key_shift_up(&mut app.items);
        assert_eq!(app.state.selected(), Some(&"d"));
    }
}

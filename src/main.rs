use crate::app::App;
use crate::event::handle_key_event;
use crate::ui::draw;
use crossterm::event::{read, Event};
use std::io::stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod event;
mod ui;

fn main() {
    let mut app = App::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    loop {
        draw(&mut terminal, &mut app);

        match read().unwrap() {
            Event::Key(event) => handle_key_event(event, &mut app),
            _ => {}
        }
    }
}

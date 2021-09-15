use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(event: KeyEvent, app: &mut App) {
    match event.code {
        KeyCode::Char(c) => app.input += &c.to_string(),
        _ => {}
    }
}

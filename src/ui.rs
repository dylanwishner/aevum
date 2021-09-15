use crate::app::App;
use crossterm::cursor::position;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::{Frame, Terminal};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    terminal
        .draw(|f| {
            draw_clock_widget(f, app);
        })
        .unwrap();
}

fn draw_clock_widget<B: Backend>(f: &mut Frame<B>, app: &App) {
    let widget = Paragraph::new(app.input.clone()).block(
        Block::default()
            .title(" Clock Speed (MHz) ")
            .borders(Borders::ALL),
    );

    let cursor_position = position().unwrap().1;
    f.render_widget(widget, Rect::new(0, cursor_position, f.size().width, 3))
}

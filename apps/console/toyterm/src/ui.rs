use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::views::{Styles, View};

pub fn ui(frame: &mut Frame, app: &mut App, view: &mut dyn View) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Styles::border_style_default());

    let title = Paragraph::new(Text::styled(
        view.title(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let navigation_text = Line::from(Span::styled(view.navigation_text(app), Style::default()));

    view.render(frame, chunks[1], app);

    let mode_footer =
        Paragraph::new(vec![Line::default(), navigation_text]).block(Block::default());

    frame.render_widget(mode_footer, chunks[2]);
}

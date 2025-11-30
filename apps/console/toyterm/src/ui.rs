use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::views::{self, View};

pub fn ui(styles: &views::Styles, frame: &mut Frame, app: &mut App, view: &mut dyn View) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        view.title(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    view.render(styles, frame, chunks[1], app);

    let navigation_text = Span::styled(view.navigation_text(), Style::default().fg(Color::Green));

    let mode_footer =
        Paragraph::new(Line::from(navigation_text)).block(Block::default().borders(Borders::ALL));

    let current_keys_hint = Span::styled(view.hint_text(), Style::default().fg(Color::Red));

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}

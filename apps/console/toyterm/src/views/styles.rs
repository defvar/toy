use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::{
    style::{self, Style},
    text::Span,
};
use std::borrow::Cow;

pub struct Styles {}

impl Styles {
    pub fn border_block(&self) -> ratatui::widgets::Block<'_> {
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
    }
}

pub(crate) fn span_bold<'a>(text: impl Into<Cow<'a, str>>) -> Span<'a> {
    Span::styled(text, Style::default().add_modifier(style::Modifier::BOLD))
}

pub(crate) fn span<'a>(text: impl Into<Cow<'a, str>>) -> Span<'a> {
    Span::styled(text, Style::default())
}

pub fn center_split(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1] // Return the middle chunk
}

/// Split horizontally at the specified ratio.
pub fn horizontal_split(percent_x: u16, r: Rect) -> (Rect, Rect) {
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(percent_x),
            Constraint::Percentage(100 - percent_x),
        ])
        .split(r);

    (areas[0], areas[1])
}

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::Block;
use ratatui::{style::Style, text::Span};
use std::borrow::Cow;
use std::sync::OnceLock;

static STYLES: OnceLock<Option<Styles>> = OnceLock::new();

pub struct Styles {}

impl Styles {
    pub fn get() -> &'static Styles {
        if let Some(c) = STYLES.get_or_init(move || Some(Styles {})).as_ref() {
            c
        } else {
            panic!("missing styles.")
        }
    }

    pub fn border_style_default() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn border_style_focus() -> Style {
        Style::default().fg(Color::Green)
    }

    pub fn border_block(&self) -> ratatui::widgets::Block<'_> {
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Self::border_style_default())
    }

    pub fn block_query_text() -> Block<'static> {
        Block::default().style(Style::default().bg(Color::DarkGray).fg(Color::Black))
    }

    pub fn block_query_text_input() -> Block<'static> {
        Block::default().style(Style::default().bg(Color::DarkGray).fg(Color::White))
    }
}

pub(crate) fn span<'a>(text: impl Into<Cow<'a, str>>) -> Span<'a> {
    Span::styled(text, Style::default())
}

pub fn split_center(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
pub fn split_horizontal(percent_x: u16, r: Rect) -> (Rect, Rect) {
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(percent_x),
            Constraint::Percentage(100 - percent_x),
        ])
        .split(r);

    (areas[0], areas[1])
}

pub fn split_vertical_with_length(length: u16, r: Rect) -> (Rect, Rect) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(length)])
        .split(r);

    (areas[0], areas[1])
}

pub fn padding(r: Rect, top: u16, right: u16, bottom: u16, left: u16) -> Rect {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top),
            Constraint::Min(1),
            Constraint::Length(bottom),
        ])
        .split(r);

    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left),
            Constraint::Min(1),
            Constraint::Length(right),
        ])
        .split(areas[1]);

    areas[1]
}

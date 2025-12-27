use crate::views::styles;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

#[derive(Default)]
pub struct ErrorPopup {
    message: String,
}

impl ErrorPopup {
    pub fn with(message: impl Into<String>) -> ErrorPopup {
        ErrorPopup {
            message: message.into(),
        }
    }

    pub fn to_widget_and_area(&self, base_area: Rect) -> (Paragraph<'_>, Rect) {
        let popup_block = Block::default()
            .title("error")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let text = Text::styled(&self.message, Style::default().fg(Color::Red));

        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let paragraph = Paragraph::new(text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let popup_area = styles::split_center(60, 25, base_area);

        (paragraph, popup_area)
    }
}

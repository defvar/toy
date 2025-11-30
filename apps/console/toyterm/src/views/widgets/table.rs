use crate::views::styles::span;
use crate::views::{Styles, TABLE_HIGHLIGHT_SYMBOL};
use ratatui::layout;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Color;
use ratatui::widgets::{Cell, Row, Table};

pub struct Definition {
    title: Option<String>,
    columns: Vec<Column>,
    row_height: u16,
    row_highlight_symbol: String,
    border_color: Option<Color>,
}
pub struct Column {
    field: String,
    width: u16,
}

impl Definition {
    pub fn with(title: impl Into<String>, columns: Vec<Column>) -> Definition {
        Definition {
            title: Some(title.into()),
            columns,
            row_height: 1,
            row_highlight_symbol: TABLE_HIGHLIGHT_SYMBOL.to_string(),
            border_color: None,
        }
    }

    pub fn row_height(mut self, v: u16) -> Self {
        self.row_height = v;
        self
    }

    pub fn row_highlight_symbol(mut self, v: impl Into<String>) -> Self {
        self.row_highlight_symbol = v.into();
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn to_table<'a, T>(&'a self, styles: &'a Styles, rows: T) -> Table<'a>
    where
        T: IntoIterator<Item = Row<'a>>,
    {
        let header = Row::new(self.columns.iter().map(|v| Cell::from(v.field.as_str())))
            .height(self.row_height)
            .style(Style::default().add_modifier(Modifier::BOLD));

        let fields_width = layout::Constraint::Percentage(100);
        let mut column_layouts = self
            .columns
            .iter()
            .map(|c| layout::Constraint::Length(c.width))
            .collect::<Vec<_>>();
        column_layouts.push(fields_width);

        let mut table = Table::default()
            .rows(rows)
            .header(header)
            .widths(column_layouts)
            .highlight_symbol(self.row_highlight_symbol.as_str());

        let mut border_block = styles.border_block();
        if let Some(title) = &self.title {
            border_block = border_block.title(span(title));
        }
        if let Some(color) = self.border_color {
            border_block = border_block.border_style(Style::default().fg(color));
        }

        table = table.block(border_block);
        table
    }
}

impl Column {
    pub fn with(field: impl Into<String>, width: u16) -> Self {
        Self {
            field: field.into(),
            width,
        }
    }
}

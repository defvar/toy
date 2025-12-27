use crate::states::loading::LoadingState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loading<'a> {
    label: Option<ratatui::text::Span<'a>>,
    style: ratatui::style::Style,
    bar_style: ratatui::style::Style,
}

impl Default for Loading<'_> {
    fn default() -> Self {
        Self {
            label: Some(ratatui::text::Span::from("loading")),
            style: ratatui::style::Style::default(),
            bar_style: ratatui::style::Style::default(),
        }
    }
}

impl ratatui::widgets::Widget for Loading<'_> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let mut state = LoadingState::default();
        state.calc_step(0);
        ratatui::widgets::StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl ratatui::widgets::StatefulWidget for Loading<'_> {
    type State = LoadingState;

    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        buf.set_style(area, self.style);

        let throbber_area = area;
        if throbber_area.height < 1 {
            return;
        }

        // render a symbol.
        let symbol = {
            state.normalize();
            let len = state.symbols().len() as i8;
            if 0 <= state.index() && state.index() < len {
                state.symbols()[state.index() as usize].clone()
            } else {
                state.empty()
            }
        };

        let symbol_span = ratatui::text::Span::styled(format!("{} ", symbol), self.bar_style);
        let (col, row) = buf.set_span(
            throbber_area.left(),
            throbber_area.top(),
            &symbol_span,
            symbol_span.width() as u16,
        );

        // render a label.
        if let Some(label) = self.label {
            if throbber_area.right() <= col {
                return;
            }
            buf.set_span(col, row, &label, label.width() as u16);
        }
    }
}

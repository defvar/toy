use crate::app::App;
use crate::states::AppActions;
use crate::states::graph::{CurrentArea, GraphAction};
use crate::views::widgets::error_popup::ErrorPopup;
use crate::views::widgets::loading::Loading;
use crate::views::widgets::table::{Column, Definition};
use crate::views::{Styles, View, styles};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row};

#[derive(Default)]
pub struct GraphView {}

impl View for GraphView {
    fn title(&self) -> String {
        "Graph".to_owned()
    }

    fn hint_text(&self) -> String {
        "(q) back to top / (r) reload".to_owned()
    }

    fn navigation_text(&self) -> String {
        "".to_owned()
    }

    fn render(&mut self, styles: &Styles, frame: &mut Frame, area: Rect, app: &mut App) {
        // error
        if let Some(e) = app.state_mut().graph_mut().error() {
            let e = ErrorPopup::with(e);
            let (error, error_area) = e.to_widget_and_area(area);
            frame.render_widget(error, error_area);
            return;
        }

        if app.state().graph().is_empty() {
            frame.render_stateful_widget(
                Loading::default(),
                area,
                app.state_mut().graph_mut().loading_state_mut(),
            );
            app.tx.send(AppActions::Graph(GraphAction::Tick)).unwrap();
            return;
        }

        let mut table_def = Definition::with(
            "(l) graph",
            vec![Column::with("name", 10), Column::with("label", 20)],
        );

        let items = app.state().graph().items().to_vec();
        let selected = app.state().graph().selected();
        let rows = items.iter().map(|x| {
            Row::new(vec![
                Cell::from(x.name()),
                Cell::from(
                    x.labels()
                        .iter()
                        .map(|x| format!("{}={}", x.key(), x.value()))
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ])
        });

        if app.state().graph().current_area() == CurrentArea::List {
            table_def = table_def.border_color(Color::Green);
        }

        let table = table_def.to_table(styles, rows);

        let json = selected
            .map(|x| toy_pack_json::pack_to_string_pretty(&x.services()))
            .map(|x| x.unwrap_or("".to_owned()));
        let json_lines = json.map(|x| {
            x.split('\n')
                .enumerate()
                .map(|(idx, x)| {
                    let row = idx + 1;
                    Line::from(vec![
                        Span::styled(format!("{row:>3}"), Style::default().fg(Color::DarkGray)),
                        Span::styled("  ", Style::default()),
                        Span::styled(x.to_owned(), Style::default()),
                    ])
                })
                .collect::<Vec<_>>()
        });
        let json_span_style = if app.state().graph().current_area() == CurrentArea::Detail {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };
        let json_span = Paragraph::new(json_lines.unwrap_or(vec![]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(json_span_style)
                    .title_top("(d) detail"),
            )
            .scroll((app.state().graph().scroll_vertical(), 0));

        let (left, right) = styles::horizontal_split(50, area);
        frame.render_stateful_widget(table, left, app.state_mut().graph_mut().state_mut());
        frame.render_widget(json_span, right);
    }

    fn handle(&self, key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Char('q') => app.tx.send(AppActions::BackToTop).unwrap(),
            KeyCode::Char('r') => app
                .tx
                .send(AppActions::Graph(GraphAction::GetGraphListStart))
                .unwrap(),
            KeyCode::Char('d') => app
                .tx
                .send(AppActions::Graph(GraphAction::ChangeArea(
                    CurrentArea::Detail,
                )))
                .unwrap(),
            KeyCode::Char('l') => app
                .tx
                .send(AppActions::Graph(GraphAction::ChangeArea(
                    CurrentArea::List,
                )))
                .unwrap(),
            KeyCode::Up => app.tx.send(AppActions::Graph(GraphAction::Up)).unwrap(),
            KeyCode::Down => app.tx.send(AppActions::Graph(GraphAction::Down)).unwrap(),
            _ => {}
        };
    }
}

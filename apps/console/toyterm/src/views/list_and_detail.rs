use crate::app::App;
use crate::states::list_and_detail::{Filter, ListAndDetailActions, ListAndDetailState, Mode};
use crate::states::{AppActions, list_and_detail::CurrentArea};
use crate::views::widgets::error_popup::ErrorPopup;
use crate::views::widgets::loading::Loading;
use crate::views::widgets::table::Definition;
use crate::views::{Styles, styles};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row};
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Default)]
pub struct ListAndDetailView<A> {
    list_count: usize,
    detail_line_count: usize,
    detail_height: u16,
    _tp: PhantomData<A>,
}

impl<A> ListAndDetailView<A>
where
    A: ListAndDetailActions,
{
    pub fn navigation_text<T, S, F>(&self, state_func: S, app: &App) -> String
    where
        T: Clone + Serialize,
        F: Filter<T>,
        S: Fn(&App) -> &ListAndDetailState<T, F>,
    {
        let state = state_func(app);
        match state.current_mode() {
            Mode::Filter => "(ctrl + c) back".to_owned(),
            Mode::Normal => {
                "(q) back | (r) reload | (←) list | (→) detail | (t) top | (e) end".to_owned()
            }
        }
    }

    pub fn render<T, S, R, F>(
        &mut self,
        state_func: S,
        mut definition: Definition,
        row_creator: R,
        frame: &mut Frame,
        area: Rect,
        app: &mut App,
    ) where
        T: Clone + Serialize,
        S: Fn(&mut App) -> &mut ListAndDetailState<T, F>,
        F: Filter<T>,
        R: Fn(&T) -> Row,
    {
        let state = state_func(app);

        // error
        if let Some(e) = state.error() {
            let e = ErrorPopup::with(e);
            let (error, error_area) = e.to_widget_and_area(area);
            frame.render_widget(error, error_area);
            return;
        }

        if state.is_empty() {
            frame.render_stateful_widget(Loading::default(), area, state.loading_state_mut());
            app.tx.send(A::tick()).unwrap();
            return;
        }

        let items = state.items().cloned().collect::<Vec<_>>();
        self.list_count = items.len();

        let selected = state.selected();
        let rows = items.iter().map(row_creator);

        if state.current_area() == CurrentArea::List {
            definition = definition.border_color(Color::Green);
        }

        definition = definition.borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
        let table = definition.to_table(Styles::get(), rows);

        let json = selected
            .map(|x| toy_pack_json::pack_to_string_pretty(&x))
            .map(|x| x.unwrap_or("".to_owned()));
        let json_lines = json
            .map(|x| {
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
            })
            .unwrap_or(vec![]);

        self.detail_line_count = json_lines.len();

        let (list_border_style, detail_border_style) =
            if state.current_area() == CurrentArea::Detail {
                (Styles::border_style_default(), Styles::border_style_focus())
            } else {
                (Styles::border_style_focus(), Styles::border_style_default())
            };
        let json_span = Paragraph::new(json_lines)
            .block(
                Block::default()
                    .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                    .border_style(detail_border_style)
                    .title_top("(d) detail"),
            )
            .scroll((state.scroll_position().detail().y, 0));

        let list_query_block = Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
            .border_style(list_border_style);
        let deatil_query_block = Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
            .border_style(detail_border_style);
        let list_query_span =
            if state.filtering_input().is_none() && state.current_mode() == Mode::Normal {
                Paragraph::new(Line::from("/ to filter")).block(Styles::block_query_text())
            } else {
                Paragraph::new(Line::from(
                    state
                        .filtering_input()
                        .map(|x| x.to_owned())
                        .unwrap_or("".to_owned()),
                ))
                .block(Styles::block_query_text_input())
            };
        let detail_query_span =
            Paragraph::new(Line::from("/ to search")).block(Styles::block_query_text());

        let (main, footer) = styles::split_vertical_with_length(2, area);
        let (footer_left_block, footer_right_block) = styles::split_horizontal(50, footer);
        let footer_left = styles::padding(footer_left_block, 0, 1, 1, 1);
        let footer_right = styles::padding(footer_right_block, 0, 1, 1, 1);
        let (left, right) = styles::split_horizontal(50, main);

        self.detail_height = right.height - 1; //border size
        frame.render_stateful_widget(table, left, state.table_state_mut());
        frame.render_widget(json_span, right);

        frame.render_widget(list_query_block, footer_left_block);
        frame.render_widget(list_query_span, footer_left);

        frame.render_widget(deatil_query_block, footer_right_block);
        frame.render_widget(detail_query_span, footer_right);
    }

    pub fn handle<T, S, F>(&self, state_func: S, key: KeyEvent, app: &mut App)
    where
        T: Clone,
        S: Fn(&App) -> &ListAndDetailState<T, F>,
        F: Filter<T>,
    {
        let state = state_func(app);
        match state.current_mode() {
            Mode::Filter => {
                match key.code {
                    KeyCode::Char('c')
                        if key.modifiers == KeyModifiers::CONTROL
                            && state.current_mode() == Mode::Filter =>
                    {
                        app.tx.send(A::end_filter()).unwrap()
                    }
                    KeyCode::Char(c) if state.current_mode() == Mode::Filter => {
                        app.tx.send(A::push_filtering_input(c)).unwrap()
                    }
                    KeyCode::Backspace if state.current_mode() == Mode::Filter => {
                        app.tx.send(A::remove_filtering_input()).unwrap()
                    }
                    _ => {}
                };
            }
            Mode::Normal => {
                match key.code {
                    KeyCode::Char('q') => app.tx.send(AppActions::BackToTop).unwrap(),
                    KeyCode::Char('r') => app.tx.send(A::reload()).unwrap(),
                    KeyCode::Char('d') | KeyCode::Right => {
                        app.tx.send(A::change_area(CurrentArea::Detail)).unwrap()
                    }
                    KeyCode::Char('l') | KeyCode::Left => {
                        app.tx.send(A::change_area(CurrentArea::List)).unwrap()
                    }
                    KeyCode::Up => app.tx.send(A::up()).unwrap(),
                    KeyCode::Down => app.tx.send(A::down()).unwrap(),
                    KeyCode::Home | KeyCode::Char('t') => app.tx.send(A::scroll(i16::MIN)).unwrap(),
                    KeyCode::End | KeyCode::Char('e') => {
                        let offset = match state.current_area() {
                            CurrentArea::List => (self.list_count as i16).saturating_sub(1),
                            CurrentArea::Detail => (self.detail_line_count as u16)
                                .saturating_sub(self.detail_height)
                                .saturating_sub(state.scroll_position().detail().y)
                                as i16,
                        };
                        app.tx.send(A::scroll(offset)).unwrap()
                    }
                    KeyCode::Char('/') => match state.current_area() {
                        CurrentArea::List => app.tx.send(A::start_filter()).unwrap(),
                        CurrentArea::Detail => {}
                    },
                    _ => {}
                };
            }
        }
    }
}

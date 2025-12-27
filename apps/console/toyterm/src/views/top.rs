use crate::app::App;
use crate::states::{AppActions, top::TopAction};
use crate::views::View;
use crate::views::styles::{Styles, span};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{self, Span},
    widgets::{List, ListItem},
};

pub struct TopView;

impl View for TopView {
    fn title(&self) -> String {
        "Top".to_string()
    }

    fn navigation_text(&self, _app: &App) -> String {
        "(q) quit".to_string()
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut App) {
        let menu_items: Vec<ListItem> = app
            .state()
            .top()
            .menu_items()
            .iter()
            .map(|i| ListItem::new(vec![text::Line::from(Span::raw(i.to_owned()))]))
            .collect();

        let menu_items = List::new(menu_items)
            .block(Styles::get().border_block().title(span(" menu ")))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        frame.render_stateful_widget(menu_items, area, app.state_mut().top_mut().state_mut());
    }

    fn handle(&self, key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Char('q') => {
                app.tx.send(AppActions::Quit).unwrap();
            }
            KeyCode::Up => app.tx.send(AppActions::Top(TopAction::Up)).unwrap(),
            KeyCode::Down => app.tx.send(AppActions::Top(TopAction::Down)).unwrap(),
            KeyCode::Enter => {
                app.tx.send(AppActions::Top(TopAction::Select)).unwrap();
            }
            _ => {}
        };
    }
}

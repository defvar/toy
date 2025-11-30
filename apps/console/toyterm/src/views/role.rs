use crate::app::App;
use crate::states::{AppActions, role::RoleAction};
use crate::views::View;
use crate::views::styles::{self, Styles};
use crate::views::widgets::{
    error_popup::ErrorPopup,
    loading::Loading,
    table::{Column, Definition},
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{Cell, Row};
use ratatui::{Frame, layout::Rect};

#[derive(Default)]
pub struct RoleView {}

impl View for RoleView {
    fn title(&self) -> String {
        "Roles".to_string()
    }

    fn hint_text(&self) -> String {
        "(q) to back to top".to_string()
    }

    fn navigation_text(&self) -> String {
        "".to_string()
    }

    fn render(&mut self, styles: &Styles, frame: &mut Frame, area: Rect, app: &mut App) {
        // error
        if let Some(e) = app.state_mut().role_mut().error() {
            let e = ErrorPopup::with(e);
            let (error, error_area) = e.to_widget_and_area(area);
            frame.render_widget(error, error_area);
            return;
        }

        if app.state().role().is_empty() {
            frame.render_stateful_widget(
                Loading::default(),
                area,
                app.state_mut().role_mut().loading_state_mut(),
            );
            app.tx.send(AppActions::Role(RoleAction::Tick)).unwrap();
        } else {
            let role_table_def = Definition::with(
                " role ",
                vec![Column::with("name", 10), Column::with("note", 20)],
            );
            let rule_table_def = Definition::with(
                " rule ",
                vec![Column::with("resource", 20), Column::with("verb", 20)],
            );

            let role_items = app.state().role().items().to_vec();
            let current_role = app.state().role().selected();
            let role_rows = role_items.iter().map(|x| {
                Row::new(vec![
                    Cell::from(x.name()),
                    Cell::from(x.note().unwrap_or("")),
                ])
            });

            let rule_rows = current_role
                .map(|c| {
                    c.rules()
                        .iter()
                        .map(|x| (x.resources().join(","), x.verbs().join(",")))
                        .map(|x| Row::new(vec![Cell::from(x.0), Cell::from(x.1)]))
                        .collect::<Vec<_>>()
                })
                .unwrap_or(vec![]);

            let role_table = role_table_def.to_table(styles, role_rows);
            let rule_table = rule_table_def.to_table(styles, rule_rows);

            let (left, right) = styles::horizontal_split(50, area);
            frame.render_stateful_widget(role_table, left, app.state_mut().role_mut().state_mut());
            frame.render_widget(rule_table, right);
        }
    }

    fn handle(&self, key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Char('q') => app.tx.send(AppActions::BackToTop).unwrap(),
            KeyCode::Char('r') => app
                .tx
                .send(AppActions::Role(RoleAction::GetRoleListStart))
                .unwrap(),
            KeyCode::Up => app.tx.send(AppActions::Role(RoleAction::Up)).unwrap(),
            KeyCode::Down => app.tx.send(AppActions::Role(RoleAction::Down)).unwrap(),
            _ => {}
        };
    }
}

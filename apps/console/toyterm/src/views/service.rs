use crate::app::App;
use crate::states::service::ServiceAction;
use crate::views::View;
use crate::views::list_and_detail::ListAndDetailView;
use crate::views::widgets::table::{Column, Definition};
use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::widgets::{Cell, Row};

#[derive(Default)]
pub struct ServiceView {
    inner: ListAndDetailView<ServiceAction>,
}

impl View for ServiceView {
    fn title(&self) -> String {
        "Service".to_owned()
    }

    fn navigation_text(&self, app: &App) -> String {
        self.inner
            .navigation_text(|x| x.state().service().list_and_detail(), app)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &mut App) {
        let def = Definition::with(
            "(l) service",
            vec![
                Column::with("name_space", 30),
                Column::with("service_name", 20),
            ],
        );

        self.inner.render(
            |x| x.state_mut().service_mut().list_and_detail_mut(),
            def,
            |x| {
                Row::new(vec![
                    Cell::from(x.service_type().name_space()),
                    Cell::from(x.service_type().service_name()),
                ])
            },
            frame,
            area,
            app,
        );
    }

    fn handle(&self, key: KeyEvent, app: &mut App) {
        self.inner
            .handle(|x| x.state().service().list_and_detail(), key, app)
    }
}

use crate::app::{App, CurrentView};
use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use std::collections::HashMap;

mod graph;
mod role;
mod styles;
mod top;
mod widgets;

pub use graph::GraphView;
pub use role::RoleView;
pub use styles::Styles;
pub use top::TopView;

const TABLE_HIGHLIGHT_SYMBOL: &str = "> ";

pub trait View {
    fn title(&self) -> String;

    fn hint_text(&self) -> String {
        "(q) to back to top".to_string()
    }

    fn navigation_text(&self) -> String;

    fn render(&mut self, styles: &Styles, frame: &mut Frame, area: Rect, app: &mut App);

    fn handle(&self, key: KeyEvent, app: &mut App);
}

/// Hold the created `View`
#[derive(Default)]
pub struct ViewContainer {
    views: HashMap<CurrentView, Box<dyn View>>,
}

impl ViewContainer {
    /// Returns the cached view. If it does not exist, a new one is created.
    pub fn view_mut(&mut self, current: CurrentView) -> &mut Box<dyn View> {
        self.views.entry(current).or_insert(current.view())
    }
}

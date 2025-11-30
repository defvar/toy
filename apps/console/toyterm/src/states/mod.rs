mod list;
mod table;

pub mod graph;
pub mod loading;
pub mod role;
pub mod top;

use crate::app::CurrentView;
use crate::states::graph::GraphAction;
use role::RoleAction;
use top::TopAction;

pub trait Action {}

#[derive(Debug, Clone)]
pub enum AppActions {
    Quit,
    BackToTop,
    ChangeView(CurrentView),
    Top(TopAction),
    Role(RoleAction),
    Graph(GraphAction),
}

pub trait State {
    type Action: Action;

    fn transition(&mut self, action: Self::Action);
}

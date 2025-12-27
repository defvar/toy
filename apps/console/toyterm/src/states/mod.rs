mod list;
mod table;

pub mod graph;
pub mod list_and_detail;
pub mod loading;
pub mod role;
pub mod service;
pub mod top;

use crate::app::CurrentView;
use crate::states::graph::GraphAction;
use crate::states::service::ServiceAction;
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
    Service(ServiceAction),
    Graph(GraphAction),
}

pub trait State {
    type Action: Action;

    fn transition(&mut self, action: Self::Action);
}

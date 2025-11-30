use crate::app::CurrentView;
use crate::states::graph::GraphState;
use crate::states::{AppActions, role::RoleState, top::TopState};
use tokio::sync::mpsc::UnboundedSender;

pub struct AppState {
    current_view: CurrentView,
    top: TopState,
    role: RoleState,
    graph: GraphState,
}

impl AppState {
    pub fn new(tx: UnboundedSender<AppActions>) -> Self {
        Self {
            current_view: CurrentView::Top,
            top: TopState::new(tx.clone()),
            role: RoleState::new(tx.clone()),
            graph: GraphState::new(tx.clone()),
        }
    }

    pub fn current_view(&self) -> CurrentView {
        self.current_view
    }

    pub fn change_view(&mut self, v: CurrentView) {
        self.current_view = v;
    }

    pub fn top(&self) -> &TopState {
        &self.top
    }

    pub fn top_mut(&mut self) -> &mut TopState {
        &mut self.top
    }

    pub fn role(&self) -> &RoleState {
        &self.role
    }

    pub fn role_mut(&mut self) -> &mut RoleState {
        &mut self.role
    }

    pub fn graph(&self) -> &GraphState {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut GraphState {
        &mut self.graph
    }
}

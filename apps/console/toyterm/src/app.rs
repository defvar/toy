use crate::state::AppState;
use crate::states::{AppActions, State};
use crate::views;
use crate::views::View;
use std::sync::OnceLock;
use tokio::sync::mpsc::UnboundedSender;
use toy::api_client::http::HttpApiClient;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CurrentView {
    Top,
    Role,
    Service,
    Graph,
}

impl CurrentView {
    pub fn view(&self) -> Box<dyn View> {
        match self {
            CurrentView::Top => Box::new(views::TopView),
            CurrentView::Role => Box::new(views::RoleView::default()),
            CurrentView::Service => Box::new(views::ServiceView::default()),
            CurrentView::Graph => Box::new(views::GraphView::default()),
        }
    }
}

static CLIENT: OnceLock<Option<HttpApiClient>> = OnceLock::new();

pub struct App {
    state: AppState,
    pub tx: UnboundedSender<AppActions>,
    should_quit: bool,
}

impl App {
    pub fn with(tx: UnboundedSender<AppActions>, client: HttpApiClient) -> Self {
        let _ = CLIENT.get_or_init(move || Some(client.clone()));
        Self {
            state: AppState::new(tx.clone()),
            tx,
            should_quit: false,
        }
    }

    pub fn client() -> &'static HttpApiClient {
        if let Some(c) = CLIENT.get_or_init(move || None).as_ref() {
            c
        } else {
            panic!("missing http api client.")
        }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn update(&mut self, action: AppActions) {
        match action {
            AppActions::Quit => {
                self.should_quit = true;
            }
            AppActions::BackToTop => self.state.change_view(CurrentView::Top),
            AppActions::ChangeView(v) => self.state.change_view(v),
            AppActions::Top(a) => self.state.top_mut().transition(a),
            AppActions::Role(a) => self.state.role_mut().transition(a),
            AppActions::Service(a) => self.state.service_mut().transition(a),
            AppActions::Graph(a) => self.state.graph_mut().transition(a),
        }
    }
}

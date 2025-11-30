use crate::app::App;
use crate::states::loading::LoadingState;
use crate::states::table::StatefulTable;
use crate::states::{Action, AppActions, State};
use ratatui::widgets::TableState;
use tokio::sync::mpsc::UnboundedSender;
use toy::api_client::toy_api::common::{ListObject, ListOption};
use toy::api_client::toy_api::graph::Graph;
use toy::api_client::{ApiClient, client::GraphClient};

pub struct GraphState {
    raw: StatefulTable<Graph>,
    tx: UnboundedSender<AppActions>,
    error: Option<String>,
    loading_state: LoadingState,
    current_area: CurrentArea,
    scroll: u16,
}

#[derive(Debug, Clone)]
pub enum GraphAction {
    GetGraphListStart,
    UpdateGraphList(Vec<Graph>),
    Tick,
    ChangeArea(CurrentArea),
    OnError(String),
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentArea {
    List,
    Detail,
}

impl Action for GraphAction {}

impl State for GraphState {
    type Action = GraphAction;

    fn transition(&mut self, action: Self::Action) {
        match action {
            GraphAction::GetGraphListStart => {
                self.clear();
                let tx = self.tx.clone();
                toy_rt::spawn(async move {
                    match App::client().graph().list(ListOption::new()).await {
                        Ok(list) => {
                            tx.send(AppActions::Graph(GraphAction::UpdateGraphList(
                                list.items().into(),
                            )))
                            .unwrap();
                        }
                        Err(e) => tx
                            .send(AppActions::Graph(GraphAction::OnError(e.error_message())))
                            .unwrap(),
                    }
                });
            }
            GraphAction::UpdateGraphList(list) => {
                self.raw.set_items(list);
            }
            GraphAction::Tick => self.loading_state.tick(),
            GraphAction::ChangeArea(current_area) => {
                self.focus(current_area);
            }
            GraphAction::OnError(msg) => {
                *self.error_mut() = Some(msg.to_string());
            }
            GraphAction::Up => match self.current_area() {
                CurrentArea::List => self.raw.previous(),
                CurrentArea::Detail => self.scroll_vertical_previous(),
            },
            GraphAction::Down => match self.current_area() {
                CurrentArea::List => self.raw.next(),
                CurrentArea::Detail => self.scroll_vertial_next(),
            },
        }
    }
}

impl GraphState {
    pub fn new(tx: UnboundedSender<AppActions>) -> GraphState {
        GraphState {
            raw: StatefulTable::new(),
            tx,
            error: None,
            loading_state: LoadingState::default(),
            current_area: CurrentArea::List,
            scroll: 0,
        }
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.clear_errors();
        self.scroll_top();
        self.current_area = CurrentArea::List;
    }

    pub fn focus(&mut self, area: CurrentArea) {
        self.current_area = area;
    }

    pub fn current_area(&self) -> CurrentArea {
        self.current_area
    }

    pub fn scroll_vertical(&self) -> u16 {
        self.scroll
    }

    pub fn scroll_top(&mut self) {
        self.scroll = 0;
    }

    pub fn scroll_vertial_next(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub fn scroll_vertical_previous(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn is_empty(&self) -> bool {
        self.raw.items().is_empty()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn error_mut(&mut self) -> &mut Option<String> {
        &mut self.error
    }

    pub fn loading_state_mut(&mut self) -> &mut LoadingState {
        &mut self.loading_state
    }

    pub fn clear_errors(&mut self) {
        self.error = None;
    }

    pub fn items(&self) -> &[Graph] {
        self.raw.items()
    }

    pub fn selected(&self) -> Option<&Graph> {
        self.raw.selected()
    }

    pub fn state_mut(&mut self) -> &mut TableState {
        self.raw.state_mut()
    }
}

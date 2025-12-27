use crate::app::App;
use crate::states::list_and_detail::{
    CurrentArea, Filter, ListAndDetailActions, ListAndDetailState, Mode,
};
use crate::states::{Action, AppActions, State};
use tokio::sync::mpsc::UnboundedSender;
use toy::api_client::toy_api::common::{ListObject, ListOption};
use toy::api_client::toy_api::graph::Graph;
use toy::api_client::{ApiClient, client::GraphClient};

pub struct GraphState {
    raw: ListAndDetailState<Graph, GraphFilter>,
    tx: UnboundedSender<AppActions>,
}

#[derive(Debug, Clone)]
pub enum GraphAction {
    None,
    GetGraphListStart,
    UpdateGraphList(Vec<Graph>),
    Tick,
    ChangeArea(CurrentArea),
    OnError(String),
    Up,
    Down,
    Scroll(i16),
    StartFilter,
    Filtering(char),
    RemoveFilteringInput,
    EndFilter,
}

impl Action for GraphAction {}

impl Default for GraphAction {
    fn default() -> Self {
        Self::None
    }
}

impl ListAndDetailActions for GraphAction {
    fn reload() -> AppActions {
        AppActions::Graph(GraphAction::GetGraphListStart)
    }

    fn tick() -> AppActions {
        AppActions::Graph(GraphAction::Tick)
    }

    fn change_area(area: CurrentArea) -> AppActions {
        AppActions::Graph(GraphAction::ChangeArea(area))
    }

    fn up() -> AppActions {
        AppActions::Graph(GraphAction::Up)
    }

    fn down() -> AppActions {
        AppActions::Graph(GraphAction::Down)
    }

    fn scroll(v: i16) -> AppActions {
        AppActions::Graph(GraphAction::Scroll(v))
    }

    fn start_filter() -> AppActions {
        AppActions::Graph(GraphAction::StartFilter)
    }

    fn push_filtering_input(input: char) -> AppActions {
        AppActions::Graph(GraphAction::Filtering(input))
    }

    fn remove_filtering_input() -> AppActions {
        AppActions::Graph(GraphAction::RemoveFilteringInput)
    }

    fn end_filter() -> AppActions {
        AppActions::Graph(GraphAction::EndFilter)
    }
}

impl State for GraphState {
    type Action = GraphAction;

    fn transition(&mut self, action: Self::Action) {
        match action {
            GraphAction::None => (),
            GraphAction::GetGraphListStart => {
                self.raw.clear();
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
            GraphAction::Tick => self.raw.loading_state_mut().tick(),
            GraphAction::ChangeArea(current_area) => {
                self.raw.focus(current_area);
            }
            GraphAction::OnError(msg) => {
                *self.raw.error_mut() = Some(msg.to_string());
            }
            GraphAction::Up => self.raw.scroll((0, -1)),
            GraphAction::Down => self.raw.scroll((0, 1)),
            GraphAction::Scroll(v) => self.raw.scroll((0, v)),
            GraphAction::StartFilter => {
                self.raw.change_mode(Mode::Filter);
            }
            GraphAction::Filtering(v) => self.raw.push_filtering_input(v),
            GraphAction::RemoveFilteringInput => self.raw.remove_filtering_input(),
            GraphAction::EndFilter => {
                self.raw.change_mode(Mode::Normal);
            }
        }
    }
}

pub struct GraphFilter;

impl Filter<Graph> for GraphFilter {
    fn apply(&self, v: &Graph, input: &str) -> bool {
        v.name().contains(input)
    }
}

impl GraphState {
    pub fn new(tx: UnboundedSender<AppActions>) -> GraphState {
        GraphState {
            raw: ListAndDetailState::new(GraphFilter),
            tx,
        }
    }

    pub fn list_and_detail(&self) -> &ListAndDetailState<Graph, GraphFilter> {
        &self.raw
    }

    pub fn list_and_detail_mut(&mut self) -> &mut ListAndDetailState<Graph, GraphFilter> {
        &mut self.raw
    }
}

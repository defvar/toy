use crate::app::App;
use crate::states::list_and_detail::{
    CurrentArea, Filter, ListAndDetailActions, ListAndDetailState, Mode,
};
use crate::states::{Action, AppActions, State};
use tokio::sync::mpsc::UnboundedSender;
use toy::api_client::toy_api::common::ListObject;
use toy::api_client::toy_api::services::{ServiceSpec, ServiceSpecListOption};
use toy::api_client::{ApiClient, client::ServiceClient};

pub struct ServiceState {
    raw: ListAndDetailState<ServiceSpec, ServiceFilter>,
    tx: UnboundedSender<AppActions>,
}

#[derive(Debug, Clone)]
pub enum ServiceAction {
    None,
    GetServiceListStart,
    UpdateServiceList(Vec<ServiceSpec>),
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

impl Action for ServiceAction {}

impl Default for ServiceAction {
    fn default() -> Self {
        Self::None
    }
}

impl ListAndDetailActions for ServiceAction {
    fn reload() -> AppActions {
        AppActions::Service(ServiceAction::GetServiceListStart)
    }

    fn tick() -> AppActions {
        AppActions::Service(ServiceAction::Tick)
    }

    fn change_area(area: CurrentArea) -> AppActions {
        AppActions::Service(ServiceAction::ChangeArea(area))
    }

    fn up() -> AppActions {
        AppActions::Service(ServiceAction::Up)
    }

    fn down() -> AppActions {
        AppActions::Service(ServiceAction::Down)
    }

    fn scroll(v: i16) -> AppActions {
        AppActions::Service(ServiceAction::Scroll(v))
    }

    fn start_filter() -> AppActions {
        AppActions::Service(ServiceAction::StartFilter)
    }

    fn push_filtering_input(input: char) -> AppActions {
        AppActions::Service(ServiceAction::Filtering(input))
    }

    fn remove_filtering_input() -> AppActions {
        AppActions::Service(ServiceAction::RemoveFilteringInput)
    }

    fn end_filter() -> AppActions {
        AppActions::Service(ServiceAction::EndFilter)
    }
}

impl State for ServiceState {
    type Action = ServiceAction;

    fn transition(&mut self, action: Self::Action) {
        match action {
            ServiceAction::None => (),
            ServiceAction::GetServiceListStart => {
                self.raw.clear();
                let tx = self.tx.clone();
                toy_rt::spawn(async move {
                    match App::client()
                        .service()
                        .list(ServiceSpecListOption::new())
                        .await
                    {
                        Ok(list) => {
                            tx.send(AppActions::Service(ServiceAction::UpdateServiceList(
                                list.items().into(),
                            )))
                            .unwrap();
                        }
                        Err(e) => tx
                            .send(AppActions::Service(ServiceAction::OnError(
                                e.error_message(),
                            )))
                            .unwrap(),
                    }
                });
            }
            ServiceAction::UpdateServiceList(list) => {
                self.raw.set_items(list);
            }
            ServiceAction::Tick => self.raw.loading_state_mut().tick(),
            ServiceAction::ChangeArea(current_area) => {
                self.raw.focus(current_area);
            }
            ServiceAction::OnError(msg) => {
                *self.raw.error_mut() = Some(msg.to_string());
            }
            ServiceAction::Up => self.raw.scroll((0, -1)),
            ServiceAction::Down => self.raw.scroll((0, 1)),
            ServiceAction::Scroll(v) => self.raw.scroll((0, v)),
            ServiceAction::StartFilter => {
                self.raw.change_mode(Mode::Filter);
            }
            ServiceAction::Filtering(c) => self.raw.push_filtering_input(c),
            ServiceAction::RemoveFilteringInput => self.raw.remove_filtering_input(),
            ServiceAction::EndFilter => {
                self.raw.change_mode(Mode::Normal);
            }
        }
    }
}

pub struct ServiceFilter;

impl Filter<ServiceSpec> for ServiceFilter {
    fn apply(&self, v: &ServiceSpec, input: &str) -> bool {
        v.service_type().service_name().contains(input)
            || v.service_type().name_space().contains(input)
    }
}

impl ServiceState {
    pub fn new(tx: UnboundedSender<AppActions>) -> ServiceState {
        ServiceState {
            raw: ListAndDetailState::new(ServiceFilter),
            tx,
        }
    }

    pub fn list_and_detail(&self) -> &ListAndDetailState<ServiceSpec, ServiceFilter> {
        &self.raw
    }

    pub fn list_and_detail_mut(&mut self) -> &mut ListAndDetailState<ServiceSpec, ServiceFilter> {
        &mut self.raw
    }
}

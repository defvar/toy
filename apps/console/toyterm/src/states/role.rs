use crate::app::App;
use crate::states::loading::LoadingState;
use crate::states::table::StatefulTable;
use crate::states::{Action, AppActions, State};
use ratatui::widgets::TableState;
use tokio::sync::mpsc::UnboundedSender;
use toy::api_client::ApiClient;
use toy::api_client::client::{Rbaclient, RoleClient};
use toy::api_client::toy_api::common::{ListObject, ListOption};
use toy::api_client::toy_api::role::Role;

pub struct RoleState {
    raw: StatefulTable<Role>,
    tx: UnboundedSender<AppActions>,
    error: Option<String>,
    loading_state: LoadingState,
}

#[derive(Debug, Clone)]
pub enum RoleAction {
    GetRoleListStart,
    UpdateRoleList(Vec<Role>),
    Tick,
    OnError(String),
    Up,
    Down,
}

impl Action for RoleAction {}

impl State for RoleState {
    type Action = RoleAction;

    fn transition(&mut self, action: Self::Action) {
        match action {
            RoleAction::GetRoleListStart => {
                self.raw.clear();
                self.clear_errors();
                let tx = self.tx.clone();
                toy_rt::spawn(async move {
                    match App::client().rbac().role().list(ListOption::new()).await {
                        Ok(list) => {
                            tx.send(AppActions::Role(RoleAction::UpdateRoleList(
                                list.items().into(),
                            )))
                            .unwrap();
                        }
                        Err(e) => tx
                            .send(AppActions::Role(RoleAction::OnError(e.error_message())))
                            .unwrap(),
                    }
                });
            }
            RoleAction::UpdateRoleList(list) => {
                self.raw.set_items(list);
            }
            RoleAction::Tick => self.loading_state.tick(),
            RoleAction::OnError(msg) => {
                *self.error_mut() = Some(msg.to_string());
            }
            RoleAction::Up => self.raw.previous(),
            RoleAction::Down => self.raw.next(),
        }
    }
}

impl RoleState {
    pub fn new(tx: UnboundedSender<AppActions>) -> RoleState {
        RoleState {
            raw: StatefulTable::new(),
            tx,
            error: None,
            loading_state: LoadingState::default(),
        }
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

    pub fn clear_errors(&mut self) {
        self.error = None;
    }

    pub fn items(&self) -> &[Role] {
        self.raw.items()
    }

    pub fn selected(&self) -> Option<&Role> {
        self.raw.selected()
    }

    pub fn state_mut(&mut self) -> &mut TableState {
        self.raw.state_mut()
    }

    pub fn loading_state_mut(&mut self) -> &mut LoadingState {
        &mut self.loading_state
    }
}

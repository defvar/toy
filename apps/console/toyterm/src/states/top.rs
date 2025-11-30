use crate::app::CurrentView;
use crate::states::graph::GraphAction;
use crate::states::{Action, AppActions, RoleAction, State, list::StatefulList};
use ratatui::widgets::ListState;
use tokio::sync::mpsc::UnboundedSender;

const MENU_ITEMS: [&str; 2] = ["Roles", "Graphs"];

pub struct TopState {
    raw: StatefulList<String>,
    tx: UnboundedSender<AppActions>,
}

impl TopState {
    fn send(&mut self, action: AppActions) {
        self.tx.send(action).unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum TopAction {
    Select,
    Up,
    Down,
}

impl Action for TopAction {}

impl State for TopState {
    type Action = TopAction;

    fn transition(&mut self, action: Self::Action) {
        match action {
            TopAction::Select => {
                match self.state_mut().selected() {
                    None => self.send(AppActions::ChangeView(CurrentView::Top)),
                    Some(i) => match i {
                        0 => {
                            self.send(AppActions::ChangeView(CurrentView::Role));
                            self.send(AppActions::Role(RoleAction::GetRoleListStart));
                        }
                        1 => {
                            self.send(AppActions::ChangeView(CurrentView::Graph));
                            self.send(AppActions::Graph(GraphAction::GetGraphListStart));
                        }
                        _ => self.send(AppActions::ChangeView(CurrentView::Top)),
                    },
                };
            }
            TopAction::Up => self.raw.previous(),
            TopAction::Down => self.raw.next(),
        }
    }
}

impl TopState {
    pub fn new(tx: UnboundedSender<AppActions>) -> TopState {
        let mut menu_items = StatefulList::with_items(
            MENU_ITEMS
                .to_vec()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
        menu_items.state_mut().select_first();

        TopState {
            raw: menu_items,
            tx,
        }
    }

    pub fn menu_items(&self) -> &[String] {
        &self.raw.items()
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        self.raw.state_mut()
    }
}

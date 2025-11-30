use ratatui::widgets::TableState;

pub struct StatefulTable<T> {
    state: TableState,
    items: Vec<T>,
}

#[allow(unused)]
impl<T> StatefulTable<T>
where
    T: Clone,
{
    pub fn new() -> StatefulTable<T> {
        Self {
            state: TableState::default(),
            items: vec![],
        }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn extend_from_slice(&mut self, items: &[T]) {
        self.items.extend_from_slice(items);
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state.select_first();
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.state.select(None);
    }

    pub fn state_mut(&mut self) -> &mut TableState {
        &mut self.state
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn selected(&self) -> Option<&T> {
        if let Some(idx) = self.state.selected() {
            if self.items.len() > idx {
                Some(&self.items[idx])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return self.state.select(None);
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return self.state.select(None);
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

use ratatui::widgets::ListState;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

#[allow(unused)]
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }

    pub fn state(&self) -> &ListState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn select(&mut self, idx: Option<usize>) {
        self.state.select(idx);
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
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

use crate::states::AppActions;
use crate::states::loading::LoadingState;
use crate::states::table::StatefulTable;
use ratatui::layout::Position;
use ratatui::widgets::TableState;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListAndDetailState<T, F> {
    list: StatefulTable<T>,
    error: Option<String>,
    loading_state: LoadingState,
    current_area: CurrentArea,
    scroll_position: ScrollPosition,
    mode: Mode,

    filtering_input: Option<String>,
    filter: F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentArea {
    List,
    Detail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Filter,
}

pub trait ListAndDetailActions {
    fn reload() -> AppActions;

    fn tick() -> AppActions;

    fn change_area(area: CurrentArea) -> AppActions;

    fn up() -> AppActions;

    fn down() -> AppActions;

    fn scroll(v: i16) -> AppActions;

    fn start_filter() -> AppActions;

    fn push_filtering_input(input: char) -> AppActions;

    fn remove_filtering_input() -> AppActions;

    fn end_filter() -> AppActions;
}

pub trait Filter<T> {
    fn apply(&self, v: &T, input: &str) -> bool;
}

pub struct NoopFilter<T> {
    _tp: PhantomData<T>,
}
impl<T> Filter<T> for NoopFilter<T> {
    fn apply(&self, _v: &T, _input: &str) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollPosition {
    list: Position,
    detail: Position,
}

#[allow(dead_code)]
impl ScrollPosition {
    pub fn new(list: Position, detail: Position) -> Self {
        Self { list, detail }
    }

    pub fn with_scroll_position(position: Position) -> ScrollPosition {
        ScrollPosition {
            list: position,
            detail: position,
        }
    }

    pub fn list(&self) -> Position {
        self.list
    }

    pub fn detail(&self) -> Position {
        self.detail
    }
}

impl Default for ScrollPosition {
    fn default() -> Self {
        Self {
            list: Position::default(),
            detail: Position::default(),
        }
    }
}

impl<T, F> ListAndDetailState<T, F>
where
    T: Clone,
    F: Filter<T>,
{
    pub fn new(filter: F) -> Self {
        Self {
            list: StatefulTable::new(),
            error: None,
            loading_state: LoadingState::default(),
            current_area: CurrentArea::List,
            scroll_position: ScrollPosition::default(),
            mode: Mode::Normal,
            filtering_input: None,
            filter,
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
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

    pub fn current_mode(&self) -> Mode {
        self.mode
    }

    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn scroll_position(&self) -> ScrollPosition {
        self.scroll_position
    }

    /// offset 0:x 1:y
    pub fn scroll(&mut self, offset: (i16, i16)) {
        let mut position = match self.current_area {
            CurrentArea::List => self.scroll_position.list,
            CurrentArea::Detail => self.scroll_position.detail,
        };

        position.x = position.x.saturating_add_signed(offset.0);
        position.y = position.y.saturating_add_signed(offset.1);

        match self.current_area {
            CurrentArea::List => {
                let count = self.items().count();
                let max = if count > 0 { count - 1 } else { 0 } as u16;
                position.y = max.min(position.y);
                self.scroll_position.list = position;
                self.select(position.y);
            }
            CurrentArea::Detail => {
                self.scroll_position.detail = position;
            }
        };
    }

    pub fn scroll_top(&mut self) {
        match self.current_area {
            CurrentArea::List => {
                self.scroll_position.list.y = 0;
                self.list.select_first();
            }
            CurrentArea::Detail => {
                self.scroll_position.detail.y = 0;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.items().is_empty()
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

    pub fn items(&self) -> impl Iterator<Item = &T> {
        self.list.items().iter().filter(|i| {
            if let Some(ref input) = self.filtering_input {
                self.filter.apply(i, input)
            } else {
                true
            }
        })
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.list.set_items(items);
    }

    pub fn select(&mut self, idx: u16) {
        self.list.select(idx);
        self.scroll_position.detail = Position::new(0, 0);
    }

    pub fn selected(&self) -> Option<&T> {
        self.list.selected()
    }

    pub fn table_state_mut(&mut self) -> &mut TableState {
        self.list.state_mut()
    }

    pub fn filtering_input(&self) -> Option<&String> {
        self.filtering_input.as_ref()
    }

    pub fn push_filtering_input(&mut self, input: char) {
        match self.filtering_input {
            Some(ref mut filtering_input) => {
                filtering_input.push(input);
            }
            None => self.filtering_input = Some(input.to_string()),
        }
    }

    pub fn remove_filtering_input(&mut self) {
        if let Some(ref mut filtering_input) = self.filtering_input {
            filtering_input.pop();
            if filtering_input.is_empty() {
                self.filtering_input = None;
            }
        }
    }

    pub fn clear_filtering_input(&mut self) {
        self.filtering_input = None;
    }
}

impl<T> Default for ListAndDetailState<T, NoopFilter<T>>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new(NoopFilter {
            _tp: PhantomData::<T>,
        })
    }
}

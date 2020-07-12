use tui::widgets::ListState;

use crate::utils::numbers::get_random_to;

pub struct StatefulList<String> {
    pub state: ListState,
    pub items: Vec<String>,
}

impl<String> StatefulList<String> {
    pub fn with_items(items: Vec<String>) -> StatefulList<String> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next_random(&mut self) -> Option<usize> {
        let number_of_names: i32 = self.items.len() as i32;
        if number_of_names < 1 {
            return None;
        }
        Some(get_random_to(number_of_names) as usize)
    }

    pub fn next(&mut self) {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn remove(&mut self, index: usize) -> String {
        self.items.remove(index)
    }
}

use crate::base::list::StatefulList;

pub struct App {
    pub items: StatefulList<String>,
    original_name: Vec<String>,
    pub current_name: String,
}

impl App {
    pub fn new(names: Vec<String>) -> App {
        App {
            items: StatefulList::with_items(names.clone()),
            original_name: names.clone(),
            current_name: "".to_string(),
        }
    }

    pub fn next_name(&mut self) {
        match self.items.next_random() {
            Some(selected_name_index) => self.pick_name_with_index(selected_name_index),
            None => {}
        }
    }

    pub fn select_current_name(&mut self) {
        match self.items.state.selected() {
            Some(selected_name_index) => self.pick_name_with_index(selected_name_index),
            None => {}
        }
    }

    fn pick_name_with_index(&mut self, index: usize) {
        self.items.unselect();
        let item: String = self.items.remove(index);
        self.current_name = item;
    }

    pub fn unselect(&mut self) {
        self.items.unselect();
    }

    pub fn select_next_name(&mut self) {
        self.items.next();
    }

    pub fn select_previous_name(&mut self) {
        self.items.previous();
    }

    pub fn remove_selected(&mut self) {
        match self.items.state.selected() {
            Some(selected_name_index) => {
                self.items.remove(selected_name_index);
            }
            None => {}
        }
        self.items.unselect();
    }

    pub fn on_tick(&mut self) {}

    pub fn restart(&mut self) {
        self.items = StatefulList::with_items(self.original_name.clone());
        self.unselect();
    }
}

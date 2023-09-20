use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoEntry {
    pub done: bool,
    pub description: String,
}

impl TodoEntry {
    fn mark_as_done(&mut self) {
        self.done = true;
    }

    fn mark_as_not_done(&mut self) {
        self.done = false;
    }
}


impl std::fmt::Display for TodoEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut checkbox = "[ ]";
        if self.done {
            checkbox = "[X]";
        };
        write!(f, "{} {}", checkbox, self.description)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub list_name: String,
    pub file_location: String,
    pub todo_list: Vec<TodoEntry>
}


impl TodoList {
    pub fn save_list(&self) -> bool {
        match serde_json::to_string(&self) {
            Ok(r) => match fs::write(&self.list_name, r) {
                Ok(_) => return true,
                Err(_) => {println!("Failed to write to file");
                    return false}
            }
            Err(e) => panic!("Failed to serialize struct: {e}")
        }
    }

    fn load_list(&self) {
    }
}

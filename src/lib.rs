use serde::{Serialize, Deserialize};

/*
 * TodoEntry Struct
 * Holds the data for a single todo list entry
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoEntry {
    done: bool,
    pub description: String,
}

impl TodoEntry {
    pub fn mark_as_done(&mut self) {
        self.done = true;
    }

    pub fn mark_as_not_done(&mut self) {
        self.done = false;
    }

    pub fn new(desc: String) -> TodoEntry {
        TodoEntry {
            done: false,
            description: desc
        }
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

/*
 * TodoList Struct
 * Holds each TodoEntry in the todo list
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub list_name: String,
    pub todo_list: Vec<TodoEntry>,
}

impl TodoList {
    pub fn print_list(&self) {
        for i in self.todo_list.iter() {
            println!("{}", i);
        }
    }
    
    pub fn new(name: String) -> TodoList {
        TodoList {
            list_name:  name,
            todo_list: Vec::new()
        }
    }

    pub fn add(&mut self, item: TodoEntry) {
        self.todo_list.push(item);
    }

    pub fn remove(&mut self, index: usize) {
        let _ = self.todo_list.remove(index);
    }
    
    pub fn rename(&mut self, name: String) {
        self.list_name = name;
    }
}

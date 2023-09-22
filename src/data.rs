#![allow(dead_code)]
use std::fs;
use serde::{Serialize, Deserialize};

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
    pub todo_list: Vec<TodoEntry>,
}

impl TodoList {
    pub fn print_list(&self) {
        for i in self.todo_list.iter() {
            println!("{}", i);
        }
    }
}

pub fn save_list(list: &mut TodoList) -> bool {
    if !std::path::Path::new("lists").exists(){
        match fs::create_dir("lists") {
            Ok(_) => println!("New directory created"),
            Err(e) => println!("Failed to create directory: {}", e)
        }
    }
    let path = format!("lists/{}.json", &list.list_name);
    match serde_json::to_string(list) {
        Ok(r) => match fs::write(path, r) {
            Ok(_) => true,
            Err(_) => {println!("Failed to write to file");
                false}
        }
        Err(e) => panic!("Failed to serialize struct: {e}")
    }
}
pub fn load_list(list_name: String) -> TodoList {
    let path = format!("lists/{}.json", list_name);
    let data = match fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => panic!("File could not be open: {}", e)
    };
    match serde_json::de::from_str(&data) {
        Ok(r) => r,
        Err(e) => panic!("Deserialization failed: {}", e)
    }
}

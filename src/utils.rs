#![allow(dead_code)]
use crate::TodoList;
use std::fs;

pub fn save_list(list: &mut TodoList) -> bool {
//! Saves the list into a json file
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

pub fn load_list(list_path: &String) -> Option<TodoList> {
//! Loads the list at the provided path
    let data = match fs::read_to_string(list_path) {
        Ok(r) => r,
        Err(e) => {
            println!("File could not be open: {}", e);
            return None
        }
    };
    match serde_json::de::from_str(&data) {
        Ok(r) => Some(r),
        Err(e) => {
            println!("Deserialization failed: {}", e);
            None
        }
    }
}

pub fn find_lists() -> Vec<String> {
//! Reads the lists directory and retreive the path of all the lists
    let path = fs::read_dir("lists/").unwrap();
    let mut path_list: Vec<String> = Vec::new();
    for i in path {
        let cur_path = i.unwrap().path().display().to_string();
        if is_file_valid(&cur_path) {
            path_list.push(cur_path);
        }
    }
    path_list
}

fn is_file_valid(file_path: &String) -> bool {
//! Helper function to verify if the provided path is for a valid list
    load_list(file_path).is_some()
}

pub fn print_path_list(path_list: Vec<String>) {
//! Debug function to print all the lists
    for i in path_list.iter() {
        println!("{}", i);
    }
}

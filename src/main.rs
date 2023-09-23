#![allow(dead_code)]
use rust_todo::*;
use console::{self, Term};

mod utils;
use utils::*;

fn main() {
    let terminal = Term::stdout();
    main_menu_loop(&terminal);
}

fn get_lists(term: &Term) -> Vec<String> {
    term.clear_screen().unwrap();
    utils::find_lists()
}

fn main_menu_loop(term: &Term) {
    loop {
        let lists = get_lists(&term);
        term.clear_screen().unwrap();
        print_list_selection(&term, &lists);
        let quit = display_main_menu(&term, &lists);
        if quit { break };
    }
}

fn list_menu_loop(term: &Term, mut list: TodoList) {
    loop {
        term.clear_screen().unwrap();
        print_todo_list(&term, &list);
        let quit = display_list_menu(&term, &mut list);
        if quit { break }
    }
    save_list(&mut list);
}

fn print_list_selection(term: &Term, list: &Vec<String>) {
    for (i, value) in list.iter().enumerate() {
        let line = format!("{}. {}", i+1, value);
        term.write_line(&line).unwrap();
    }
}

fn print_todo_list(term: &Term, todo: &TodoList) {
    term.write_line(&todo.list_name).unwrap();
    for (i, value) in todo.todo_list.iter().enumerate() {
        let line = format!("{:02}. {}", i+1, value);
        term.write_line(&line).unwrap();
    }
}

fn display_number_menu(term: &Term) -> usize {
    term.write_line("Input desired list number: ").unwrap();
    let choice = term.read_line()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap_or(0);
    term.clear_last_lines(1).unwrap();
    choice as usize
}

fn display_main_menu(term: &Term, lists: &Vec<String>) -> bool {
    term.write_line("[O] open list | [N] new list | [D] delete list | [Q] quit").unwrap();
    let choice = term.read_char().unwrap();
    match choice {
        'o' | 'O' => {open_list(&term, &lists); false},
        'n' | 'N' => {new_list(&term); false}, 
        'd' | 'D' => {delete_selected_list(&term, &lists); false},
        'q' | 'Q' => true,
        _ => false
    }
}

fn open_list(term: &Term, lists: &Vec<String>) {
    let choice = display_number_menu(&term);
    let item: &String = match lists.get(choice-1) {
        Some(r) => r,
        None => return
    };
    let list = match load_list(item) {
        Some(r) => r,
        None => return
    };
    list_menu_loop(&term, list);
}

fn display_list_menu(term: &Term, list: &mut TodoList) -> bool {
    term.write_line("[N] new entry | [M] mark as done | [U] mark as not done| [D] delete entry | [R] rename list | [Q] quit").unwrap();
    let choice = term.read_char().unwrap();
    match choice {
        'n' | 'N' => {new_entry(&term, list); false},
        'm' | 'M' => {mark_entry_as_done(&term, list); false},
        'u' | 'U' => {mark_entry_as_not_done(&term, list); false},
        'd' | 'D' => {delete_entry(&term, list); false},
        'r' | 'R' => {rename_list(&term, list); false},
        'q' | 'Q' => true,
        _ => false
    }
}

fn new_entry(term: &Term, list: &mut TodoList) {
    term.write_line("Enter new item description: ").unwrap();
    let desc = term.read_line().unwrap();
    list.add(TodoEntry::new(desc));
}

fn mark_entry_as_done(term: &Term, list: &mut TodoList) {
    let choice = display_number_menu(&term) - 1;
    list.todo_list.get_mut(choice).unwrap().mark_as_done();
}

fn mark_entry_as_not_done(term: &Term, list: &mut TodoList) {
    let choice = display_number_menu(&term) - 1;
    list.todo_list.get_mut(choice).unwrap().mark_as_not_done();
}

fn delete_entry(term: &Term, list: &mut TodoList) {
    let choice = display_number_menu(&term) - 1;
    list.remove(choice);
}

fn rename_list(term: &Term, list: &mut TodoList) {
    term.write_line("Enter new name: ").unwrap();
    let old_name = list.list_name.clone();
    let name = term.read_line().unwrap();
    list.rename(name);
    delete_list_by_name(&old_name);
}

fn new_list(term: &Term) {
    term.write_line("Enter name of new list: ").unwrap();
    let name = term.read_line().unwrap();
    let mut list = TodoList::new(name);
    save_list(&mut list);
}

fn delete_selected_list(term: &Term, lists: &Vec<String>) {
    let choice = display_number_menu(&term);
    let path_to_list = match lists.get(choice-1) {
        Some(r) => r,
        None => return
    };
    let line = format!("Are you sure you want to delete {} ? [Y/N]", &path_to_list);
    term.write_line(&line).unwrap();
    match term.read_char().unwrap() {
        'y' | 'Y' => (),
        _ => return
    }
    delete_list_by_path(path_to_list);
}

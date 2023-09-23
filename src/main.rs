#![allow(dead_code)]
use rust_todo::*;
use console::{self, Term};
use utils::{load_list, save_list};
mod utils;

fn main() {
    let terminal = Term::stdout();
    let list_of_list = start(&terminal);
    main_menu_loop(&terminal, &list_of_list);
}

fn start(term: &Term) -> Vec<String> {
    term.clear_screen().unwrap();
    utils::find_lists()
}

fn main_menu_loop(term: &Term, lists: &Vec<String>) {
    loop {
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
        'n' | 'N' => {false}, 
        'd' | 'D' => {false},
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
    term.write_line("[N] new entry | [M] mark as done | [U] mark as not done| [D] delete entry | [Q] quit").unwrap();
    let choice = term.read_char().unwrap();
    match choice {
        'n' | 'N' => {new_entry(&term, list); false},
        'm' | 'M' => {mark_entry_as_done(&term, list); false},
        'u' | 'U' => {mark_entry_as_not_done(&term, list); false},
        'd' | 'D' => {delete_entry(&term, list); false},
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
    let name = term.read_line().unwrap();
    list.rename(name);
}

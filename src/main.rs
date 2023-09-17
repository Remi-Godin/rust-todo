use console::Term;
use std::time::SystemTime;

fn main() {
    let mut todo_list: Vec<TodoEntry> = Vec::new();
    let term = Term::stdout();
    loop {
        let _ = term.clone().clear_screen();
        print_entries(&todo_list, 0);
        todo_list.push(create_entry(term.clone()));
        manage_entries(term.clone());
    }
}

fn print_entries(entry_list: &Vec<TodoEntry>, count: i32) {
    for i in entry_list.iter() {
        println!("{}", i);
    }
}

fn create_entry(term: Term) -> TodoEntry {
    let _ = term.write_line("Enter task description...");
    let desc = term.read_line();
    let _ = term.clear_last_lines(2);
    let new_entry = TodoEntry {
        complete: false,
        description: desc.unwrap_or_default(),
        timestamp: SystemTime::now(),
    };
    return new_entry;
}

fn manage_entries(term: Term) {
    loop {
        let input = term.read_char().unwrap_or_default();
        let _ = match input {
            'h' => term.move_cursor_left(1),
            'j' => term.move_cursor_down(1),
            'k' => term.move_cursor_up(1),
            'l' => term.move_cursor_right(1),
            _ => break,
        };
    }
}

struct TodoEntry {
    complete: bool,
    description: String,
    timestamp: SystemTime,
}

impl std::fmt::Display for TodoEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut flag = "[ ]";
        if self.complete == true {
            flag = "[X]";
        }
        write!(f, "{:?} {} {}", self.timestamp, flag, self.description)
    }
}

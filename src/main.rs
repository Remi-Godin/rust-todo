use console::Term;
use std::time::SystemTime;

fn main() {
    let mut todo_list: Vec<TodoEntry> = Vec::new();
    let term = Term::stdout();
    loop {
        let _ = term.clone().clear_screen();
        print_entries(&todo_list);
        main_menu(&term, &mut todo_list)
    }
}

fn main_menu(term: &Term, todo_list: &mut Vec<TodoEntry>) {
    let _ = term.write_line("(N)ew entry, (C)omplete entries");
    let _ = match term.read_key().unwrap() {
        console::Key::Char('n') => todo_list.push(create_entry(term.clone()).unwrap()),
        console::Key::Char('c') => {complete_entries(todo_list, &select_entries(term.clone()));
            return}
        _ => println!("Unknown command")
    };

}

fn print_entries(entry_list: &Vec<TodoEntry>) -> &Vec<TodoEntry>{
    for (i, j) in entry_list.iter().enumerate() {
        println!("{:02}. {}", i, j);
    }
    entry_list
}

fn create_entry(term: Term) -> Option<TodoEntry> {
    let _ = term.write_line("Enter task description, or press [enter] to exit...");
    let desc = match term.read_line() {
        Err(e) => panic!("An error has occured when reading user input: {}", e),
        Ok(r) => r
    };
    if desc == "" {
        return None
    }
    let _ = term.clear_last_lines(2);
    let new_entry = TodoEntry {
        complete: false,
        description: desc,
        _timestamp: SystemTime::now(),
        _completed_timestamp: SystemTime::now()
    };
    return Some(new_entry);
}

fn select_entries(term: Term) -> Vec<usize>{
    let _ = term.write_line("Enter the number of each tasks you want to act on, separated by spaces...");
    let args = match term.read_line() {
        Err(e) => panic!("An error as occured when reading user input: {}", e),
        Ok(r) => r
    };
    let mut args_vec: Vec<usize> = Vec::new();
    for i in args.split(" ") {
        args_vec.push(i.trim().parse::<usize>().unwrap_or_default());
    }
    args_vec
}

fn complete_entries(todo_list: &mut Vec<TodoEntry>, entries: &Vec<usize>) {
    for i in entries.iter() {
        todo_list.get_mut(*i).unwrap().complete();
    }
    print_entries(&todo_list);
}

fn _manage_entries(term: Term) {
    loop {
        let input = term.read_char().unwrap_or_default();
        let _ = match input {
            'j' => term.move_cursor_down(1),
            'k' => term.move_cursor_up(1),
            _ => break,
        };
    }
}

struct TodoEntry {
    complete: bool,
    description: String,
    _timestamp: SystemTime,
    _completed_timestamp: SystemTime,
}

impl std::fmt::Display for TodoEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut flag = "[ ]";
        if self.complete == true {
            flag = "[X]";
        }
        write!(f, "{} {}", flag, self.description)
    }
}

impl TodoEntry {
    fn complete(&mut self) {
        self.complete = true;
    }
}

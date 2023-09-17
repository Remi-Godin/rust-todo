use console::Term;

fn main() {
    let mut todo_list: Vec<TodoEntry> = Vec::with_capacity(10);
    let term = Term::stdout();
    loop {
        term.clone().clear_screen();
        print_entries(&todo_list, 0);
        todo_list.push(create_entry(term.clone()));
    }
}

fn print_entries(entry_list: &Vec<TodoEntry>, count: i32) {
    for i in entry_list.iter() {
        println!("{}", i);
    }
}

fn create_entry(term: Term) -> TodoEntry {
    term.write_line("Enter task description...");
    let desc = term.read_line();
    term.clear_last_lines(2);
    let new_entry = TodoEntry {
        complete: false,
        description: desc.unwrap_or_default()
    };
    return new_entry
}

impl std::fmt::Display for TodoEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut flag = "[ ]";
        if self.complete == true{
            flag = "[X]";
        }
        write!(f, "{} {}", flag, self.description)
    }
}
struct TodoEntry {
    complete: bool,
    description: String
}

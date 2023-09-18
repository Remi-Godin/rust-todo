use console::Term;

fn main() {
    let mut todo_list: Vec<TodoEntry> = Vec::new();
    let term = Term::stdout();
    loop {
        let _ = term.clone().clear_screen();
        print_entries(&todo_list);
        let quit = main_menu(&term, &mut todo_list);
        if quit {
            break
        }
    }
}

fn main_menu(term: &Term, todo_list: &mut Vec<TodoEntry>) -> bool {
    let _ = term.write_line("(N)ew entry|(M)ark as done|(U)nmark as done|(D)elete entries|(Q)uit");
    let _ = match term.read_key().unwrap() {
        console::Key::Char('n') => match create_entry(term.clone()) {
            Some(r) => {todo_list.push(r);
                return false}
            None => {print!("");
                return false}
        }
        console::Key::Char('m') => {
            let _ = term.clear_last_lines(1);
            mark_as_done(todo_list, &select_entries(term.clone()));
            return false},
        console::Key::Char('u') => {
            let _ = term.clear_last_lines(1);
            mark_as_not_done(todo_list, &select_entries(term.clone()));
            return false},
        console::Key::Char('d') => {
            let _ = term.clear_last_lines(1);
            delete_entries(todo_list, &select_entries(term.clone()));
            return false},
        console::Key::Char('q') => return true,
        _ => return false
    };

}

fn print_entries(todo_list: &Vec<TodoEntry>) {
    for (i, j) in todo_list.iter().enumerate() {
        println!("{:02}. {}", i, j);
    }
}

fn create_entry(term: Term) -> Option<TodoEntry> {
    let _ = term.clear_last_lines(1);
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
    };
    return Some(new_entry);
}

fn delete_entries(todo_list: &mut Vec<TodoEntry>, entries: &Vec<usize>) {
    for i in entries.iter().rev() {
        if i < &todo_list.len(){
            todo_list.remove(*i);
        }
    }
    print_entries(&todo_list);
}

fn select_entries(term: Term) -> Vec<usize>{
    let _ = term.clear_last_lines(1);
    let _ = term.write_line("Enter the number of each tasks you want to act on, separated by spaces...");
    let args = match term.read_line() {
        Err(e) => panic!("An error as occured when reading user input: {}", e),
        Ok(r) => r
    };
    let mut args_vec: Vec<usize> = Vec::new();
    for i in args.split(" ") {
        if i.parse::<usize>().is_ok() {
            args_vec.push(i.parse::<usize>().unwrap());
        }
    }
    args_vec.sort();
    args_vec
}

fn mark_as_done(todo_list: &mut Vec<TodoEntry>, entries: &Vec<usize>) {
    for i in entries.iter() {
        if i < &todo_list.len(){
            todo_list.get_mut(*i).unwrap().complete();
        }
    }
    print_entries(&todo_list);
}

fn mark_as_not_done(todo_list: &mut Vec<TodoEntry>, entries: &Vec<usize>) {
    for i in entries.iter() {
        if i < &todo_list.len(){
            todo_list.get_mut(*i).unwrap().incomplete();
        }
    }
    print_entries(&todo_list);
}

struct TodoEntry {
    complete: bool,
    description: String,
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

    fn incomplete(&mut self) {
        self.complete = false;
    }
}

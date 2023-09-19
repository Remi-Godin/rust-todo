use chrono::{Utc, DateTime};

fn main() {
}

fn test_1() {
    let test = TodoEntry {
        done: false,
        description: String::from("Testing"),
        creation_timestamp: chrono::offset::Utc::now(),
        _completed_timestamp: chrono::offset::Utc::now()

    };
    println!("{}", test);
}

struct TodoEntry {
    done: bool,
    description: String,
    creation_timestamp: DateTime<Utc>,
    _completed_timestamp: DateTime<Utc>
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
        if self.done == true {
            checkbox = "[X]";
        };
        write!(f, "{} {}", checkbox, self.description)
    }
}


struct TodoList {
    todo_list: Vec<TodoEntry>
}

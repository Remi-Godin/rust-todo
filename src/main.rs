mod data;

fn main() {
    let my_list = _test_2();
    my_list.print_list();
}


fn _test_1() {
    let test = data::TodoEntry {
        done: false,
        description: String::from("Testing"),
    };
    let my_todo_list: Vec<data::TodoEntry> = vec![test];
    let mut todo_list = data::TodoList {
        list_name: String::from("Testing"),
        todo_list: my_todo_list,
    };
    data::save_list(&mut todo_list);
}

fn _test_2() -> data::TodoList {
    data::load_list(String::from("Testing"))
}

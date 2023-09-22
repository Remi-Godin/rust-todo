mod data;

fn main() {
    let my_list = _test_2();
    my_list.print_list();
}


fn _test_1() {
    let test_entry = data::TodoEntry::new(String::from("Testing"));
    let mut todo_list = data::TodoList::new(String::from("List_01"));
    todo_list.add(test_entry);
    data::save_list(&mut todo_list);
}

fn _test_2() -> data::TodoList {
    data::load_list(String::from("Testing"))
}

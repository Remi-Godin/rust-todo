mod data;

fn main() {
    test_1();
}


fn test_1() {
    let test = data::TodoEntry {
        done: false,
        description: String::from("Testing"),
    };
    let mut my_todo_list: Vec<data::TodoEntry> = Vec::new();
    my_todo_list.push(test);
    let todo_list = data::TodoList {
        list_name: String::from("Testing"),
        file_location: String::from("UNUSUED"),
        todo_list: my_todo_list
    };
    todo_list.save_list();

    
}

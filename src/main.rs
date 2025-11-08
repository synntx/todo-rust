use std::io::{self};

const COMMAND_ADD: &str = "add";
const COMMAND_DONE: &str = "done";
const COMMAND_LIST: &str = "list";
const COMMAND_QUIT: &str = "quit";

#[derive(Debug)]
struct Todo {
    id: i32,
    title: String,
    is_done: bool,
}

fn main() {
    // Functionalities:
    // 1. Add TODO_NAME
    // 2. Done TODO_ID
    // 3. LIST

    // let mut args: Vec<String> = env::args().collect();
    // let mut args: String = Vec::new();

    let mut todos: Vec<Todo> = Vec::new();
    let mut id: i32 = 0;

    loop {
        let mut args = String::new();

        io::stdin()
            .read_line(&mut args)
            .expect("failed to read line");

        let trimmed_input = args.trim();

        let parts: Vec<&str> = trimmed_input.splitn(2, ' ').collect();

        // let command = &parts[1];
        // let todo_id_or_name = parts.get(1);

        match parts.first() {
            Some(&COMMAND_ADD) => {
                // Handle Add Command
                // add todo_title
                if let Some(item_to_add) = parts.get(1) {
                    println!("Adding Item: {}", item_to_add);
                    id += 1;
                    todos.push(Todo {
                        id: id,
                        title: item_to_add.to_string(),
                        is_done: false,
                    });
                } else {
                    println!("'{}' command requires an item to add.", COMMAND_ADD);
                }
            }
            Some(&COMMAND_LIST) => {
                // Handle Listing Todos
                println!("Here is the Complete list of todos: ");
                for todo in &todos {
                    println!("Id: {}", todo.id);
                    println!("Title: {}", todo.title);
                    println!("Is Done: {}", todo.is_done)
                }
            }
            Some(&COMMAND_DONE) => {
                // Handle Todo Status
                // done todo_id
                if let Some(todo_id) = parts.get(1) {
                    // todos.push(Todo {
                    //     id: 1,
                    //     title: item_to_add,
                    //     is_done: false,
                    // });

                    // Task: find the todo with the givven id
                    // todos is a growable array (Vec)
                    for todo in &mut todos {
                        if todo.id == todo_id.parse().expect("Failed to parse string as i32") {
                            todo.is_done = true;
                            println!("Making todo with id({}) done", todo.id);
                        }
                    }
                } else {
                    println!("'{}' command requires an item to add.", COMMAND_ADD);
                }
            }
            Some(&COMMAND_QUIT) => {
                // Exit the prorgram
                break;
            }
            _ => {
                println!("Unknown Command: {}", trimmed_input)
            }
        }
    }

    // Add
    // if command.to_lowercase() == COMMAND_ADD {
    //     let todo = Todo {
    //         id: 1,
    //         title: todo_id_or_name,
    //         is_done: false,
    //     };

    //     todos.push(todo);
    // }

    // List
    // if command.to_lowercase() == COMMAND_LIST {
    //     // Show something like that:
    //     // 1. [ ] Buy milk
    //     for todo in &todos {
    //         println!("{}", todo.id);
    //         println!("{}", if todo.is_done { "✅" } else { "[ ]" });
    //         println!("{}", todo.title)
    //     }
    // }
}

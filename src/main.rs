use serde::{Deserialize, Serialize};
use std::fs::File;
use std::i32;
use std::io::{self, BufReader, BufWriter, ErrorKind};

const COMMAND_ADD: &str = "add";
const COMMAND_DONE: &str = "done";
const COMMAND_LIST: &str = "list";
const COMMAND_QUIT: &str = "quit";

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: i32,
    title: String,
    is_done: bool,
}

fn save_todos(todos: &Vec<Todo>) {
    // open file
    // write file
    // handle errors if any

    let file_result = File::create("todos.json");
    match file_result {
        Ok(file) => {
            let writer = BufWriter::new(file);
            match serde_json::to_writer(writer, &todos) {
                Ok(_ok) => println!("||  written!!!  ||"),
                Err(_) => println!("||  not written in file  ||"),
            }
        }
        Err(_e) => println!("Some Error occured while creating the file"),
    }
}

fn main() {
    // Functionalities:
    // 1. Add TODO_NAME
    // 2. Done TODO_ID
    // 3. LIST

    let mut todos: Vec<Todo> = Vec::new();
    let mut id: i32 = 0;

    let todos_file = File::open("todos.json");

    match todos_file {
        Ok(file) => {
            println!("File opened successfully");
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(list) => todos = list,
                Err(_e) => {
                    println!("error occured while deserializing the json file");
                    todos = Vec::new();
                }
            }
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("file not found");
                match File::create("todos.json") {
                    Ok(_new_file) => println!("New File Craeted"),
                    Err(err) => println!("Some Error Occured: {}", err),
                }
            }
            other_error => {
                println!("other error: {}", other_error)
            }
        },
    }

    loop {
        let mut args = String::new();

        io::stdin()
            .read_line(&mut args)
            .expect("failed to read line");

        let trimmed_input = args.trim();

        let parts: Vec<&str> = trimmed_input.splitn(2, ' ').collect();

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
                    save_todos(&todos);
                } else {
                    println!("'{}' command requires an item to add.", COMMAND_ADD);
                }
            }
            Some(&COMMAND_LIST) => {
                // Handle Listing Todos
                // list
                println!("Here is the Complete list of todos: ");
                for todo in &todos {
                    println!("Id: {}", todo.id);
                    println!("Title: {}", todo.title);
                    println!("Is Done: {}", todo.is_done)
                }
                save_todos(&todos);
            }
            Some(&COMMAND_DONE) => {
                // Handle Todo Status
                // done todo_id

                if let Some(todo_id) = parts.get(1) {
                    let parsed_id = match todo_id.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => {
                            continue;
                        }
                    };
                    for todo in &mut todos {
                        if todo.id == parsed_id {
                            todo.is_done = true;
                            println!("Making todo with id({}) done", todo.id);
                        }
                    }
                    save_todos(&todos);
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
}

/*todo list using vec. traits, methods */
use std::io::{self, Write};

struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: String) {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };

        self.todos.push(todo);
        self.next_id += 1;

        println!("Todo added.");
    }

    fn complete(&mut self, id: usize) {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.completed = true;
                println!("Todo {} completed.", id);
                return;
            }
        }

        println!("Todo not found.");
    }

    fn delete(&mut self, id: usize) {
        let original_length = self.todos.len();

        self.todos.retain(|todo| todo.id != id);

        if self.todos.len() < original_length {
            println!("Todo {} deleted.", id);
        } else {
            println!("Todo not found.");
        }
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("No todos.");
            return;
        }

        for todo in &self.todos {
            let status = if todo.completed {
                "✓"
            } else {
                " "
            };

            println!("[{}] {}: {}", status, todo.id, todo.title);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print!("> "); //writter buffered input immediately
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();

        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let argument = parts.next().unwrap_or("");

        match command {
            "add" => {
                if argument.is_empty() {
                    println!("Usage: add <todo>");
                } else {
                    todo_list.add(argument.to_string());
                }
            }

            "complete" => {
                match argument.parse::<usize>() {
                    Ok(id) => todo_list.complete(id),
                    Err(_) => println!("Usage: complete <id>"),
                }
            }

            "delete" => {
                match argument.parse::<usize>() {
                    Ok(id) => todo_list.delete(id),
                    Err(_) => println!("Usage: delete <id>"),
                }
            }

            "list" => {
                todo_list.list();
            }

            "quit" => {
                println!("Goodbye!");
                break;
            }

            "" => {}

            _ => {
                println!("Unknown command.");
                println!("Commands: add, complete, delete, list, quit");
            }
        }
    }
}
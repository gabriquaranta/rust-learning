extern crate colored;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Todo {
    task: String,
    done: bool,
}

impl Todo {
    fn new(text: &str) -> Self {
        Todo {
            task: text.to_string(),
            done: false,
        }
    }

    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.done {
            return write!(f, "{}", self.task.green());
        } else {
            return write!(f, "{}", self.task);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { list: Vec::new() }
    }

    fn add(&mut self, text: &str) {
        self.list.push(Todo::new(text))
    }

    fn remove(&mut self, index: usize) {
        if index < self.list.len() {
            let removed = self.list.remove(index);
            println!("Removed: {}", removed.task);
        } else {
            println!("Invalid Index.");
        }
    }

    fn toggle(&mut self, index: usize) {
        if index < self.list.len() {
            self.list[index].toggle()
        } else {
            println!("Invalid Index.")
        }
    }

    fn save_todo(&self, path: &PathBuf) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let json_data = serde_json::to_string(&self.list)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    fn load_todo(&mut self, path: &PathBuf) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        self.list = serde_json::from_str(&json_data)?;
        Ok(())
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = String::from("TODOs:\n").red();
        let _ = write!(f, "{}", title);
        for (index, todo) in self.list.iter().enumerate() {
            if todo.done {
                let _ = write!(f, "\n {}. {}", index, todo.task.green());
            } else {
                let _ = write!(f, "\n {}. {}", index, todo.task);
            }
        }
        return write!(f, "");
    }
}

fn get_terminal_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read command");
    return input;
}

fn main() {
    let mut todolist = TodoList::new();
    let file_path = PathBuf::from("todos/todos.json");

    print!("\x1B[2J\x1B[1;1H"); // clear the screen and put the cursor at first row & first col of the screen.
    println!("{}", String::from(" - TODO APP - ").blue().bold());

    // new or load
    loop {
        println!("\nEnter a command:\n - load (l)\n - new (n) !will overwrite!");
        let command = get_terminal_input();

        match command.trim().to_lowercase().as_str() {
            "load" | "l" => {
                if let Err(e) = todolist.load_todo(&file_path) {
                    println!("Failed to load todos: {}", e);
                } else {
                    println!("Todos loaded from file");
                }
                break;
            }
            "new" | "n" => {
                break;
            }
            _ => {
                println!("not valid")
            }
        }
    }

    // use
    loop {
        print!("\x1B[2J\x1B[1;1H");
        // clear the screen and put the cursor at first row & first col of the terminal

        println!("{}", String::from(" - TODO APP - ").blue().bold());
        println!("\n{}", todolist);

        // input
        println!("\nEnter a command:\n - add (a)\n - remove (r)\n - toggle (t)\n - exit+save (e)");
        let command = get_terminal_input();

        // match
        match command.trim().to_lowercase().as_str() {
            "add" | "a" => {
                println!("Enter a task:");
                let task = get_terminal_input();
                todolist.add(task.trim());
            }
            "remove" | "r" => {
                println!("Enter the index:");
                let index = get_terminal_input();
                todolist.remove(index.trim().parse().unwrap_or(usize::MAX));
            }
            "toggle" | "t" => {
                println!("Enter the index:");
                let index = get_terminal_input();
                todolist.toggle(index.trim().parse().unwrap_or(usize::MAX));
            }
            "exit" | "e" => {
                if let Err(e) = todolist.save_todo(&file_path) {
                    println!("Failed to save todos: {}", e);
                } else {
                    println!("Todos saved to file");
                }
                print!("\x1B[2J\x1B[1;1H");
                break;
            }
            _ => {
                println!("Not a command.")
            }
        }
    }
}

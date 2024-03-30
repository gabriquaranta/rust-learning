extern crate colored;

use chrono::prelude::*;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Todo {
    task: String,
    done: bool,
    last_mod_time: String,
    priority: usize,
}

impl Todo {
    fn new(text: &str) -> Self {
        Todo {
            task: text.to_string(),
            done: false,
            last_mod_time: Local::now().to_string(),
            priority: 0,
        }
    }

    fn toggle(&mut self) {
        self.done = !self.done;
        self.last_mod_time = Local::now().to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { list: Vec::new() }
    }

    pub fn add(&mut self, text: &str) {
        self.list.push(Todo::new(text))
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.list.len() {
            let removed = self.list.remove(index);
            println!("Removed: {}", removed.task);
        } else {
            println!("Invalid Index.");
        }
    }

    pub fn toggle(&mut self, index: usize) {
        if index < self.list.len() {
            self.list[index].toggle()
        } else {
            println!("Invalid Index.")
        }
    }

    pub fn save_todo(&self, path: &PathBuf) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let json_data = serde_json::to_string(&self.list)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    pub fn load_todo(&mut self, path: &PathBuf) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        self.list = serde_json::from_str(&json_data)?;
        Ok(())
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = String::from("TODOs:\n").bold().red();
        let _ = write!(f, "{}", title);
        for (index, todo) in self.list.iter().enumerate() {
            if todo.done {
                let _ = write!(
                    f,
                    "\n {}. P{}: {}  - {}",
                    index,
                    todo.priority.to_string().magenta(),
                    todo.task.bold().green().strikethrough(),
                    todo.last_mod_time.cyan(),
                );
            } else {
                let _ = write!(
                    f,
                    "\n {}. P{}: {}  - {}",
                    index,
                    todo.priority.to_string().magenta(),
                    todo.task.bold(),
                    todo.last_mod_time.cyan(),
                );
            }
        }
        return write!(f, "");
    }
}

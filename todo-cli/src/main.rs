extern crate colored;

use colored::Colorize;
use std::fmt;
use std::io;

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

fn main() {
    let mut todolist = TodoList::new();

    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear the screen and put the cursor at first row & first col of the screen.
        println!("{}", String::from(" - TODO APP - ").blue().bold());
        println!("\n{}", todolist);

        // input
        println!("\nEnter a command (add/remove/toggle/show/exit):");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        // match
        match command.trim().to_lowercase().as_str() {
            "add" => {
                println!("Enter a task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to add");
                todolist.add(task.trim());
            }
            "remove" => {
                println!("Enter the index:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to remove");
                todolist.remove(index.trim().parse().unwrap_or(usize::MAX));
            }
            "toggle" => {
                println!("Enter the index:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to remove");
                todolist.toggle(index.trim().parse().unwrap_or(usize::MAX));
            }
            "exit" => {
                print!("\x1B[2J\x1B[1;1H");
                break;
            }
            _ => {
                println!("Not a command.")
            }
        }
    }
}

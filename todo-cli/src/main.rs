mod todolist;
use crate::todolist::TodoList;

extern crate colored;

use colored::Colorize;
use std::io;
use std::path::PathBuf;

fn get_terminal_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read command");
    return input;
}

fn main() {
    let mut todolist = TodoList::new();
    let file_path = PathBuf::from("src/todos/todos.json");

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

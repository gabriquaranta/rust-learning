use rand::Rng;
use std::io;

fn roll_dice(faces: i32) -> i32 {
    let roll = rand::thread_rng().gen_range(0..faces);
    return roll;
}

fn main() {
    println!("Hello, world!");

    // dice choice
    let mut input = String::new();
    println! {"\nWhich dice to roll?"};
    let _ = io::stdin().read_line(&mut input);

    let mut roll = 0;

    // parse dice
    match input.trim().parse::<i32>() {
        Ok(parsed_int) => {
            println!("Rolling  d{}", parsed_int);
            roll = roll_dice(parsed_int);
            println!("Result: {}", roll);
        }
        Err(e) => {
            println!("Error parsing dice faces: {}", e);
        }
    }

    // modifier
    input = String::new();
    println! {"\nWith what modifier?"};
    let _ = io::stdin().read_line(&mut input);

    // parse modifier
    match input.trim().parse::<i32>() {
        Ok(parsed_int) => {
            println!("\n\nYOUR ROLL: {}", parsed_int + roll);
        }
        Err(e) => {
            println!("Error parsing modifier: {}", e);
        }
    }
}

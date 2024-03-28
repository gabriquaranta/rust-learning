use rand::Rng;
use std::io;

fn random_word() -> &'static str {
    let dictionary: Vec<&'static str> = vec![
        "apple",
        "banana",
        "orange",
        "grape",
        "strawberry",
        "pineapple",
        "watermelon",
        "blueberry",
        "kiwi",
        "peach",
        "apricot",
        "blackberry",
        "cherry",
        "coconut",
        "fig",
        "lemon",
        "lime",
        "mango",
        "papaya",
        "pear",
        "raspberry",
        "plum",
        "grapefruit",
        "melon",
        "cantaloupe",
        "nectarine",
        "tangerine",
        "passionfruit",
        "lychee",
        "guava",
        "persimmon",
        "quince",
        "date",
        "starfruit",
        "mulberry",
        "avocado",
        "pomegranate",
        "durian",
        "dragonfruit",
        "kiwifruit",
        "jackfruit",
        "boysenberry",
        "cranberry",
        "elderberry",
        "gooseberry",
        "rhubarb",
        "soursop",
    ];

    let index = rand::thread_rng().gen_range(0..dictionary.len());

    return dictionary[index];
}

fn main() {
    println!("Hello Hangman!");

    let word = random_word();
    let chars_vec: Vec<char> = word.chars().collect();
    let word_len = word.chars().count();

    let mut guess = vec!['_'; word_len];
    let mut counter = 0;

    println!("{}", word);

    loop {
        println!("\n\n{:?}\nWrong Guesses: {}", guess, counter);

        // input
        let mut input = String::new();
        println! {"Enter a letter:"};
        let _ = io::stdin().read_line(&mut input);
        let input = input.trim();
        let input_char = input.chars().next().unwrap();

        // check if input is in word
        let mut found = false;
        for (i, c) in chars_vec.iter().enumerate() {
            if c == &input_char {
                guess[i] = input_char;
                found = true;
            }
        }

        // update counter if not
        if found == false {
            println!("Wrong Guess!");
            counter = counter + 1;
        }

        //check counter
        if counter >= 6 {
            println!("\nLOSS! Words was: {}", word);
            return;
        }

        // check if full and exit
        let mid: String = guess.iter().collect();
        if mid.eq(word) {
            println!("\nWIN! Words was: {}", word);
            return;
        }
    }
}

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Missing file path argument!");
        return;
    }

    let file_name = &args[1];
    let content = match read_to_string(file_name) {
        Ok(content) => content,
        Err(why) => {
            println!("Error reading file '{}': {}", file_name, why);
            return;
        }
    };

    // Initialize counters for lines, words, and characters
    let mut lines = 0;
    let mut words = 0;
    let mut characters = 0;

    for line in content.lines() {
        lines += 1;
        characters += line.len();
        words += line.split_whitespace().count();
    }

    println!(
        "File: {}\nLines: {}\nWords: {}\nCharacters: {}",
        file_name, lines, words, characters
    );
}

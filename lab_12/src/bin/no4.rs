use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let mut store_vec: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        println!("Input: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input!");

        if input.trim().is_empty() {
            break;
        }

        store_vec.push(input.trim().to_uppercase());
    }

    let mut data_file = File::create("data.txt").expect("File creation failed");

    for line in &store_vec {
        data_file
            .write_all(format!("{}\n", line).as_bytes())
            .expect("Write failed");
    }

    let mut data_file = File::open("data.txt").expect("File open failed");
    let mut contents = String::new();
    data_file
        .read_to_string(&mut contents)
        .expect("File read failed");

    for line in contents.lines() {
        println!("{}", line);
    }
}

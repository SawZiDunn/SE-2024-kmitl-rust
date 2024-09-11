use std::io;

struct Book {
    title: String,
    author: String,
    published_year: Option<u32>,
}

fn main() {
    // match divide(10, 0) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e) => println!("Error: {}", e),
    // }

    // ==========================

    // let s1 = "Hello";
    // let s2 = "World";
    // let result = concat_strings(s1, s2);
    // println!("{}", result); // HelloWorld

    // ==============================

    // let s = String::from("Hello, World!");
    // let result = get_substring(&s, 7, 12);
    // println!("{}", result); // World

    // ===============================

    // let s1 = "abc";

    // match parse_int(s1) {
    //     Ok(n) => println!("Parsed number: {}", n),
    //     Err(e) => println!("Failed to parse: {}", e),
    // }

    // =========================

    // let sentence = "The quick brown fox jumps over the lazy dog";
    // let longest = find_longest_word(sentence);
    // println!("The longest word is: {}", longest); // jumps

    // =================================

    // let result = calculate(3, 4);
    // println!("Sum: {}, Product: {}", result.0, result.1);

    // ==================================

    // let book = Book {
    //     title: "A".to_string(),
    //     author: "B".to_string(),
    //     published_year: None,
    // };
    // match book.published_year {
    //     Some(year) => println!("{} was published in {}", book.title, year),
    //     None => println!("{} has no publication year", book.title),
    // }

    // ===============
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{}", sum(&nums));
}

fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn find_longest_word(sentence: &str) -> &str {
    let chunks: Vec<&str> = sentence.split_whitespace().collect();

    if chunks.is_empty() {
        return "";
    }
    let mut longest_word = chunks[0];
    for word in &chunks[1..] {
        if longest_word.len() <= word.len() {
            longest_word = word;
        }
    }

    longest_word
}
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    // match s.parse() {
    //     Ok(n) => Ok(n),
    //     Err(err) => Err(err),
    // }

    s.parse()
    // s.parse<i32>()
}

fn get_substring(s: &String, x: usize, y: usize) -> &str {
    &s[x..y]
}

fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{s1}{s2}")
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("cannot be divided by 0".to_string())
    } else {
        Ok(x / y)
    }
}

fn sum(nums: &Vec<i32>) -> i32 {
    // let mut total: i32 = 0;
    // for i in nums {
    //     total += i;
    // }
    // total
    nums.iter().sum()
}

fn greet_optional(name: Option<&str>) {
    match name {
        Some(x) => println!("Hello {}", x),
        None => println!("Hello World"),
    }
}

fn get_day() {
    let days: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let mut input = String::new();
    println!("Enter the number of the day: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    let day_no: i32 = match input.trim().parse::<i32>() {
        // input.trim().parse()
        Ok(n) => n,
        Err(e) => {
            println!("{}\nmust be an integer!", e);
            return;
        }
    };

    let day = match day_no {
        1 => days[0],
        2 => days[1],
        3 => days[2],
        4 => days[3],
        5 => days[4],
        6 => days[5],
        7 => days[6],
        _ => "Invalid",
    };
    println!("{}", day);
}

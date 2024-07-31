use std::io;

fn pascal(row: u32, col: u32) -> usize {
    if col == 0 {
        1
    } else if col == row {
        1
    } else {
        pascal(row - 1, col - 1) + pascal(row - 1, col)
    }
}

fn print_pascal_row(n: u32, row: u32) {
    for _ in 0..(n - row - 1) {
        print!("  ") // two space
    }
    for j in 0..row + 1 {
        print!("{:>4}", pascal(row, j));
    }
    println!();
}

fn main() {
    let n: u32 = loop {
        println!("Enter number between 1 and 9 : ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse() {
            Ok(num) if (1..10).contains(&num) => break num,
            _ => println!("Your input isn't valid number between 1 and 9"),
        }
    };

    for row in 0..n {
        print_pascal_row(n, row); // the pascal row in single line
    }
}

use std::io;

// let max_length = std::cmp::max(player1.len(), player2.len()) + 14;

fn main() {
    let mut player_1 = String::new();
    let mut player_2 = String::new();

    println!("Enter Player 1 name: ");
    io::stdin()
        .read_line(&mut player_1)
        .expect("Failed to read input!");

    // trimming to remove any extra chars and converting back to string
    // since trim() converts a string to &str
    player_1 = player_1.trim().to_string();

    println!("Enter Player 2 name: ");
    io::stdin()
        .read_line(&mut player_2)
        .expect("Failed to read input!");

    player_2 = player_2.trim().to_string();

    let p1_length = player_1.len();
    let p2_length = player_2.len();
    let mut max_length = 0;
    let mut len_diff = 0;

    if p1_length > p2_length {
        max_length = p1_length;
        len_diff = p1_length - p2_length;
    } else {
        max_length = p2_length;
        len_diff = p2_length - p1_length;
    }

    // vertical format

    // count the output
    max_length += 12 + 2;
    let mut vertical_border = String::new();

    for _i in 0..max_length {
        vertical_border.push('*');
    }

    let vertical_extra_line = format!("*{}*", " ".repeat(max_length - 2));

    println!("{}", vertical_border);
    println!("{}", vertical_extra_line);
    println!("* Player 1: {} *", player_1);
    println!("{}", vertical_extra_line);
    println!("{}", vertical_border);

    println!("{}", vertical_extra_line);
    println!("* Player 2: {}{} *", player_2, " ".repeat(len_diff));
    println!("{}", vertical_extra_line);
    println!("{}", vertical_border);

    println!();

    //horizontal format

    let new_max_length = p1_length + p2_length + 2 + 10 + 3 + 10 + 2;
    let mut horizontal_border = String::new();
    for _i in 0..new_max_length {
        horizontal_border.push('*');
    }

    let horizontal_extra_line = format!(
        "*{}*{}*",
        " ".repeat(p1_length + 12),
        " ".repeat(p2_length + 12)
    );

    println!("{}", horizontal_border);
    println!("{}", horizontal_extra_line);
    println!("* Player 1: {} * Player 2: {} *", player_1, player_2);
    println!("{}", horizontal_extra_line);
    println!("{}", horizontal_border)
}

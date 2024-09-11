fn pascal_triangle_balanced(n: usize) -> Vec<String> {
    if n == 0 {
        return vec![];
    }

    // Recursive function to generate the triangle
    fn generate_row(current_row: usize, last_row: &Vec<u64>) -> Vec<u64> {
        if current_row == 0 {
            vec![1]
        } else {
            let mut row = vec![1];
            for i in 0..last_row.len() - 1 {
                row.push(last_row[i] + last_row[i + 1]);
            }
            row.push(1);
            row
        }
    }

    let mut triangle = vec![vec![1u64]];
    for i in 1..n {
        triangle.push(generate_row(i, &triangle[i - 1]));
    }

    // Format each row and center align it
    let mut formatted_rows = Vec::new();
    let max_width = triangle
        .last()
        .unwrap()
        .iter()
        .map(|num| num.to_string().len() + 1)
        .sum::<usize>();

    for row in triangle {
        let row_str: Vec<String> = row.iter().map(|num| num.to_string()).collect();
        let joined_row = row_str.join(" ");
        let padding = (max_width - joined_row.len()) / 2;
        let padded_row = format!("{}{}", " ".repeat(padding), joined_row);
        formatted_rows.push(padded_row);
    }

    formatted_rows
}

fn main() {
    let triangle = pascal_triangle_balanced(7);
    for row in triangle {
        println!("{}", row);
    }
}

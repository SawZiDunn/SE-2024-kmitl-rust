use std::io;

fn get_matrix() -> Vec<Vec<i32>> {
    let mut result_vec: Vec<Vec<i32>> = Vec::new();

    let mut input = String::new();
    println!("Enter rows: ");
    io::stdin().read_line(&mut input).expect("Failed..");
    let rows: i32 = input.trim().parse().expect("Failed");
    input.clear();

    println!("Enter columns: ");
    io::stdin().read_line(&mut input).expect("Failed..");
    let columns: i32 = input.trim().parse().expect("Failed");
    input.clear();

    for i in 1..rows {
        let mut temp: Vec<i32> = Vec::new();
        for j in 1..columns {
            println!("Enter number: ");
            io::stdin().read_line(&mut input).expect("Failed..");
            temp.push(input.trim().parse().expect("Failed.."));
        }
        result_vec.push(temp);
    }

    result_vec
}

fn multiply_matrix(matrix1: &[[i32; 3]; 2], matrix2: &[[i32; 3]; 3]) -> Vec<Vec<i32>> {
    let mut final_matrix: Vec<Vec<i32>> = Vec::new();

    for i in 0..matrix1.len() {
        let mut row: Vec<i32> = Vec::new();

        for j in 0..matrix2[0].len() {
            let mut total: i32 = 0;

            for k in 0..matrix1[0].len() {
                total += matrix1[i][k] * matrix2[k][j];
            }

            row.push(total);
        }
        final_matrix.push(row);
    }

    final_matrix
}

fn main() {
    let a: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    // let mut matrix1: Vec<[i32; 3]> = vec![[1, 4, 4], [2, 3, 2]];

    let b: [[i32; 3]; 3] = [[7, 8, 9], [10, 11, 12], [13, 14, 15]];
    // let mut matrix2: Vec<[i32; 3]> = vec![[1, 4, 4], [2, 3, 2], [1, 3, 5]];

    let final_matrix = multiply_matrix(&a, &b);

    for i in &final_matrix {
        println!("{:?}", i);
    }
}

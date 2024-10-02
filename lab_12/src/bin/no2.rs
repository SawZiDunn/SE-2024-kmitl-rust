use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        4 => match args[3].trim() {
            "+" | "-" | "/" | "*" => {
                operate(args[1].trim(), args[2].trim(), args[3].trim());
            }

            _ => {
                println!("Unsupported operators!")
            }
        },
        _ => {
            println!("Please provide only three arguments!");
        }
    }
}

fn operate(x: &str, y: &str, operator: &str) {
    let a = match x.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The first argument is not a number!");
            return;
        }
    };

    let b = match y.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The second argument is not a number!");
            return;
        }
    };

    match operator {
        "+" => {
            println!("{}", a + b)
        }
        "-" => {
            println!("{}", a - b)
        }
        "*" => {
            println!("{}", a * b)
        }
        "/" => {
            if b == 0.0 {
                println!("Cannot divide by 0!");
            } else {
                println!("{}", a / b)
            }
        }
        _ => {}
    }
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", &args);

    match args.len() {
        4 => {
            println!("Arguments provided: {:?}", &args[1..]);
        }
        _ => {
            println!("Please provide only three arguments!");
        }
    }
}

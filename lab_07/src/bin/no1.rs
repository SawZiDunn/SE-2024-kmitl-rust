fn factorial(x: u32) -> u32 {
    println!("Calculating factorial({})", x);
    println!("Value: {}, Memory Address: {:p}", x, &x);

    if x <= 0 {
        return 1;
    } else {
        x * factorial(x - 1)
    }
}

fn main() {
    let x = 5;
    let result = factorial(x);
    println!("Factorial result: {}", result);
}

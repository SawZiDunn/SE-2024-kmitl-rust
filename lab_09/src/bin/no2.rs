fn main() {
    let mut x: Vec<i32> = Vec::new();

    println!("length: {}, capacity: {}", x.len(), x.capacity());

    for i in 1..=5 {
        x.push(i);
    }

    println!("length: {}, capacity: {}", x.len(), x.capacity());

    let mut y: Vec<i32> = Vec::with_capacity(10);

    // original capacity is doubled every time the length exceeds the capacity by 1
    for i in 1..=21 {
        y.push(i);
    }

    println!("length: {}, capacity: {}", y.len(), y.capacity());
}

fn swap_elements<T, N>(a: Vec<T>, b: Vec<N>) -> (Vec<N>, Vec<T>) {
    (b, a)
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec!["four", "five", "six"];
    let result = swap_elements(vec1, vec2);
    println!("{:?} {:?}", result.0, result.1);
}

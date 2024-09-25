// Define a struct for our custom Fibonacci iterator
struct Fibonacci {
    current: u64,
    next: u64,
}

// Implement the Iterator trait for our Fibonacci struct
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_fib = self.current;

        // Calculate the next Fibonacci number
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;

        Some(current_fib)
    }
}

// Implement a constructor for the Fibonacci struct
impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

fn main() {
    let fib = Fibonacci::new();

    for (i, num) in fib.enumerate().take(10) {
        println!("Fibonacci {}: {}", i + 1, num);
    }
}

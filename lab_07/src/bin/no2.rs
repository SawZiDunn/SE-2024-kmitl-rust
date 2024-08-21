// stack data structure basic implementation
use std::fmt;

struct BoxedStack {
    data: Box<Vec<i32>>,
}

impl fmt::Debug for BoxedStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack Contents: {:?}", self.data)
    }
}

impl BoxedStack {
    fn new() -> Self {
        BoxedStack {
            data: Box::new(Vec::new()),
        }
    }

    fn push(&mut self, value: i32) {
        println!("Pushing {} onto the stack!", value);
        self.data.insert(0, value);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            println!("The stack is empty!\nYou cannot pop.");
            return None;
        }
        println!("{} from the stack!", self.data[0]);
        Some(self.data.remove(0))
    }

    fn peek(&self) -> Option<&i32> {
        if self.data.is_empty() {
            return None;
        }
        println!("The top of the stack is {}", self.data.first().unwrap());
        self.data.first().clone() // or as_ref()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn print_stack(&self) {
        if self.data.is_empty() {
            println!("The stack is empty!\nYou cannot print.");
        } else {
            println!("Stack Contents0: {:?}", self.data);
        }
    }

    fn count(&self) -> i32 {
        let x = self.data.len() as i32;
        println!("The length of the stack is {}", x);
        x
    }
}

fn main() {
    let mut boxstack1 = BoxedStack::new();
    boxstack1.push(10);
    boxstack1.push(20);
    boxstack1.push(30);

    boxstack1.count();

    boxstack1.print_stack();
    boxstack1.peek();

    boxstack1.pop(); //  30 popped
    boxstack1.print_stack();
    boxstack1.peek();

    boxstack1.pop(); // 20 popped
    boxstack1.print_stack();

    boxstack1.pop(); // 10 popped
    boxstack1.print_stack();

    println!(
        "Is the stack empty?: {}",
        boxstack1.is_empty().to_string().to_uppercase()
    );
}

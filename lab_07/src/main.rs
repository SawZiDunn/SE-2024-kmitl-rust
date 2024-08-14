// stack data structure basic implementation

#[derive(Debug)]
struct BoxedStack {
    data: Box<Vec<i32>>,
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
        Some(self.data.remove(0))
    }

    fn peek(&self) -> Option<&i32> {
        self.data.first().clone() // or as_ref()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn print_stack(&self) {
        if self.data.is_empty() {
            println!("The stack is empty!");
        } else {
            println!("Stack Contents: {:?}", self.data);
        }
    }
}

fn main() {
    let mut boxstack1 = BoxedStack::new();
    boxstack1.push(10);
    boxstack1.push(20);
    boxstack1.push(30);

    boxstack1.print_stack();
    println!("The top of the stack is {}", boxstack1.peek().unwrap());

    println!("Popped {} from the stack!", boxstack1.pop().unwrap()); //  30 popped
    boxstack1.print_stack();
    println!("The top of the stack is {}", boxstack1.peek().unwrap());

    println!("Popped {} from the stack!", boxstack1.pop().unwrap()); // 20 popped
    boxstack1.print_stack();

    println!("Popped {} from the stack!", boxstack1.pop().unwrap()); // 10 popped
    boxstack1.print_stack();

    println!("Is the stack empty?: {}", boxstack1.is_empty());
}

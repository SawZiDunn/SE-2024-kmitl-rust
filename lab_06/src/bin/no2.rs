use std::fmt;

struct DataStore<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T> DataStore<T> {
    fn new() -> Self {
        DataStore { items: Vec::new() }
    }

    // 2. DataStore Methods
    fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    fn remove_item(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    fn get_item(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn find_item<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.items.iter().find(|&item| predicate(item))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum DataType<T> {
    Number(T),
    Text(String),
    Boolean(bool),
}

// 4. print Method for DataType
impl<T: fmt::Display> fmt::Display for DataType<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Number(num) => write!(f, "Number: {}", num),
            DataType::Text(text) => write!(f, "Text: {}", text),
            DataType::Boolean(b) => write!(f, "Boolean: {}", b),
        }
    }
}

fn main() {
    // DataStore with DataType<i32>
    let mut int_store = DataStore::<DataType<i32>>::new();
    int_store.add_item(DataType::Number(42));

    // DataStore with DataType<f64>
    let mut float_store = DataStore::<DataType<f64>>::new();
    float_store.add_item(DataType::Number(3.14));

    // DataStore with String
    let mut string_store = DataStore::<String>::new();
    string_store.add_item("Hello".to_string());

    println!("{}", int_store.get_item(0).unwrap());
    println!("{}", float_store.get_item(0).unwrap());
    println!("{}", string_store.get_item(0).unwrap());

    let found = int_store.find_item(|item| match item {
        DataType::Number(n) => *n > 30,
        _ => false,
    });

    println!("Found: {:?}", found);
}

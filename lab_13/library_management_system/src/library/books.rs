use super::LibraryItem;

pub struct Book {
    title: String,
    available: bool,
}

impl Book {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            available: true,
        }
    }
}

impl LibraryItem for Book {
    fn title(&self) -> &str {
        &self.title
    }

    fn check_out(&mut self) {
        self.available = false;
        println!("Checked out book: {}", self.title);
    }

    fn check_in(&mut self) {
        self.available = true;
        println!("Checked in book: {}", self.title);
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

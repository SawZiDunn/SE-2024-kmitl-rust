use super::LibraryItem;

pub struct AudioBook {
    title: String,
    available: bool,
}

impl AudioBook {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            available: true,
        }
    }
}

impl LibraryItem for AudioBook {
    fn title(&self) -> &str {
        &self.title
    }

    fn check_out(&mut self) {
        self.available = false;
        println!("Checked out audiobook: {}", self.title);
    }

    fn check_in(&mut self) {
        self.available = true;
        println!("Checked in audiobook: {}", self.title);
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

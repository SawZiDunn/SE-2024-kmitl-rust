mod library;

use library::{books::Book, media::AudioBook, LibraryItem};

fn main() {
    let mut book = Book::new("The Rust Programming Language");
    let mut audiobook = AudioBook::new("Rust for Beginners");

    print_item_status(&book);
    book.check_out();
    print_item_status(&book);

    print_item_status(&audiobook);
    audiobook.check_out();
    print_item_status(&audiobook);
}

fn print_item_status(item: &dyn LibraryItem) {
    println!(
        "{} is {}",
        item.title(),
        if item.is_available() {
            "available"
        } else {
            "not available"
        }
    );
}

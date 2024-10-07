mod book;
mod library;

use book::Book;
use library::Library;

fn main() {
    println!("📚 Welcome to the Book Library!");

    let mut my_library = Library::new("My Rusty Library");

    let book1 = Book::new("The Rust Programming Language", "Steve Klabnik", 2019);
    let book2 = Book::new("Rust for Beginners", "John Doe", 2021);

    my_library.add_book(book1);
    my_library.add_book(book2);

    my_library.list_books();

    my_library.remove_book(1);
    my_library.list_books();

    match my_library.find_book_by_title("Rust for Beginners") {
        Some(book) => println!("Found book: {:?}", book),
        None => println!("Book not found."),
    }

    my_library.list_books_by_author("Steve Klabnik");
}

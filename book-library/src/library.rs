use crate::book::Book;
use std::collections::HashMap;

pub struct Library {
    pub name: String,
    pub books: HashMap<u32, Book>,
}

impl Library {
    // Function to create a new Library
    pub fn new(name: &str) -> Library {
        Library {
            name: name.to_string(),
            books: HashMap::new(),
        }
    }

    // Add a new book to the library
    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.id, book.clone());
        println!("✅ Added book: {} by {}", book.title, book.author);
    }

    // Remove a book by its ID
    pub fn remove_book(&mut self, id: u32) {
        if let Some(book) = self.books.remove(&id) {
            println!("❌ Removed book: {}", book.title);
        } else {
            println!("⚠️ Book not found!");
        }
    }

    // List all books in the library
    pub fn list_books(&self) {
        if self.books.is_empty() {
            println!("📚 No books available.");
        } else {
            println!("\n📚 Listing all books:");
            for book in self.books.values() {
                println!("ID: {}, Title: {}, Author: {}, Year: {}", 
                         book.id, book.title, book.author, book.year);
            }
        }
    }

    // Find a book by its title
    pub fn find_book_by_title(&self, title: &str) -> Option<&Book> {
        for book in self.books.values() {
            if book.title.to_lowercase() == title.to_lowercase() {
                return Some(book);
            }
        }
        None
    }

    // List books by a specific author
    pub fn list_books_by_author(&self, author: &str) {
        let books_by_author: Vec<&Book> = self.books
            .values()
            .filter(|&book| book.author == author)
            .collect();

        if books_by_author.is_empty() {
            println!("No books found by author: {}", author);
        } else {
            println!("\n📚 Books by {}:", author);
            for book in books_by_author {
                println!("ID: {}, Title: {}", book.id, book.title);
            }
        }
    }
}

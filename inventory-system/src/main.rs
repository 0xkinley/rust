mod inventory;
mod item;

use inventory::Inventory;
use item::{Book, Electronic};

fn main(){

    let mut book_inventory = Inventory::new();

    let book1 = Book{
        title: String::from("Harry Potter and The Philosopher's Stone"),
        author: String::from("J.K. Rowling"),
        year: 1997,
    };

    let book2 = Book{
        title: String::from("Alice in wonderland"),
        author: String::from("Lewis Carroll"),
        year: 1865,
    };

    book_inventory.add_item(book1);
    book_inventory.add_item(book2);

    println!("Book Inventory:");
    book_inventory.list_items();

    let mut electronic_inventory = Inventory::new();

    let electronic1 = Electronic{
        name: String::from("Laptop"),
        brand: String::from("Dell"),
    };

    let electronic2 = Electronic{
        name: String::from("Smartwatches"),
        brand: String::from("Apple"),
    }; 

    electronic_inventory.add_item(electronic1);
    electronic_inventory.add_item(electronic2);

    println!("\n Electronic Inventory:");
    electronic_inventory.list_items();


}

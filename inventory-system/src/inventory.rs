use std::fmt::Display;

pub struct Inventory<T> {
    items: Vec<T>,
}

impl <T> Inventory<T> {
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn list_items(& self) 
    where
        T: Display, {
            for item in &self.items{
                println!("{}", item);
            }
    }
}
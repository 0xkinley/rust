#[derive(Debug, Clone)]

pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl Book {
    // Function to create a new Book struct
    pub fn new(title: &str, author: &str, year: u32) -> Book {
        static mut ID_COUNTER: u32 = 1;
        unsafe {
            let book = Book {
                id: ID_COUNTER,
                title: title.to_string(),
                author: author.to_string(),
                year,
            };
            ID_COUNTER += 1;
            book
        }
    }
}

use std::fmt;

pub struct Book{
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl fmt::Display for Book{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{} by {}, published in {}", self.title, self.author, self.year)
    }
}

pub struct Electronic{
    pub name: String,
    pub brand: String,
}

impl fmt::Display for Electronic{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{} from {}", self.name, self.brand)
    }
}
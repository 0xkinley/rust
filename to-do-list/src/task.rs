use std::fmt;

#[derive(Debug)]

pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {

    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed {"✔️"} else {"❌"};
        write!(f, "[{}]  {} - {}", status, self.id, self.description)
    }
}
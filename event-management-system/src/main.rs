use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use chrono::{NaiveDate, Utc};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Event {
    name: String,
    location: String,
    date: NaiveDate,
    completed: bool,
}

impl Event {
    
    fn new(name: String, location: String, date: NaiveDate) -> Event {
        Event {
            name,
            location,
            date,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

struct EventManager {
    events: HashMap<String, Event>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            events: HashMap::new(),
        }
    }

    fn add_event(&mut self, event: Event) {
        self.events.insert(event.name.clone(), event);
    }

    fn list_events(&self) {
        for event in self.events.values() {
            println!("{:?}", event);
        }
    }

    fn find_event(&self, name: &str) -> Option<&Event> {
        self.events.get(name)
    }

    fn mark_event_completed(&mut self, name: &str) -> Result<(), String> {
        match self.events.get_mut(name) {
            Some(event) => {
                event.mark_completed();
                Ok(())
            }
            None => Err(format!("Event '{}' not found", name)),
        }
    }

    fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        for event in self.events.values() {
            let event_data = format!(
                "{},{},{},{}\n",
                event.name, event.location, event.date, event.completed
            );
            file.write_all(event_data.as_bytes())?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let name = parts[0].to_string();
                let location = parts[1].to_string();
                let date = NaiveDate::parse_from_str(parts[2], "%Y-%m-%d").unwrap();
                let completed = parts[3].parse::<bool>().unwrap();
                let event = Event {
                    name,
                    location,
                    date,
                    completed,
                };
                self.add_event(event);
            }
        }
        Ok(())
    }
}

fn main() {
    let mut event_manager = EventManager::new();

    let event1 = Event::new(
        "Token2049".to_string(),
        "Singapore".to_string(),
        NaiveDate::from_ymd(2024, 9, 18),
    );
    let event2 = Event::new(
        "Web3Conf".to_string(),
        "Goa".to_string(),
        NaiveDate::from_ymd(2024, 12, 10),
    );
    event_manager.add_event(event1);
    event_manager.add_event(event2);

    event_manager.list_events();

    if let Some(event) = event_manager.find_event("Token2049") {
        println!("Found event: {:?}", event);
    } else {
        println!("Event not found!");
    }

    match event_manager.mark_event_completed("Token2049") {
        Ok(_) => println!("Event marked as completed!"),
        Err(e) => println!("Error: {}", e),
    }

    if let Err(e) = event_manager.save_to_file("events.txt") {
        println!("Error saving events: {}", e);
    }

    let mut new_event_manager = EventManager::new();
    if let Err(e) = new_event_manager.load_from_file("events.txt") {
        println!("Error loading events: {}", e);
    } else {
        println!("Loaded events:");
        new_event_manager.list_events();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_add_event() {
        let mut manager = EventManager::new();
        let event = Event::new(
            "Coldplay".to_string(),
            "Bombay".to_string(),
            NaiveDate::from_ymd(2024, 11, 30),
        );
        manager.add_event(event.clone());

        assert_eq!(manager.find_event("Coldplay"), Some(&event));
    }

    #[test]
    fn test_mark_event_completed() {
        let mut manager = EventManager::new();
        let event = Event::new(
            "Music Festival".to_string(),
            "Ziro".to_string(),
            NaiveDate::from_ymd(2024, 9, 15),
        );
        manager.add_event(event);

        assert!(manager.mark_event_completed("Music Festival").is_ok());
        assert!(manager.find_event("Music Festival").unwrap().completed);
    }

    #[test]
    fn test_find_event_not_found() {
        let manager = EventManager::new();
        assert_eq!(manager.find_event("Nonexistent Event"), None);
    }

    #[test]
    fn test_save_and_load_events() {
        let mut manager = EventManager::new();
        let event = Event::new(
            "Diljit Dosanjh".to_string(),
            "New Delhi".to_string(),
            NaiveDate::from_ymd(2024, 8, 10),
        );
        manager.add_event(event.clone());

        manager.save_to_file("test_events.txt").unwrap();

        let mut new_manager = EventManager::new();
        new_manager.load_from_file("test_events.txt").unwrap();

        assert_eq!(new_manager.find_event("Diljit Dosanjh"), Some(&event));
    }
}

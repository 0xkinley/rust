use std::fmt;

pub trait Event {
    fn details(&self) -> String;
    fn get_type(&self) -> String;
}

pub struct Movie {
    pub title: String,
    pub duration: u32,
}

impl Event for Movie {
    fn details(&self) -> String {
        format!("Movie: {} ({} minutes)", self.title, self.duration)
    }

    fn get_type(&self) -> String {
        "Movie".to_string()
    }
}

impl fmt::Debug for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Movie: {}", self.title)
    }
}

// Flight event
pub struct Flight {
    pub airline: String,
    pub destination: String,
}

impl Event for Flight {
    fn details(&self) -> String {
        format!("Flight with {} to {}", self.airline, self.destination)
    }

    fn get_type(&self) -> String {
        "Flight".to_string()
    }
}

impl fmt::Debug for Flight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Flight: {}", self.airline)
    }
}

// Train event
pub struct Train {
    pub route: String,
    pub travel_time: u32,
}

impl Event for Train {
    fn details(&self) -> String {
        format!("Train: {} ({} hours)", self.route, self.travel_time)
    }

    fn get_type(&self) -> String {
        "Train".to_string()
    }
}

impl fmt::Debug for Train {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Train: {}", self.route)
    }
}

// Concert event
pub struct Concert {
    pub artist: String,
    pub duration: u32,
}

impl Event for Concert {
    fn details(&self) -> String {
        format!("Concert: {} ({} minutes)", self.artist, self.duration)
    }

    fn get_type(&self) -> String {
        "Concert".to_string()
    }
}

impl fmt::Debug for Concert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Concert by {}", self.artist)
    }
}

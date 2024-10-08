use crate::event::Event;

pub struct BookingSystem<'a> {
    bookings: Vec<&'a dyn Event>,
}

impl<'a> BookingSystem<'a> {

    pub fn new() -> Self {
        BookingSystem {
            bookings: Vec::new(),
        }
    }

    pub fn book_ticket(&mut self, event: &'a dyn Event) {
        println!("Booking a ticket for: {}", event.details());
        self.bookings.push(event);
    }

    pub fn list_bookings(&self) {
        println!("\nAll bookings:");
        for booking in &self.bookings {
            println!("{} - {}", booking.details(), booking.get_type());
        }
    }
}

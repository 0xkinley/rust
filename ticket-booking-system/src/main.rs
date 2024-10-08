mod booking;
mod event;

use booking::BookingSystem;
use event::{Concert, Flight, Movie, Train};

fn main() {
    let mut system = BookingSystem::new();

    let movie = Movie {
        title: "Inception".to_string(),
        duration: 148,
    };
    let flight = Flight {
        airline: "IndiGo Airlines".to_string(),
        destination: "Bengaluru".to_string(),
    };
    let train = Train {
        route: "New Delhi - Agra".to_string(),
        travel_time: 5,
    };
    let concert = Concert {
        artist: "Coldplay India Tour".to_string(),
        duration: 120,
    };

    system.book_ticket(&movie);
    system.book_ticket(&flight);
    system.book_ticket(&train);
    system.book_ticket(&concert);

    system.list_bookings();
}

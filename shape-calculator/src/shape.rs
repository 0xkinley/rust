pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
    pub side1: f64,
    pub side2: f64,
    pub side3: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}
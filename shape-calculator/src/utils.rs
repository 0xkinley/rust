use crate::shape::Shape;
use std::fmt::Debug;

pub fn print_shape_info<T: Shape + Debug>(shape: &T) {
    println!("{:?} - Area: {:.2}, Perimeter: {:.2}", shape, shape.area(), shape.perimeter());
}

impl std::fmt::Debug for crate::shape::Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle with radius {}", self.radius)
    }
}

impl std::fmt::Debug for crate::shape::Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle with width {} and length {}", self.width, self.length)
    }
}

impl std::fmt::Debug for crate::shape::Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square with side length {}", self.side)
    }
}

impl std::fmt::Debug for crate::shape::Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle with base {}, height {}, and sides {}-{}-{}", 
               self.base, self.height, self.side1, self.side2, self.side3)
    }
}

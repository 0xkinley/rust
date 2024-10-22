trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }

    fn name(&self) -> &'static str {
        "Circle"
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &'static str {
        "Rectangle"
    }
}

struct ShapeManager {
    shapes: Vec<Box<dyn Shape>>,
}

impl ShapeManager {

    fn new() -> ShapeManager {
        ShapeManager { shapes: vec![] }
    }

    fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn display_shapes(&self) {
        for shape in &self.shapes {
            println!("{} has an area of: {}", shape.name(), shape.area());
        }
    }
}

fn main() {

    let mut manager = ShapeManager::new();

    let circle = Box::new(Circle { radius: 5.0 });
    let rectangle = Box::new(Rectangle { width: 4.0, height: 7.0 });

    manager.add_shape(circle);
    manager.add_shape(rectangle);

    manager.display_shapes();
}

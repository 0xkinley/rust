mod shape;
mod utils;

use shape::{Circle, Rectangle, Square, Triangle};
use utils::print_shape_info;

fn main() {
    
    let circle = Circle { radius: 5.0 };

   
    let rectangle = Rectangle {
        width: 10.0,
        length: 5.0,
    };

    
    let square = Square { side: 4.0 };

    
    let triangle = Triangle {
        base: 6.0,
        height: 4.0,
        side1: 5.0,
        side2: 6.0,
        side3: 7.0,
    };

    print_shape_info(&circle);
    print_shape_info(&rectangle);
    print_shape_info(&square);
    print_shape_info(&triangle);
}


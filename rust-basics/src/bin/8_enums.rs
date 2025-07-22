fn main() {
    //-------------------------------------
    //               Enums
    //-------------------------------------

    // They allow you to define a type by enumerating 
    // its possible variants.
    let my_direction = Direction::North;
    move_around(my_direction);

    // enum with values
    // Create instances of different shapes
    // Pattern matching
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}

//-------------------------------------
//               Enums
//-------------------------------------
// enum with values
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape 
    // Pattern matching
    let ans = match shape {
        Shape:: Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => {
            width * height
        }
        Shape::Square(side) => side * side
    };
    return ans;
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn move_around(_direction: Direction) {
    // implements logic to move a character around
}

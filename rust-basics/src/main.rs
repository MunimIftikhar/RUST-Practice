use std::fs;

fn main() {

    //-------------------------------------
    //               Numbers
    //--------------------------------------
    let x: i32 = -10; // signed integer
    let y: u32 = 10; // unsigned integer
    let z: f32 = 10.5;

    println!("x: {}, y: {}, z: {}", x, y, z);

    // Boolean
    let is_male: bool = false;
    let is_above_18: bool = true;

    if is_above_18{
        println!("You are above 18")
    }

    if !is_male{
        println!("You are not male")
    }

    //-------------------------------------
    //               Strings
    //-------------------------------------
    let ax: &str = "call me Monah Darlin"; // can change space at runtime

    // don't have a fixed type
    // very similar to arrays (can change its length at runtime)
    // head/stack (when we need more space in ram while appending into the string)

    let greeting = String::from(ax);
    println!("{}", greeting);

    // optional character
    let char1 = greeting.chars().nth(1);

    match char1{
        Some(c) => println!("{}", c),
        None => print!("No character at index 1000"),
    }

    println!("{}", char1.unwrap());

    //-------------------------------------
    //            Conditionals
    //-------------------------------------
    if is_above_18{
        println!("You are above 18")
    } 
    else if is_male {
        println!("You are not male")
    }

    //-------------------------------------
    //                 loop
    //-------------------------------------
    for i in 0..10 {
        print!("{}", i);
    } 
    println!("");

    let sentence: String = String::from("call me Monah Darlin");
    let first_word: String = get_first_word(sentence);
    print!("first word at: {}", first_word);

    //-------------------------------------
    //             functions
    //-------------------------------------
    let a = 10;
    let b = 20;
    let sum = do_sum(a, b);
    println!("sum: {}", sum);

    // Move to rust-jargon to understand ownership, borrowing & references, Mutability.

    //-------------------------------------
    //            Structs
    //-------------------------------------
    let name = String::from("Alice");
    let age = 30;
    let active = true;

    let _user = User{
        name: name,
        age: age,
        active: active,
    };

    // you can also create struct without any data to 
    // associate the the functions with it
    let noShape = NoShape;
    print!("The area of the no shape is {}", noShape.area());
    
    // struct is saved in the stack but if we define variables
    // like strings inside of struct it gets saved in heap

    // Here, we are defining the area function that is 
    // associated to the struct
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());

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

    //-------------------------------------
    //     Error Handling | Result enum
    //-------------------------------------

    // Rust gives Result enum to handle error handling
    /*
    enum Result<A, B> {
        Ok(A),
        Err(B),
    }
     */

    /*  uncomment to run the code
    let file_path = String::from("example.txt");
    let contents = fs::read_to_string(file_path);

    match contents{
        Ok(content) => {
            println!("File cntent: {}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    println!("hi there");

    let res = read_from_file_unsafe("example.txt".to_string());
    println!("File cntent: {}", res);

    let res_1 = read_from_file_unsafe_1("example.txt".to_string());
    println!("File cntent: {}", res_1.unwrap());
    */

    //-------------------------------------
    //     Error Handling | Option enum
    //-------------------------------------

    // option enums helps get rid of null types throught the 
    // code base
    // Rust doesn't have null
    // having null is a billion dollar mistake
    // because you'll face a lot of runtime errors when you
    // expect a variable to be something but it turns out to
    // null at runtime
    /*
    pub enum Option<T> {
        None, // I'll have none or
        Some(T), // something of any type
    }
    */
    // here, we are using option to return null if we don't
    // find a character in a string
    let my_string = String::from("raman");
    let res = find_first_a(my_string);

    match res {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }

}

//-------------------------------------
//     Error Handling | Option enum
//-------------------------------------

fn find_first_a(s: String) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index);
        }
    }
    return None;
}

//-------------------------------------
//     Error Handling | Result enum
//-------------------------------------

// this code will crash if the error occurs
fn read_from_file_unsafe(file_path: String) -> String{
    let contents = fs::read_to_string(file_path);
    // returning the string here only
    return contents.unwrap();
}

// this code will crash if the error occurs
fn read_from_file_unsafe_1(file_path: String) -> Result<String, String>{
    let contents = fs::read_to_string(file_path);
    // returning the string here only
    match contents{
        Ok(content) => Ok(content),
        Err(_) => Err("Error reading file".to_string())
    }
}

//-------------------------------------
//               Generics
//-------------------------------------

struct Point<T>{
    x: T,
    y: T
}

struct Poin1t<A, B>{
    x: A,
    y: B,
    z: B
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

//-------------------------------------
//               structs
//-------------------------------------
struct NoShape;

impl NoShape{
    fn area(&self) -> u32 {
        return 0;
    }
}

struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct User{
    name: String,
    age: u32,
    active: bool,
}

//-------------------------------------
//         strings | loops
//-------------------------------------
fn get_first_word(sentence: String) -> String{
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}

fn do_sum(a: i32, b: i32) -> i32{
    return a + b;
}

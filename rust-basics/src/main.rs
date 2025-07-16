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

    //-------------------------------------
    //          stack vs heap
    //-------------------------------------
    update_string();

    //-------------------------------------
    //               ownership
    //-------------------------------------

    // at a time only one owner can point the data
    // if we point another the fist one dies automatically
    let s1 = String::from("Hi there");
    let s2 = s1;
    // gives error: borrow of moved value
    // println!("{}", s1); // invalid
    println!("{}", s2); // valid
    // you don't need to explicitly free the data
    // as the owner dies the data gets freed automatically

    // another example

    let my_string = String::from("hello");
    // If we pass an int the var get copies (cloned)
    // when we pass string ownership gets transferred
    // some_string will become the owner now
    // if we pass clone "my_string.clone" the data gets cloned
    // two things can never point one thing
    // clonning is very expensive
    takes_ownership(my_string.clone());

    // another fix would be to return and assign the string to same var again
    // this can help pass ownerships without clonning or references
    // my_string = takes_ownership(my_string);

    // This line would cause a compile error 
    // because ownership has been moved.
    println!("{}", my_string); 

    //-------------------------------------
    //       Borrowing & References
    //-------------------------------------

    // Problem of ownership is we had to hack to return to original owner
    // i.e. returning the variable from func & assigning it again

    // References: We use references to give the address of an original owner 
    // rather than the owenership of the owner over to a function
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);    // This is valid, The first pointer wasn't invalidated

    // another example

    // Borrowing: Transferring ownership of variables to function, the ownership
    // of strings to the function, the ownership of string remains with original
    // variable in main function. This allows you to use original string after 
    // function call
    let my_string = String::from("Hello, Rust!");
    takes_ownership_1(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred

    //-------------------------------------
    //         Mutable References
    //-------------------------------------

    // If you want to update the values of a variable inside 
    // the function, use mutable refrence
    let mut r1 = String::from("Hello");
    update_word(&mut r1); // mutable reference
    let _r3 = &mut r1; // mutable reference
    // cannot borrow the mutable reference twice
    // If someone makes an immutable reference , they don’t 
    // expect the value to change suddenly
    // If more than one mutable references happen, there is a 
    // possibility of a data race and synchronization issues
    // let r4 = &mut r1;
    // update_word(&mut r3);
    println!("{}", r1);

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
//       Mutable References
//-------------------------------------   
fn update_word(word: &mut String) {
    word.push_str(" World");
}


//-------------------------------------
//       Borrowing & References
//-------------------------------------
fn takes_ownership_1(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}


//-------------------------------------
//             Ownership
//-------------------------------------
fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

//-------------------------------------
//          stack vs heap
//-------------------------------------
fn update_string(){
    // string will be saved in the heap
    // because string has no fixed size it can be changed
    // stack will point the string in the heap
    let mut s = String::from("Initial String");
    println!("Before update: {}", s);
    println!("Capacity: {}. Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());


    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!("Capacity: {}. Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
}

fn do_sum(a: i32, b: i32) -> i32{
    return a + b;
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



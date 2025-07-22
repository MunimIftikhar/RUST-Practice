fn main(){
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
}
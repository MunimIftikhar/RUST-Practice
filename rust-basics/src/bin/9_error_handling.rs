use std::fs;
fn main() {
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
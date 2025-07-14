fn main() {
    // Numbers
    //--------------------
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

    //-------------------
    // Strings
    //-------------------
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

    //-------------------
    // Conditionals
    //-------------------
    if is_above_18{
        println!("You are above 18")
    } 
    else if is_male {
        println!("You are not male")
    }

    //-------------------
    // loop
    //-------------------
    for i in 0..10 {
        print!("{}", i);
    } 
    println!("");

    let sentence: String = String::from("call me Monah Darlin");
    let first_word: String = get_first_word(sentence);
    print!("first word at: {}", first_word);

    //-------------------
    // functions
    //-------------------
    let a = 10;
    let b = 20;
    let sum = do_sum(a, b);
    println!("sum: {}", sum);

    //-------------------
    // stack vs heap
    //-------------------
    update_string();

    //-------------------
    // ownership
    //-------------------

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

    //-------------------
    // Borrowing & Ownership
    //-------------------

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
}

fn takes_ownership_1(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

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



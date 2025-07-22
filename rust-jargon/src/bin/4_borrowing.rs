fn main(){
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

    // Example 2

    // we can use & to reference s1 where
    // s2 will borrow the s1 by pointing the s1, 
    // here, s1 still owns the value
    let r1 = String::from("Monah");
    let r2 = &r1; // this is valid

    // Mutablity: you can use mut keyword to also change the
    // value by reference
    let mut v1 = String::from("Monah");
    let v2 = &mut v1; // this is valid
    
}

fn takes_ownership_1(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}
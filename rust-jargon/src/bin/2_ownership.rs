fn main() {
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
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}
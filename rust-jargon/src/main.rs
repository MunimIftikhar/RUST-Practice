fn main() {

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
    //              Moving
    //-------------------------------------

    // RUST Ownership: only one owner at a time

    let a1 = String::from("Monah");
    let a2 = a1; // owner gets moved
    // a1 is completely invalid now 
    // a2 is now the owner of the string
    println!("Name is {}", a2);

    // Example 2

    let a3 = String::from("Monah");
    { // the ownership is moved to a4
        let a4 = a3;
        println!("Name is {}", a4);
    }
    // You're trying to access the null pointer (dangling pointer)
    // println!("Name is {}", a1); // this will throw an error

    // Example 3

    let s1 = String::from("Monah");
    // Ownership moved to do_something variable s2
    do_something(s1);

    // you can return the var to same old var 
    // to return the ownership (but this is an ugly code)

    //-------------------------------------
    //              Borrowing
    //-------------------------------------

    // we can use & to reference s1 where
    // s2 will borrow the s1 by pointing the s1, 
    // here, s1 still owns the value
    let r1 = String::from("Monah");
    let r2 = &r1; // this is valid

    // Mutablity: you can use mut keyword to also change the
    // value by reference
    let mut v1 = String::from("Monah");
    let v2 = &mut v1; // this is valid

    //-------------------------------------
    //         Rules of References
    //-------------------------------------
    /*
        * At any given time, you can have either one mutable
          reference or any number of immutable references
        * References must always be valid
    */ 
    let mut b1 = String::from("Monah");
    let b2 = &mut b1; // this is valid
    // let mut b3 = &b1; // this is invalid

    // Example 2
    let a1 = String::from("Monah");
    let a2 = &a1; // this is valid
    let a3 = &a1; // this is valid
    let a4 = &a1; // this is valid
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

fn do_something(s2: String){
    println!("Name is {}", s2);
}

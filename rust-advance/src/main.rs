fn main() {
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

fn do_something(s2: String){
    println!("Name is {}", s2);
}

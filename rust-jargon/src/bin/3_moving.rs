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
}

fn do_something(s2: String){
    println!("Name is {}", s2);
}
fn main() {
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

fn update_word(word: &mut String) {
    word.push_str(" World");
}
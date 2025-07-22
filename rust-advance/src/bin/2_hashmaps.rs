use std::collections::HashMap;

fn main() {    
    //-------------------------------------------------------
    //                Collections | Hashmaps
    //-------------------------------------------------------

    /* 
        Hashmaps stores a key value pair in rust
        similar to dict in python
    */

    /* 
        Methods: insert, get, remove, clear
    */

    let mut users: HashMap<String, u32> = HashMap::new();
    // Another way to define hashmap
    let mut users1 = HashMap::new();

    users.insert(String::from("Monah"), 22);
    users.insert(String::from("Fatimah"), 18);

    users1.insert(String::from("Monah"), 22);
    users1.insert(String::from("Fatimah"), 18);

    // get method returns an option coz key might exist or not
    let first_user_age = users.get("Mariah");

    match first_user_age{
        Some(age) => print!("First User Age: {}", age),
        None => print!("User not found"),
    }

    /* 
        HARD ASSIGNMENT:

        Write a function that takes a vector of tuples (each 
        tuple containing a key and a value) and returns a 
        HashMap where the keys are the unique keys from the
        input tuples and the values are vectors of all 
        corresponding values associated with each key.

        EASY ASSIGNMENT:

        Write a function called group_values_by_key which takes
        a vector as an input but it is a vector of tuple 
        (String, i32). Every value of this vector has two parts.
        You have to convert the vector to hashmap and 
        return a hashmap.
    */

    let vec = vec![(String::from("Monah"), 22), (String::from("Fatimah"), 16), (String::from("Mariah"), 18)];
    let hashmap = group_values_by_key(vec);
    println!("HashMap: {:?}", hashmap);

}

//-------------------------------------------------------
//                Collections | Hashmaps
//-------------------------------------------------------
fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {

    let mut hashmap = HashMap::new();
    // we destructured the vec here in the form of tuple
    for (key,value) in vec{
        hashmap.insert(key, value);
    }
    return hashmap;
}

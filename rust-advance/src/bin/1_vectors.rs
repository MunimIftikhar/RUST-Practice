fn main() {
    //-------------------------------------------------------
    //                      Collections
    //-------------------------------------------------------
    /*
        Rust's standard library includes a number of very useful 
        data structures called collections. Most other data types
        represent one specific value, but collections can contain
        multiple values. The data these collections point to is
        stored on the heap.
    */

    //-------------------------------------------------------
    //               Collections | Vectors
    //-------------------------------------------------------
    /* 
        Vectors allow you to store more than one value in
        a single data structure that puts all the values 
        next to each other in memory.

        The var will be stored in the stack but data will
        be stored in the heap.

        Works like array where we can add, delete a value
    */
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);

    /* 
        ASSIGNMENT:

        Write a function that takes a vector as an input and
        returns a vector with even values.
    */

    /*
        we are passing the ownership to even_number function
        if we are not using &
        If we use &, we are passing vec as reference and vec
        will be borrowed by the even_number function
    */   
    let even = even_number(&vec);
    println!("Even Values: {:?}", even);

    // Easier way to initialize the vector
    let mut _numbers = vec![1, 2, 3, 4, 5, 6];

    /*
        Defining the type of the vector
        Explicitly giving type using generics

        let mut numbers: Vec <i8> = vec![1, 2];

        Rust compiler automatically infer the type
        Or you can explicitly define the type
    */

}

//----------------------------------------------------------
//                   Collections | Vectors
//----------------------------------------------------------
fn even_number(vector: &Vec<i32>) -> Vec<i32> {

    let mut even = Vec::new();

    // iterating through the vector
    for val in vector {
        if val % 2 == 0{
            // de-referencing the value to access the number
            even.push(*val);
        }
    }

    return even;
}

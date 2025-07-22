use std::collections::HashMap;
fn main() {
    //-------------------------------------------------------
    //                        Iterators
    //-------------------------------------------------------
    /*    
        From official documentation:

        The iterator pattern allows you to perform some task 
        on a sequence of items in turn. An iterator is 
        responsible for the logic of iterating over each item 
        and determining when the sequence has finished. When 
        you use iterators, you don’t have to reimplement that 
        logic yourself.

        In Rust, iterators are lazy, meaning they have no effect 
        until you call methods that consume the iterator to use it
        up. 

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        we're still using iterator when we do following:
        for v in v1 {}
        for v in v1.iter() {} // same as above
    */

    // TYPES OF ITERATORS:

    // 1. iterating using the for loops
    let nums = vec![1, 2, 3];
    print!("Array values: ");
    for value in nums{
        print!("{} ", value);
    }
    println!("");

    // 2. Iterating after creating an `iterator`
    /*
        The iter() method in Rust provides a way to iterate
        over the elements of a collection by borrowing them.

        you can't mutate the variables since we have an immutable
        reference to the internal elements
    */
    let v1 = vec![1, 2, 3];

    // v1_iter borrows the values from v1
    // v1_iter did not become the owner of the v1 values
    // but we have immutable iterator
    // we can't update values using v1_iter
    let v1_iter = v1.iter();

    print!("Array values: ");

    for value in v1_iter { // using iter to iterate
        print!("{} ", value);
    }

    println!("");

    // 3. Iterating using `.next`
    let v2 = vec![1, 2, 3];

    // Here, we are defining mutable iterator
    // next() implementation requires mutable var
    let mut v2_iter = v2.iter();

    // The type here will be: Option<&i32>
    // let _first_number: Option<&i32> = v2_iter.next();
    // let _seccond_number = v2_iter.next();
    // let _third_number = v2_iter.next();
    // As fouth number don't exist next() will return none
    // so type here will be none
    // let fourth_number: None = v2_iter.next();

    print!("Array values: ");

    // .next() return an option, if val exist it return val
    // otherwise it returns none, if it is none the loop exists
    while let Some(value) = v2_iter.next() {
        print!("{} ", value);
    }

    println!("");

    // 4. iter_mut: If you need to mutate/update the values as well
    
    let mut v3 = vec![1, 2, 3];
    // If you want to borrow the values mutabily
    let v3_iter = v3.iter_mut();

    for value in v3_iter {
        *value = *value + 1;
    }

    println!("Updated Array values: {:?}", v3);

    // 5. into_iter
    /*
        The IntoIterator trait is used to convert a collection
        into an iterator that takes ownership of the collection.

        * First iter was borrowing the values this one is taking
        the owenership.
        * This means whenever you define into_iter you can no
        longer use the original variable

        Useful when:
        1. You no longer need the original collection
        2. When you need to squeeze performance benefits by
        transferring ownershio (avoiding references)
    */

    let v4 = vec![1, 2, 3];
    // If you want to borrow the values mutabily
    let v4_iter = v4.into_iter();

    print!("Array Values: ");
    for value in v4_iter {
        print!("{} ", value);
    }
    println!("");
    //println!("{:?}", v4); // This invalid

    /*
        WHICH TO CHOOSE?

        iter:

        If you want immutable references to the inner variables
        and don't want to transfer ownership.

        iter_mut:

        If you want mutable references to the inner variables and
        don't want to transfer ownership.

        iter_into:

        If you want to move the variable into the iterator and
        don't want to use it afterwards.
    */

    /*

        Iterators provide us something extra. They let you 
        use some extra functions by default. There are two
        types of extra functions: consuming and iterator adaptors.

        1. Consuming Adaptors:

        Methods that call next are called consuming adaptors,
        because calling them uses up the iterator.

        It consumes the iterator e.g. v5_iter, which means
        we can't use the iterator anymore. It also don't return
        anything.
    */

    let v5 = vec![1, 2, 3];
    let v5_iter = v5.iter();
    // sum() is a consuming adaptor
    // You can't use the v5_iter again
    let total: i32 = v5_iter.sum(); 
    println!("Sum: {}", total);
    assert_eq!(total, 6);

    /*  
        2. Iterator Adaptors:

        Iterator adaptors are methods defined on the iterator trait
        that don't consume the iterator. Instead, they produce different
        iterators by changing some aspect of the original iterator.

        It alters the original vector and returns the value.
    */
    let v6 = vec![1, 2, 3, 4, 5, 6];
    let v6_iter = v6.iter();
    // map() is a iterator adaptor
    // |x| is an argument and x + 1 is a function we want to perform
    // returns another iterator
    let iter2 = v6_iter.map(|x| x + 1);
    for x in iter2 {
        print!("{} ", x);
    }
    println!("");
    println!("Original Array Values: {:?}", v6);

    // can't use v6_iter again here because it has been altered
    let v6_iter2 = v6.iter();
    // filter() is used to filter certain values from the vec
    let iter3 = v6_iter2.filter(|x| *x % 2 == 0);
    for x in iter3 {
        print!("{} ", x);
    }
    println!("");

    /* 
        ASSIGNMENT

        Write the logic to first filter all odd values then double
        each value and create a new vector.
    */
    let s1 = vec![3, 3, 2, 6, 7, 9, 0, 4, 8];
    let s1_iter = s1.iter();
    let filter_iter = s1_iter.filter(|x| *x % 2 == 1);
    let double_iter = filter_iter.map(|x| x + 1);
    // collect converts the iterator to a vector
    // we need to explicitly define the type
    let new_vec: Vec<i32> = double_iter.collect();
    for x in new_vec.iter() {
        print!("{} ", x);
    }
    println!("");

    /*  
        Iterators in HashMaps
    */
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 50);
    scores.insert("Charlie", 50);

    println!("Iterating over key value pairs:");
    for (key, value) in scores.iter(){
        println!("{}: {}: ", key, value);
    }

    println!("Iterating over mutable key value pairs:");
    for (key, value) in scores.iter_mut(){
        *value += 10;
        println!("{}: {}: ", key, value);
    }


}

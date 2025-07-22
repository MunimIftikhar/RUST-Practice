fn main(){
    //-------------------------------------
    //          stack vs heap
    //-------------------------------------
    update_string();
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
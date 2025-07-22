fn main(){
    //-------------------------------------
    //             functions
    //-------------------------------------
    let a = 10;
    let b = 20;
    let sum = do_sum(a, b);
    println!("sum: {}", sum);

    // Move to rust-jargon to understand ownership, 
    // borrowing & references, Mutability.
}

fn do_sum(a: i32, b: i32) -> i32{
    return a + b;
}
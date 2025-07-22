fn main() {
    //-------------------------------------
    //                 loop
    //-------------------------------------
    for i in 0..10 {
        print!("{}", i);
    } 
    println!("");

    let sentence: String = String::from("call me Monah Darlin");
    let first_word: String = get_first_word(sentence);
    print!("first word at: {}", first_word);
}

//-------------------------------------
//         strings | loops
//-------------------------------------
fn get_first_word(sentence: String) -> String{
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}
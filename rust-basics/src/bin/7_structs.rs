//-------------------------------------
//               structs
//-------------------------------------
struct NoShape;

impl NoShape{
    fn area(&self) -> u32 {
        return 0;
    }
}

struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct User{
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    //-------------------------------------
    //            Structs
    //-------------------------------------
    let name = String::from("Alice");
    let age = 30;
    let active = true;

    let _user = User{
        name: name,
        age: age,
        active: active,
    };

    // you can also create struct without any data to 
    // associate the the functions with it
    let noShape = NoShape;
    print!("The area of the no shape is {}", noShape.area());
    
    // struct is saved in the stack but if we define variables
    // like strings inside of struct it gets saved in heap

    // Here, we are defining the area function that is 
    // associated to the struct
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());
}


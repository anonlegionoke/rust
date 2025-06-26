//structs
struct User {
    username: String,
    active: bool,
    email: String,
    sign_in_count: u64
}

fn main(){
    let user = User {
        username: String::from("legio"),
        active: true,
        email: String::from("proton"),
        sign_in_count: 25
    };
    println!("User details are {}", user.username);
}

//structs with impl
struct Rect {
    height: u32,
    width: u32,
}

// impl => similar to class in js/ts

/* 
    class Rect {
        width: number;
        height: number;

        constructor(width: number, height: number) {
            this.width = width;
            this.height = height;
        }
        area():number {
            return this.width * this.height;
        }
        static another():number {
            return 1;
        }
    }

    const rect = new Rect(30, 20)
    console.log('Area is', rect.area())
    console.log('Another is', Rect.another())

*/

/* NOTE: 
    static in js/ts or method without self in rust is a function which is defined in the class but not connected to the object defined.
    
    Call it directly from the class [Rect::another()] not from object
*/

impl Rect {
    // define method here
    fn area(&self) -> u32 {
        self.height * self.width
    }
    // another method without &self
    fn another() -> u32 {
        return 1;
    }
}

fn main(){
    let rect = Rect {
        width: 30,
        height: 20
    };
    println!("area of the rectangle is {}", rect.area());
    println!("another should be like this {}", Rect::another());
}

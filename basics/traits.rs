// Traits

/* Traits are like interfaces [ with some very significant changes ] in ts --can refer to trello cards for prev snippets or comments
 *
 */

pub trait Summary {
    fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarise(&self) -> String {
        format!("the name is {} and the age is {} ", self.name, self.age)
    }
}

fn main() {
    let user = User {
        name: String::from("legio"),
        age: 25
    };
    println!("The summary is {}", user.summarise())   
}
-------------------------------------------------------------------------

pub trait Summary {
    fn summarise(&self) -> String{
        String::from("Default Response Implementation")
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {}

fn main() {
    let user = User {
        name: String::from("legio"),
        age: 25
    };
    println!("The summary is {}", user.summarise());
}

-------------------------------------------------------------------------
// Traits as parameters
pub trait Summary {
    fn summarise(&self) -> String{
        String::from("Default Response Implementation")
    }
}

struct User {
    name: String,
    age: u32,
}

struct Fix;

impl Summary for User {}
impl Summary for Fix {
    fn summarise(&self) -> String {
        String::from("This is from fix")
    }
}


fn main() {
    let user = User {
        name: String::from("legio"),
        age: 25
    };
    let fix = Fix;
    notify(&fix);
}

fn notify(user: &impl Summary) {
    println!("The summary is {}", user.summarise());
}

----------------------------------------------------------------------------
// Trait Bound Syntax

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarise());
}

// If there is an another trait called Summary2 then;

fn notify<T: Summary + Summary2>(item: &T) {
    println!("{}", item.summarise());
}

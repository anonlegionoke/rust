// Lifetimes

// Generic Lifetime Annotation -> 'a in a functional example

fn main() {
    let ans;

    let str1 = String::from("small");

    let str2 = String::from("longer");

    ans = longest(&str1, &str2); 

    println!("longest is {}", ans);
}

fn longest<'a>(str1:&'a str, str2: &'a str) -> &'a str {
   if str1.len() > str2.len() {
    str1
   } else {
    str2
   }
}

// Struct with lifetimes
/* We can use references inside a struct too */

struct User<'a> {
    name: &'a str,
}

fn main() {
    let first_name = String::from("legio");
     let user: User= User {
        name: &first_name
    };
    println!("User name is {:?}", user.name);
}


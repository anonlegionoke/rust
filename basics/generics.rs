// Generics

// Code Type 1
fn main() {
    let bigger_int = largest_i32(1, 2);
    let bigger_char = largest_char('a', 'b');

    println!("Bigger int {}", bigger_int);

    println!("Bigger char{:?} ", bigger_char);
}

fn largest_i32(a:i32, b:i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn largest_char(a: char, b: char) -> char {
    if a > b {
        a
    } else {
        b
    }
}

//Code Type 2 - with GENERICS --avoid code repetition using general type arguments

fn main() {
let bigger_int = largest(1, 2);
let bigger_char = largest('a', 'b');

println!("Bigger int {}", bigger_int);

println!("Bigger char{:?} ", bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
if a > b {
    a
} else {
    b
}
}

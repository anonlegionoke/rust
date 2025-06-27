// Enums -> Used to enumerate over various types of a value

// Debug to print using {:?}
// PartialEq to compare partially since the enum Direction contains other values too

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    West,
    South
}

fn main() {
    let my_direction = Direction::South; 
    println!("Target direction is {:?}",  get_target_direction(&my_direction));
}

fn get_target_direction(my_direction: &Direction) -> Direction {
    if *my_direction == Direction::South {
        Direction::North
    } else {
        Direction::South
    }
}

// Enum with values
enum Shape {
    Circle(f64),
    Rectangle(f64, f64)
}

fn main() {
    let rect = Shape::Rectangle(20.0, 30.0);
    calculate_area(&rect);
    let circle = Shape::Circle(12.0);
    calculate_area(&circle);
}


fn calculate_area(shape: &Shape) -> f64 {
    // pattern matching syntax
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };   
    return area;
}

-------------------------------------------------------------------------------------------------------------------
-------------------------------------------------------------------------------------------------------------------

// Options and Result enums from Rust

/* NOTE: 
 *  Options -> Some / None
 *  
 *  Result -> Ok / Err
 * */

pub enum Option<T> {
    None,
    Some(T),
}

/* Example below without using Option enum */
fn find_first_a(s: String) -> i32 {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return index as i32;
        }
    }
    return  -1;
}

fn main() {
    let index = find_first_a(String::from("legio"));
    if index != -1 {
        println!("Index of first a is {}", index)
    } else {
        println!("Index not found")
    }
}

/* Example with using Option enum */
fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return  None;
}

fn main() {
    let index = find_first_a(String::from("legio"));
    match index {
        Some(value) => println!("Index of first a is {}", index),
        None => println!("Index not found")
    }
}

// Result enums from Rust -> Used for error handling

/* Q. Write a function that reads the contents of a file */

use std::fs;

fn main() {
    let reading_file  = fs::read_to_string("Hello World.txt");
    match reading_file {
        Ok(read) => println!("Reading file {}", read),
        Err(err) => println!("Error occured: {}", err)
    }
}

fn with_own_result_enum() -> Result<String, String> {
    let reading_file  = fs::read_to_string("Hello World.txt");
    match reading_file {
        Ok(read) => {
            println!("Reading file {}", read);
            Ok(read)
        }
        Err(err) => {
            println!("Error occured: {}", err);
            Err(err.to_string())
        }
    }
}

/* NOTE:
 *  Do not use ; if you are returning but not explicitly saying 'return'
 * */

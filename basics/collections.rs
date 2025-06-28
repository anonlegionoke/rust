// Collections in Rust

/* - Collections in Rust basically means data structures that can contain more than one value, like arrays,
 *   dynamic vectors, hashmaps...
 * - They are stored on the heap */

-----------------
// Vectors
-----------------

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    println!("{:?}", get_evens(&vec))
}

/* 
Q. Write a function that takes a vector as an input and returns a vector with even values
*/

fn get_evens(vec: &Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();
    for v in vec {
        if v % 2 == 0 {
            even_vec.push(*v);
        }
    }
    return even_vec;
}

// An interesting and minute nuance in dereferecning and return.
// In this type we are owning nothing, always refering, negligble performance gains, but works for
// this lifetime, and obviously more lightweight in theory.

fn get_evens_a(vec: &Vec<i32>) -> Vec<&i32> {
    let mut even_vec = Vec::new();
    for v in vec {
        if v % 2 == 0 {
            even_vec.push(v);
        }
    }
    return even_vec;
}


--------------
// Hash Maps
--------------
use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("name"), "legio");
    users.insert(String::from("age"), "25");

    let get_user_details = users.get("name");

    match get_user_details {
        Some(value) a=> println!("value is {}", value),
        None => println!("Something went wrong"),
    }
}

/* Q. Write a function that takes a vector of tuples (each tuple containing a key and a value) and returns a HashMap where the keys are the unique keys from the input tuples and the values are vectors of all corresponding values associated with each key */

use std::collections::HashMap;

fn convert_to_hashmap(vector_tuple:Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut our_hashmap = HashMap::new();
    for (key, value) in vector_tuple {
        our_hashmap.entry(key).or_insert(vec![]).push(value);
    }
    our_hashmap
}

fn main(){
    let vector_tuple = vec![(String::from("legio"), 21), (String::from("anon"), 25), (String::from("legio"), 24)];
    println!("Our hashmap is {:?}", convert_to_hashmap(vector_tuple));
}

























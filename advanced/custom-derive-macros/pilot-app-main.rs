use hello_macro::{HelloMacro, HelloDupe};
use hello_macro_derive::{HelloMacro, HelloDupe};

#[derive(HelloMacro, HelloDupe)]
struct Cars;

fn main() {
    Cars::hello_macro();
    Cars::hello_dupe();
}
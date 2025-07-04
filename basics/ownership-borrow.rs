// Ownership | Borrowing | Moving | References

/* Ownership and Borrowing added to trello notes already.
 * */

----------
// Moving
----------
fn main() {
    let mut s1 = String::from("legio");
    s1 = get_count(s1);
    println!("Name is {}", s1)
}

fn get_count(s2:String) -> String {
    s2.chars().count().to_string());
}

/* Here => s1 moved its ownership temporarily to s2 via get_count, and implicitly return s2,
 * and assigned it back to s1 again.
 * so, s1 in the println! is valid.
 */

------------
// BORROWING
------------
fn main() {
    let s1 = String::from("legio");
    println!("Count is {}", get_count(&s1))
}

fn get_count(s2: &String) -> String {
    s2.chars().count().to_string()
}

---------------------
// Borrowing with mut
---------------------
fn main() {
    let mut s1 = String::from("legio");
    println!("Count is {}", extend_str(&mut s1))
}

fn extend_str(s2: &mut String) -> String {
    s2.push_str("-anon");
    s2.to_string()
}


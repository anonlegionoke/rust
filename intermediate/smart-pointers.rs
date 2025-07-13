/* 
 * smart pointers => Pointers with additional features like store metadata & remove a pointer
 *                   when no one uses it.
 * 
 * Examples:
 *  
 *  Box<T> [Heap Allication && Single Ownership]
 *  ------
 *
 *  - Store the data specifically on the Heap instead of stack
 * 
 * Rc<T> | Arc<T>
 *
 * RefCell<T> | Mutex<T>
 *
 *
 * Check "Rust checks for the week in trello task cards for more of this intro."
 * */
-------------------------------------------------------------------------------------------------

// Box<T>

trait Vehicle {
    fn drive(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving")
    }
}

fn main() {
    let t: Box<dyn Vehicle>;

    t  = Box::new(Truck);

    t.drive();

    println!("Something")
}
----------------------------------

struct Truck {
    next_truch: Option<Box<Truck>>
}

fn main() {
}

---------------------------------------------------------------------------------------------------
// Rc<T>

use std::rc::Rc;
#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {
    let (truck_a, truck_b, truck_c) = (Rc::new(Truck{capacity: 1}), Rc::new(Truck{capacity: 2}), Rc::new(Truck{capacity: 3}));

    let facility_one = vec![Rc::clone(&truck_a), Rc::clone(&truck_b)];
    let facility_two = vec![Rc::clone(&truck_b), Rc::clone(&truck_c)];

    println!("Facility one {:?} and Facility two {:?}", facility_one, facility_two);
    println!("Truck b strong count {:?}", Rc::strong_count(&truck_b));

    std::mem::drop(facility_two);

    println!("Facility one after drop {:?}", facility_one);
    println!("Truck b strong count after drop{:?}", Rc::strong_count(&truck_b));

}

--------------------------------------------------------------------------------------------------
// Arc<T> [For Multi-thread]

use std::sync::Arc;
#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {
    let (truck_a, truck_b, truck_c) = (Arc::new(Truck{capacity: 1}), Arc::new(Truck{capacity: 2}), Arc::new(Truck{capacity: 3}));

    let thread = std::thread::spawn(move || {
        let facility_one = vec![Arc::clone(&truck_a), Arc::clone(&truck_b)];
        let facility_two = vec![Arc::clone(&truck_b), Arc::clone(&truck_c)];
        (facility_one, facility_two)
     });

    let (facility_one, facility_two) = thread.join().unwrap();

    println!("Facility one {:?} and Facility two {:?}", facility_one, facility_two);

    let truck_b = Arc::clone(&facility_one[1]);
    println!("Truck b strong count {:?}", Arc::strong_count(&truck_b));

    std::mem::drop(facility_two);

    println!("Facility one after drop {:?}", facility_one);
    println!("Truck b strong count after drop {:?}", Arc::strong_count(&truck_b));

}

-------------------------------------------------------------------------------------------------
// Deref Trait [Deref coercion happens here => automatically insert calls into .deref()]

use std::ops::Deref;
struct MyBox<T> (T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} 

fn main() {
    let x = MyBox(String::from("Hello"));

    println!("{}", x.len());
}

-------------------------------------------------------------------------------------------------


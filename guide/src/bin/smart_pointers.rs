use std::rc::Rc;

#[allow(dead_code)]
// Box: that the enclosed data needs to be stored on the heap instead of the stack
// two uses cases
// (1): when we have a variable with a trait type that can't be computed at compile time we could use a box there
// (2): recursive data types
trait Vehicle {
    fn drive(&self);
}
#[derive(Debug)]
struct Car {
    capacity: i32,
}

struct Truck {
    next_truc: Option<Box<Truck>>, // remove this for the (1)
}

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Drinving a Truck")
    }
}

//RC count the number of references to that memory and it's going to keep that memory alive until the last reference
// goes out of scope
//Example we have some structure and we want a reference to that structure to be stored in multiple data structures and
// we're not really sure which data structure is going to go out of scope or what's going to be done with that data
// structure but we do know that we want that memory to be deallocated if there's no references to it remaining

fn main() {
    //// using BOX
    //every time you reference a trait need to be prefixed with dyn
    //let t: Box<dyn Vehicle> = Box::new(Truck); // (1)
    let t: Truck = Truck {
        next_truc: Some(Box::new(Truck { next_truc: None })),
    };

    t.drive();
    ///////////////////////////////////////////////////////////
    // RC
    let (car_a, car_b, car_c) = (
        Rc::new(Car { capacity: 1 }),
        Rc::new(Car { capacity: 1 }),
        Rc::new(Car { capacity: 3 }),
    );

    let facility_one = vec![car_a, Rc::clone(&car_b)];
    //assuming you only need a read-only reference to this problem with that approach is that the main function has
    // to maintain ownership of truck b and then truck b would get deallocated when the main function is done even if we
    // stopped needing truck b long before that.

    // using Rc::clone allows to main to mantaing the ownership
    let facility_two = vec![Rc::clone(&car_b), car_c];
    println!(
        "references of car_b before deleting the vector-> {:?}",
        Rc::strong_count(&car_b)
    );
    // now we can do
    std::mem::drop(facility_one);
    println!(
        "references after deleting the vector-> {:?}",
        Rc::strong_count(&car_b)
    );
}

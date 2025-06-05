#![allow(unused)]

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Ant {
    name: String,
}

struct Spider {
    name: String,
}

//THINK ON TRAITS AS A SOME KIND OF INTERFACE
// the method can implemented or not
// if it is implemented
// the specific implementation of the struct
// can use it as default. check Ant or Spider impl
trait Animal {
    // fn talk(&self) -> String; // &self, allows the method to acces the data in the implemented strcut
    fn talk(&self) -> String {
        String::from("Cant talk!!") // default implementation
    }
}

impl Animal for Dog {
    fn talk(&self) -> String {
        String::from("DOg  makes sounds -- from Dog impl")
    }
}

// define a DEFAULT IMPLE FOR THOSE THAT HAVE SAME CODE
impl Animal for Ant {}

impl Animal for Spider {}

// TRAITS AS PARAMETERS AND TRAITS BOUNDS

// here we define the trait bounds - a trait as param
fn make_animal_talk(animal: &impl Animal) {
    // implement Animal traits instead of an concrete implementation
    println!(
        "imprimimos lo correspondiente implementado en el trait {}",
        animal.talk()
    );
}
// USANDO GENERICS
fn make_generic_talk<T>(animal: &T)
where
    T: Animal,
{
    println!("imprimimos usando generics {}", animal.talk());
}

// if we need  to pass more than one arg of the same type
fn make_animals_talk<T: Animal>(animal: &T, another_animal: &T) {
    // implement Animal traits instead of an concrete implementation
}

/*
we can use multiples TRAITS
fn make_generic_talk <T>(animal: &T)
where T: Animal + std::fmt::Display,{ <<---- here we add the Display trait
    println!("imprimimos usando generics {}", animal.talk());
}
or
fn make_animal_talk(animal: &(impl Animal + std::fmt::Display)){
    ....
}

*/

// Retuning values from TRAITS
// BOX ALLOWS MEMORY ALLOCATION ON THE HEAP

fn ret_val() -> impl Animal {
    Dog {
        name: "from_return".to_string(),
    }
}

fn ret_val_box(var: bool) -> Box<dyn Animal> {
    if var {
        Box::new(Dog {
            name: "Box_terri".to_string(),
        })
    } else {
        Box::new(Spider {
            name: "man".to_string(),
        })
    }
}
//blanket implementations
// He tostring is implemented on any type that implements Display
// impl<T: Display> ToString for T {
//     //
// }

// ASSOCIATED TYPES /////////////////////////////////////////////////////////
// similar concept like Generics but cant implement for more than one type
trait Incrementor {
    type Item; //In traits, type is used to declare an associated type
    fn increment(&mut self) -> Self::Item; // returns the value of that type
}

struct Counter {
    count: u32,
}

impl Incrementor for Counter {
    type Item = u32;
    fn increment(&mut self) -> Self::Item {
        self.count += 1;
        self.count
    }
}

// same as before but using generics
trait IncrementorT<T> {
    fn increment(&mut self) -> T; // returns the value of that type
}

struct CounterT<T> {
    count: T,
}

impl IncrementorT<u32> for CounterT<u32> {
    fn increment(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
    // impl IncrementorT<u16> for CounterT<u16> {
    //  ...
}

// RULE:
// FOR MULTIPLE CONCRETE TYPES -->> USE GENERICS
// ONLY FOR ONE TYPE -->> USE ASSOCIATED TYPES
// RUST allows  trait to have a method that shares the same name as a method from another trait
// it's also possible to implement a method directly on a type, with the same name as a method from another trait.

///// FULLY QUALIFIED SYNTAX /////////////////////////////////////////////////
trait File {
    fn write(&self);
}

trait Printer {
    fn write(&self);
}

struct Logger;

impl Logger {
    fn write(&self) {
        println!("write this from logger to screen...")
    }
}

impl File for Logger {
    fn write(&self) {
        println!("write this from File to screen On logger...")
    }
}

impl Printer for Logger {
    fn write(&self) {
        println!("write this from Printer to screen on Logger...")
    }
}
// SUPER TRAITS //////////////////////////////////////////////////
trait Speak {
    fn speack(&self);
}

trait Greet: Speak {
    fn greet(&self) {
        self.speack();
        println!("Hello from gree func on Greet Trait");
    }
}

struct Person {
    name: String,
}

impl Speak for Person {
    fn speack(&self) {
        println!("Hola desde Person strut imp..{}", self.name);
    }
}

impl Greet for Person {} // we dont need to implement nothing here
                         // because Greet has an Default implementation

//OPERATOR OVERLOADING //////////////////////////////////////////////////////
use std::{fmt::Display, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    // with this we can pass an I32 or f64
    longitude: f64,
    latitude: f64,
}

// here we overwritten and we can add two Coordinate types using the "+" sign
// check Add on std library to see the RHS behaviour
impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        //rhs = right hand side
        Coordinate {
            longitude: (self.longitude + other.longitude),
            latitude: (self.latitude + other.latitude),
        }
    }
}

/////// NEW TYPE PATTERNS
/// wrapping a single value of an existing type in a new type
/// you can use the newtype pattern to create a newtype that wraps the external type, and then you implement your trait for that newtype. This way, you can effectively implement the desired trait without breaking the orphan rule. And, if you include that external crate, it isn't going to break your code.
///
struct Wrapper(Vec<String>); //we define a new type for the  external type Vec

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let dog = Dog {
        name: "NAME".to_string(),
    };

    let ant = Ant {
        name: "Ant NAME".to_string(),
    };

    let spider = Spider {
        name: "Spider NAME".to_string(),
    };

    // println!("{} -->> {}",dog.name, dog.talk());
    // println!("{} -->> {}",ant.name, ant.talk());
    // println!("{} -->> {}",spider.name, spider.talk());
    // make_animal_talk(&dog);
    // make_generic_talk(&spider);

    // // reutning values
    // return type impl of Animal trait we can only use if there is one returning type
    // because of the compiler restrictions
    let animal = ret_val();
    println!("from ret_val {}", animal.talk());

    // for more than one option of returning type we need to use BOX
    // this bring overhead on the proccesing
    let dog = ret_val_box(true); //

    // associated types
    let mut couter = Counter { count: 0 };
    // for _ in 0..10 {
    //     println!("Counter: {}", couter.increment());
    // }

    //FULLY QUALIFIED SYNTAX
    let log = Logger;
    // calls the write method from logger
    log.write();
    // if we need a specific implementation we use
    File::write(&log);
    Printer::write(&log);
    <Logger as Printer>::write(&log);

    //////////// check equality using add method overwritten
    assert_eq!(
        Coordinate {
            latitude: 1.0,
            longitude: 2.0
        } + Coordinate {
            latitude: 3.0,
            longitude: 4.0
        },
        Coordinate {
            longitude: 6.0,
            latitude: 4.0
        }
    );
    //SUPERTRAITS ...
    let p1 = Person {
        name: "Vale".to_string(),
    };
    p1.greet(); // calls speak an greet

    /// New TYPE PATTERNS
    let wrapper = Wrapper(vec![String::from("hello"), String::from("rustaceans!!")]);
    println!("wrapper -> {}", wrapper);
}

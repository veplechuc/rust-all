// we need to tell the compiler that T implements Display in order to be printed
use std::{fmt::Display, i32};

use guide::file_mod1::printin;

fn print_data<T: std::fmt::Display>(vec: &[T], label: &str) {
    //for multuple generic params use <T, U, V,...
    println!("{}", label);
    for elem in vec {
        println!("{}", elem);
    }
}
//Another way is
fn print_data2<T>(vec: &[T], label: &str)
where
    T: std::fmt::Display + PartialOrd,
{
    //for multuple generic params use <T, U, V,...
    println!("{}", label);
    for elem in vec {
        println!("{}", elem);
    }
}

//GENERICS WITH STRUCTS

struct Coordinate<T, U> {
    // with this we can pass an I32 or f64
    longitude: T,
    latitude: U,
}

//GENERICS WITH MERHIDS

impl<T: std::fmt::Display, U: std::fmt::Display> Coordinate<T, U> {
    fn display(&self) {
        println!(
            "location latitude: {}, longitude: {}",
            &self.latitude, &self.longitude
        );
    }
}

impl Coordinate<f64, f64> {
    fn dist(&self, other: &Coordinate<f64, f64>) -> f64 {
        let long = &self.longitude + &other.longitude;
        let lati = &self.latitude + &other.latitude;
        long + lati
    }
}

// GENERICS ON ENUMS
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Result::Err(String::from("error division by zero"))
    } else {
        Result::Ok(a / b)
    }
}

//////////////
/// 67  +
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// genereics functions
fn procces<T>(arr: &[T]){

}

//Type constrains on functions
// as an example we try to print a element but the fucntion 
// receives a Generic type.. so the compiles doe not know 
// if the element implements the Display/ Debug trait
//


fn printcons<T:std::fmt::Debug>(arr: &[T]){
    for el in arr{
        println!("element -->{:?}", el);
    }

}

fn main() {
    let p1 = Point { x: 5, y: 10.0 };
    //explicity the type
    
    let p2 = Point { x: "hello", y: 'c' };
    let p4 = Point { x: "hello", y: 'c' };
    let points = vec![&p2, &p4];

    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    printcons(&points);

    let location1 = Coordinate {
        latitude: 3.100,
        longitude: 18.889,
    };

    let location2 = Coordinate {
        latitude: 3.100,
        longitude: 5,
    };

    let location3 = Coordinate {
        latitude: 3.100,
        longitude: 5.558,
    };

    // location1.display();
    // location2.display();
    println!("total {}", location1.dist(&location3));
    let result = divide(5.6, 3.4);
    match result {
        Result::Err(err) => println!("Error: {}", err),
        Result::Ok(res) => println!("Error: {}", res),
    }

    // dynamic dispatch
    // So we create &12 a reference to the number 12 and then we immediately transform it using the
    // ASCII word into &Display. So it's now lost all its nunmberness and now is only a display type.
    // And again we do the same thing with the string it loses all its stringiness and just becomes
    // a Display type according to the compiler.

    let v = vec![&12 as &dyn Display, &"Hi" as &dyn Display];
    show(v);

    //
}

//  static dispatch
// foo<T>(val:T) -> T this will generate for example
// foo_i8(i8)-> i8
// foo_i32(i32) -> i32
// base on what it is called

fn show(v: Vec<&dyn Display>) {
    for item in v {
        println!("dynamic -> {}", item)
    }
}


// closure on generics
// Standard Traits FnOnce, FnMut, Fn
// Jerarquicamente 
// FnOnce
//   |
// FnMut
//   |
//  Fn


//FnOnce ... Any closure is called at least once,can only be called once
fn func(){
    let v = String::from("Hola");
    let clo = || { 
        print!("{}", v);
        std::mem::drop(v); // <-- this drops the ownerships
    };
    clo();
    // clo(); closure cannot be moved more than once
}


//FnMut .... a closure that captures an external state mutably

fn fnmu(){
    let mut v =  String::from("Hola");
    let mut clo = || { 
        v.push_str("Some string");
    };
    clo();

}
//Fn ....... a closure that captures a state inmutably

fn fnclo(){
    let  v =  String::from("Hola");
    let clo = || { 
        print!("{}", v);
    };
    clo();
    clo(); //<-- we can call more than once because we are not changing anything just using
}


// passing to a function... 
// According th the Hierarchy this function could take 
fn receive_fnonce<F>(func: F) where F: FnOnce() {
    func();
}
// we can pass here any closure that implements FnOnce
// this means that to this function we can pass a FnOnce, FnMut or Fn


// for FnMut
fn receive_fnmut<F>(mut func: F) where F: FnMut() {
    func();
    func();
}

// for Fn
fn receive_fnm<F>(func: F) where F: Fn() {
    func();
    func();
}
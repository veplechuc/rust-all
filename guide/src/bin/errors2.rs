use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;

// Errors should be Enums
//group errors if they make sense about their origin
#[derive(Debug)]
#[non_exhaustive] // this lines forces to the to add the branch  for _ => when matching
pub enum SpecialError {
    // Error defined as Enum
    VerySpecial(i16), // group the Errors
    KindOfSpecial,    // expose your own error (exception std library error)
}

impl Display for SpecialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use SpecialError::*;
        match self {
            VerySpecial(n) => write!(f, "Some msg from Very Special error number {}", n),
            KindOfSpecial => write!(f, "Other msg from Kind Of error "),
        }
    }
}

impl Error for SpecialError {}

// same Example using thiserror

use thiserror::Error;
#[derive(Debug, Error)]
#[non_exhaustive] // this lines forces to the to add the branch  for _ => when matching
pub enum SpecialError2 {
    #[error("Some msg from Very Special error number {0}")]
    VerySpecial(i16),
    #[error("Other msg from Kind Of error ")]
    KindOfSpecial,
}

fn main() {
    // Error non recoverable
    panic!("some message that means panic!!"); // this should be avoided, last resource

    // if the result is type Resul::Err
    // result.expect("some error ");

    // not use the msessage at all
    //result.unwrap

    // Errors recoverable you return or hanlde the error

    //Handling
    //if let Err(e) = some_result {println!("Some warning {}", e)}
    // .. continue with your logic
    // or
    //return same type when error occurs when return some default
    // let result =  match compute_some_ints(){
    //      Ok(x) => x,
    //      Err(_)=> 0
    // };

    // let result = compute_some_ints().unwrap_or(0); //same but in one line

    // if you dont know how to handle and need to return the error to the caller

    // fn my_func() -> Result<String, io::Error> {
    //     let file = match File::open("some_file.txt") {
    //         Ok(f) => f,
    //         Err(e) => return Err(e),
    //     };
    //     // continue working with the file
    // }

    // the previous code could be simplified using the "?" try opperator
    // fn my_func() -> Result<String, io::Error> {
    //     let file = File::open("some_file.txt")?;
    //     // continue working with the file
    // }

    // we can chain the  ? if all func return an error compatible with the func signature
    // pub fn roll_figure()-> Result<Myfigure, FigureError>{
    //     let fig = Figure::new();
    //     fig.rotate()?.paint()?.display()?;
    // }

    /////////////////////  MORE ON ERRORS
    // if let Ok(value) = a_function_that_can_error() {
    // something with the value
    // }

    // match a_function_that_can_error() {
    //     Ok(value) => println!("oh yeah, value! {}", value);
    //     Err(e) => eprintln!("ohh no... {}", e);
    // }

    // // you don't care about the error
    // _ = a_function_that_can_error();

    // // yolo
    // let foo = a_function_that_can_error().unwrap();

    // // respectful yolo
    // let foo = a_function_that_can_error().expect("should never fail");

    // // defaults
    // let foo = a_function_that_can_error().unwrap_or(0);

    // // convert to option
    // // Ok(V) => Some(V)
    // // Err(E) => None
    // // bai felicia
    // let foo = a_function_that_can_error().ok();

    // let foo = a_function_that_can_error()
    //     .map(|value| value + 1);

    // let foo = a_function_that_can_error()
    //     .and_then(|value| another_possible_error(value))
    //     .and_then(|value| again(value));

    // // If your function returns an error, you can do this!
    // let foo = a_function_that_can_error()?;
}

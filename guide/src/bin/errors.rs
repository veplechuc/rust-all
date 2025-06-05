use std::{num::ParseIntError, vec};

fn to_int(s: &str) -> i32 {
    // s.parse().unwrap() // parse returns a -> Result<F, F::Err>
    // Result has a -> unwrap if result returns an ok
    // it returns that value, if not, the program panics and fails.

    // ---------------------
    //variant

    // s.parse()
    //     .expect(("algo paso con el parseo ".to_string() + s).as_str()) // on an error returns a message
    // ---------------------
    //
    s.parse().unwrap_or(0) //Returns the contained [Ok] value or a provided default.
}

fn sum(strs: Vec<String>) -> String {
    let mut ac = 0;
    for s in strs {
        ac += to_int(&s);
    }
    ac.to_string()
}

/////////////// Returning Result or Option
fn to_int_op(s: &str) -> Option<i32> {
    s.parse().ok() //Converts from Result<T, E> to Option<T>
}

fn sum_option(strs: Vec<String>) -> String {
    let mut ac = 0;
    for s in strs {
        // using PATTER MATCHING
        // ac += match to_int_op(&s) {
        //     Some(val) => val,
        //     None => 0,
        // }
        //using IF LET
        // if let Some(val) = to_int_op(&s) {
        //     ac += val
        // }
        // using UNWRAP OR unwrap_or
        ac += to_int_op(&s).unwrap_or(0);
    }
    ac.to_string()
}

//returning an Option
fn sum_option2(strs: Vec<String>) -> Option<String> {
    let mut ac = 0;
    for s in strs {
        ac += to_int_op(&s)?; // ? -> propagates the None
    }
    Some(ac.to_string())
}

fn sum_option3(strs: Vec<String>) -> Result<String, SumError> {
    let mut ac = 0;
    for s in strs {
        //ok_or -> Transforms the Option<T> into a Result<T, E>, mapping [Some(v)] to [Ok(v)] and None to [Err(err)].
        ac += to_int_op(&s).ok_or(SumError)?;
    }
    Ok(ac.to_string())
}

#[derive(Debug)]
struct SumError;

// propagating the error

fn to_int_resu(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

fn sum_option4(strs: Vec<String>) -> Result<String, SumError> {
    let mut ac = 0;
    for s in strs {
        // Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched
        ac += to_int_resu(&s).map_err(|_| SumError)?;
    }
    Ok(ac.to_string())
}

fn main() {
    using_option();

    // let v = vec![String::from("3"), String::from("4")];
    // let total = sum(v);

    // println!("resultado suma usando sum -> {:?}", total);

    // let v = vec![String::from("3"), String::from("ab4"), String::from("9999")];
    // let total = sum(v);
    // println!("resultado suma usando sum - con error -> {:?}", total);

    // let v = vec![String::from("3"), String::from("4")];
    // let total = sum_option2(v);
    // // println!("resultado otra suma usando option2 OK -> {:?}", total);
    // if let Some(z) = total {
    //     println!("using if let -> {:?}", z)
    // }

    // let v = vec![String::from("3"), String::from("ab4"), String::from("4")];
    // let total = sum_option2(v);
    // println!("resultado otra suma usando option2 None -> {:?}", total); // we expect a NONE

    // let v = vec![String::from("3"), String::from("4")];
    // let total = sum_option3(v);
    // //println!("resultado otra suma usando option3 SumError -> {:?}", total); // we expect a SumErr
    // match total {
    //     Ok(z) => println!("using patter matching  en sum option3 -> {:?}", z),
    //     Err(e) => println!("Error occurs {:?}", e),
    // }

    // let v = vec![String::from("3"), String::from("a4")];
    // let total = sum_option3(v);
    // println!("resultado otra suma usando option3 Ok -> {:?}", total); // Ok(7)

    let v = vec![String::from("3"), String::from("ab4")];
    let total = sum_option4(v);
    println!("resultado otra suma usando option4 SumErr -> {:?}", total);

    let v = vec![String::from("3"), String::from("4sss")];
    let total = sum_option4(v);
    println!("resultado otra suma usando option4 Ok -> {:?}", total); // Ok(7)
    match total {
        Ok(z) => println!("using patter matching  en sum option$ -> {:?}", z),
        Err(e) => println!("Error occurs {:?}", e),
    }
}

fn using_option() {
    let x = 2.0;
    let y = 0.0;

    let xx = x / y;

    println!("{}", xx);

    let result = if y != 0.0 { Some(x / y) } else { None };

    println!("value of result -> {:?}", result);

    // if we want the value
    match result {
        Some(z) => println!("using patter matching -> {:?}", z),
        None => println!("Not permited divided by zero"),
    }
    // another way -> if let / while let
    if let Some(z) = result {
        println!("using if let -> {:?}", z)
    } // if the distructuring (let Some(z) = result) works it will print the result
}

// pub enum Result<T, E> {
//     // explicit defines success and error states
//     /// Contains the success value
//     Ok(T), // tuple struct
//     /// Contains the error value
//     Err(E), // tuple struct
// }

// pub enum Option<T> {
//     None,    // struct no fields
//     Some(T), // tuple struct
// }

// unwrap takes the value of the container and returns it, else you can specify what to do or panic!

// alg.ok converts between Result y Options

// algo.map convert the types of a container ex. Result witn an error type passed to another
// Result with another error type

// ? operator, propagates the ausence of a value or the success.
// in the Option case, the Err type does not needs to be the same,
// for Result they do

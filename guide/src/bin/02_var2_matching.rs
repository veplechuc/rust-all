#![allow(unused)]

use rand::Rng;
use std::{cmp::Ordering, io, result};

fn main() {
    println!("Hello, world!");
    let x = 5; // variables ARE inmutable!!!
    println!("{}", x);

    // x = 10; //error because it is inmutable

    let x = "six"; // use "" for str

    let xy = 'd'; // use '' to char
    let r = String::from("Hola");
    let _variable: i128; // '_' previene que el compilador diga que no esta usada

    println!("{}", x); //

    const DATA: i32 = 600_000; // const MUST be annotated can not be mutable

    let dat: u8 = 255; //when it reaches the maximun values then 256 -> 0, 257 -> 1 .....

    println!("{}", dat);

    let s = "hello there"; //&str is a string slice - can not index
    for c in s.chars().rev() {
        println!("{}", c)
    }

    if let Some(c) = s.chars().nth(0) {
        println!("first char of str -> {}", c)
    }

    let str = String::from("string");
    let tr = str; // MOVES THE VALUE STR CAN NOT BE USED ANYMORE!!!

    let str = String::from("another");
    let tr = str.clone(); // need to clone it in order to have a copy

    // String is mutable most types does not have Copy trait dont copy the value it moves
    let mut str = String::from("string");
    str.push_str("...another"); // str = "string...another"

    // ------------------- BORRROWING
    //implicit
    let s = String::from("hola");

    let st = &s; // borrows but does not owns s

    //-------- explicit
    let s = String::from("hola");
    let st: &String = &s; // borrows but does not owns s-- I wrote &String

    //borrow it is like  when passed an objec to a function as reference

    let mut s = String::from("hola");
    let st = &mut s; // borrows but does not owns s
                     // st is mutable
    st.push('g'); // ue '' for chars

    // compound types
    let tup = ("some tuple", 120);
    let tup2 = (String::from("hola"), 12.3);
    println!("{}", tup.0);
    println!("{}, {}", tup2.0, tup2.1);

    let (tup1, tup11) = tup;

    let assigned_by_adot = tup.1;

    println!("tup.0 -> {}--- tup1->{} tup11->{}", tup.0, tup1, tup11);

    // ---------- ARRAYS --------------

    let arr = [1, 2, 3];

    //let arr2: [i32; 3] = [1,2,3,4];

    //println!("{}", arr2);

    let arr1 = [1; 5]; // [1,1,1,1,1,]
    for x in arr1 {
        print!("{}", x);
    }

    // ARRAYs TUPLES
    let mut lista = [1, 2, 3]; // use mut to change the values of the array

    let mut vec = Vec::<i32>::new();
    let mut vec2: Vec<i32> = lista.to_vec();

    vec2.push(1000);
    println!("{:?}", vec2);

    vec.push(100);

    //vec.pop... or vec[0] .. vec.len()
    let index = 3;
    let op = vec.get(index);
    match op {
        Some(value) => println!("something"),
        None => {}
    }

    // FUNCTIONS snake case name
    let sum = suma(22, 55);
    println!("resultado de la suma {}", sum);
    println!("resultado de la suma {}", suma(55, 77));

    // control flow
    if sum < 10 {
        // condition must be explicity a boolean -> ex:  if number {} error
        println!("sum es menos que 10"); // expresion must evaluate to boolean
    } else if sum > 900 {
        println!("sum es mayor que 900");
    } else {
        println!("sum esta entre 10 y 900");
    }

    // ternary
    let condition = true;
    let numb = if condition { 7 } else { 10 };

    //loop
    // loop {
    //     println!("inside loop");
    //     break;
    // }

    // break uses conditional names to breaks
    // same for continue
    // 'bob: loop {
    //     loop {
    //         loop {
    //             break 'bob;
    //         }
    //     }
    // }

    let mut coun = 0;

    let result = loop {
        coun += 1;
        if coun == 10 {
            break coun;
        }
    };
    println!("valor de coun {}", coun);
    println!("valor de coun {}", result);

    let ele = [1, 2, 3, 4, 5];

    println!("using loop with iter ");
    for el in ele.iter() {
        println!("{}", el);
    }

    println!("using loop with for.. in .. ");
    for n in 1..10 {
        // last number is exclusive
        println!("{}", n);
    }

    let mut num = 1;
    println!("using loop while..");
    while num != 0 {
        num = num - 1;
        println!("{}", num);
    }

    let x = &[1, 2, 4];

    /* multiline comment
       this is a multiline
       comment
    */

    //labeld loop;

    // 'outer: loop {
    //     loop {
    //         break 'outer;
    //     }
    // }
    pattern();

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You just gained the right to vote!"),
    };

    //DANGLE REFERENCE  // UNCOMMENT THIS CODE TO CHECK
    //let reference_to_nothing = dangle();
}

//Pattern matching

fn pattern() {
    let num = -1;
    match num {
        100 => println!("one hundred"),
        27..=50 => println!("between 25 and 50"), // for range
        25 | 26 => {
            // more than one sentence
            println!("25 or 26");
            println!("not sure!")
        }
        _ if num > 0 => println!("sure is positive!"), // conditional

        _ => println!("I dont know .. and I dont care!"), // no matches
    }
    // using other options
    // var => passing_to_func(var); <-- passing the values that matches to the function
    // _ => (); <-- do nothing if this is just one case use .. if let

    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }

    // with conditional
    let tup = (2, -2);
    println!("Let's compare {:?} ", tup);
    match tup {
        (x, y) if x == y => println!("equals..."),
        (x, y) if x + y == 0 => println!(" x = {x} + y = {y} == {}", x + y),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    // using patter and if let for one single case
    let y: Option<i32> = None;

    let conf = Some(3u8);
    match conf {
        Some(val) => println!("obtained value -> {}", val),
        _ => (),
    }

    // let conf = Some(3u8);
    // if let Some(val) = conf {
    //     println!("obtained value -> {}", val);
    // }
}

fn suma(x: i32, y: i32) -> i32 {
    println!("algo{} ", x);
    x + y // or  "return x + y;"
}

// formatting code run cargo fmt
// use rustfmt name_file.rs

// rust dont throws exceptions!!!

//

// portability use i32 or i16 let x: i32
// for velocity use isize -- let x: isize
// {:>10} 10 spaces in a field right aligned
// {:<10} to the left

// println!("{:<10}");
// print!("{:*>10}");

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

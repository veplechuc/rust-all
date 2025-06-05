#![allow(unused)]
#![allow(clippy::let_unit_value)]

/* Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
Unlike functions, closures can capture values from the scope in which they’re defined. */

// FnOnce applies to closures that can be called once.
// All closures implement at least this trait, because all closures can be called.
// A closure that moves captured values out of its body will only implement
// FnOnce and none of the other Fn traits, because it can only be called once.

// FnMut applies to closures that don’t move captured values out of their body,
//but that might mutate the captured values. These closures can be called more than once.

// Fn applies to closures that don’t move captured values out of their body and
// that don’t mutate captured values, as well as closures that capture nothing from their environment.
//These closures can be called more than once without mutating their environment,
//which is important in cases such as calling a closure multiple times concurrently.

// Closures automatic borrow a reference to in scope variables
// closures  basically is
// let add  = |x, y| {x+y}

use std::{fmt::format, vec};

fn main() {
    //// --------------- Fn --------------
    // Create a closure that defines if someone can vote
    let can_vote = |age: i32| age >= 18;
    println!("Can vote : {}", can_vote(8));

    let add_one = |x: i32| x + 1;
    println!("{}", add_one(3));

    // closures takes the type of the value of the first call
    let double = |num| num * 2;
    let x = 10;
    double(x);

    // to uncomment these lines we need to comment the x definition
    // let y = 10.5;
    // double(y);

    //// --------------- FnMut --------------

    // borrowing on closures -- implement FnMut
    let mut vecto = vec![12, 3, 5, 7];

    let mut var_borrow = || vecto.push(8);

    // uncommento to check the error
    //println!("{:?}", vecto);

    var_borrow();

    println!("{:?}", vecto);

    // ----------------------------------
    #[derive(Debug)]
    struct Rect {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rect {
            width: 10,
            height: 1,
        },
        Rect {
            width: 3,
            height: 5,
        },
        Rect {
            width: 7,
            height: 12,
        },
    ];
    // sort_by_key calls the closure multiple times
    let mut counter = 0;
    list.sort_by_key(|r| {
        counter += 1;
        r.width
    });
    println!("{:#?}", list);
    println!("was called -> {:?} times", counter);

    // ------------------------------
    let mut sort_operations = vec![];
    let value = String::from("by key called");

    // sort_by_key calls the closure multiple times
    list.sort_by_key(|r| {
        sort_operations.push(&value); // could implement clone() ?
        r.width
    });
    println!("{:#?}", list);
    println!("{:?}", sort_operations);

    // ----------------------------------

    let s = "🤔";
    let s2 = s.clone();
    let f = || {
        println!("dentro del closure f valor de s --> {}", s); // at this point s get drop
    };
    f();
    println!("despues de llar a f valor de s --> {}", s);

    // in this case we pass the var takes ownership
    // so we can pass the value to another thread without worry about the
    // scope of s
    let f1 = move || {
        println!("closure move --valor de s --> {}", s);
    };
    println!("despues de f1 valor de s --> {}", s);
    f1();
    println!("valor clonado de s --> {}", s2);

    // ---------------------------------------------

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

    //concatenate closures
    let add = |n1, n2| n1 + n2;
    let (from, to) = (1, 10);
    // no need to pass the from and to values
    // from an to are inferred from the scope
    let sum_range = || {
        let mut sum = 0;
        for n in from..to {
            sum = add(sum, n);
        }
        sum
    };

    println!("sum = {}", sum_range()); // we call the closure

    //////  a shorter version
    let suma = (1..10).fold(0, |ini, n| ini + n);

    println!("suma = {}", suma);

    // explicitly define the type for resu or put .sum::<i32>()
    let resu: i32 = (1..100).filter(|n| n % 2 == 0).sum();
    println!("odd numbers sum = {}", resu);

    let mut vect = vec![1, 2, 3];
    let mut some_closure = || {
        vect.push(10);
        println!("{:?}", vect)
    };

    //println!("{:?}", vect);
    some_closure();

    //// -----------------------------------------
    let mut arr2 = [1, 2, 3];

    let double = arr2.iter().map(|x| x * 2);
    double.for_each(|x| println!("double ->{}", x));

    // take into account that |&&x| is a double referencia because of the iter and filter
    let filtered = arr2.iter().filter(|x| **x > 2);
    println!("filtered -> {:?}", filtered);
    // FUNCTION TYPES

    let resp = repeat(add_val, 50);
    println!("value of repeat ->{}", resp);

    ////////////// ITERATOR
    fun_iterator();

    struct ClassicCars {
        make: &'static str,
        models: Vec<(&'static str, i32)>,
    }
    impl ClassicCars {
        fn smart_get<T>(&self, f: T)
        where
            // here define what type is  function passed in with the Fn trait
            T: Fn(&Vec<(&'static str, i32)>),
            //we are saying that the function handles this &Vec<(&'static str, i32)> type of values
        {
            f(&self.models) // calling the function and passing in the vector models
        }
    }

    let car_collection = vec![("Auto_uno", 1970), ("Auto_dos", 1980), ("Auto_tres", 2000)];
    let modelos = ClassicCars {
        make: "Some_Model",
        models: car_collection,
    };
    modelos.smart_get(|x| {
        let res: Vec<_> = x.iter().filter(|x| x.1 > 1970).collect();
        println!("{:?}", res);
        let res: Vec<_> = x.iter().map(|x| format!("{} _ {}", x.0, x.1)).collect();
        println!("{:?}", res);
    });

    println!("---- another iter ----- ");
    another_iter();
}

fn fun_iterator() {
    // reads a text file from disc,
    // skips 2 lines and takes the consecutives 4
    // filter the lines with even idx
    let file = std::fs::read_to_string("guide/src/texto.txt").unwrap();
    file.lines()
        .enumerate()
        .skip(2)
        .take(4) // this returns a tuple
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));

    let a = 10;
    let b = 0;

    let suma: i32 = (a..=b) // is 0 if start > end
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("{}", x)) // use to check whats happens(debuging)
        .sum();

    println!("{:?}", suma);

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    };
    // writting add_one with closure
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    println!("add one written as a closure.. {}", add_one_v4(5));
}

// You can pass closures to functions
fn use_func<T>(a: i32, b: i32, func: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    func(a, b)
}

// FUNCTION TYPES
fn add_val(a: i32) -> i32 {
    a + 10
}
// specifies the type an return value of the func when passed in as param
fn repeat(fun: fn(i32) -> i32, arg: i32) -> i32 {
    fun(arg) + fun(arg)
}
////////////////////////////
fn iter_ator() {
    // ITERATORS -----------------

    let v = vec![2, 3, 4, 5];

    let odd_total = v
        .into_iter() //iter adaptor
        .map(|x| x * 3) // iter adaptor
        .filter(|y| *y % 2 == 0) // iter adaptor
        .for_each(|z| println!(" valor de z --> {}", z));
    // ends of those iterators adapters with an iterator consumer
    // example .sum()
    // or add a "turbofish '::<>' " sum::<i32>() sintax
    // another iter consumer -> collect() put all the items in a new collection
    // v.iter() ... iterate over the collection inmutables references for num in v.iter() - or  for _ in &v and dont change the value
    // v.iter_mut() .. iterate over the collection of mutables references for num in v.iter_mut() or for _ in &mut v and change the value
}

//sum of squares of all odd numbers from 1..10 except those squares
// divisible by 5
fn another_iter() {
    let dat = (1..10)
        .filter(|x| x % 2 == 1)
        .map(|x| x * x)
        .filter(|x| x % 5 != 0)
        .inspect(|x| println!("{}", x))
        .sum::<i32>();
    println!("resul of apply funcs -> {}", dat);
}

fn factorial(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }
    factorial(n - 1, r * n)
}

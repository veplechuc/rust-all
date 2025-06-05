//use crossbeam::thread;
use log::{error, info};
use std::{sync::mpsc, thread, time::Duration};

fn sleep(seconds: f32) {
    thread::sleep(Duration::from_secs_f32(seconds));
}

pub mod dad {
    use super::{info, sleep};

    pub fn cook_spaghetti() -> bool {
        info!("Cooking the spaghetti...");
        sleep(15.0);
        info!("Spaghetti is ready!");
        true
    }
}

pub mod mom {
    use super::{info, sleep};

    pub fn cook_sauce_and_set_table() {
        sleep(1.0);
        info!("Cooking the sauce...");
        sleep(2.0);
        info!("Sauce is ready! Setting the table...");
        sleep(2.0);
        info!("Table is set!");
    }
}

fn main() {
    env_logger::init();
    let handle = thread::spawn(|| dad::cook_spaghetti());

    mom::cook_sauce_and_set_table();

    if handle.join().unwrap_or(false) {
        //< -- wait the thread to finish
        info!("Spaghetti time! Yum!")
    } else {
        error!("Dad messed up the spaghetti. Order pizza instead?");
    }

    // same code as before but using pattern matchig
    // match handle.join() {
    //     Ok(_) => println!("exists ok"),
    //     Err(_) => println!("error ocurrs")
    // }

    //multithreading

    //define a vec of threads
    let var = 32;

    let mut vec_t: Vec<thread::JoinHandle<_>> = vec![];
    for _ in 1..5 {
        let handl1 = thread::spawn(|| dad::cook_spaghetti());
        vec_t.push(handl1);
    }

    for t in vec_t {
        match t.join() {
            Ok(_) => println!("exists ok"),
            Err(_) => println!("error ocurrs"),
        }
    }
}

// rust does not allow capture external state by borrowing

// fn ext() {
//     let x = 5;

//     thread::spawn(|| {
//         println!("{}", x);
//     });
// }
// this func took x by borrowing and if the fun ext finishis before the thread
// x will be refering to an dangling reference to x
fn ext() -> thread::JoinHandle<()> {
    let x = 5;

    thread::spawn(move || {
        //<-- we need to move the value
        println!("{}", x);
    })
}
//this works because the compiler knows that the ownership should be moved to the
// for loop
fn ext1() -> thread::JoinHandle<()> {
    let ve = vec![1, 2, 3];

    thread::spawn(|| {
        for ite in ve {
            println!("{}", ite);
        }
    })
}
// we can borrow the ve value but need to use move
fn ext2() -> thread::JoinHandle<()> {
    let ve = vec![1, 2, 3];

    thread::spawn(move || {
        for ite in &ve {
            println!("{}", ite);
        }
    })
}

//multiple access to a data  between threads

// single message

fn single() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        tx.send(String::from("Hola from thread..")).unwrap();
    });
    let resp = rx.recv().unwrap();
    println!("{}", resp);
    match handle.join() {
        Ok(_) => println!("go nice!"),
        Err(_) => println!("found some error"),
    };
}

// multiple

// single message

fn multiple() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for item in 1..=5 {
            let msg = std::format!("message.. {}", item);
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(5));
        }
    });
    for item2 in rx {
        println!("{}", item2);
    }
    // match handle.join() {
    //     Ok(_) => println!("go nice!"),
    //     Err(_) => println!("found some error")

    // };
    handle.join().unwrap();
}

// Other example but in main

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();   // to be written after thread 2 code

//     thread::spawn(move || {
//         let my_vec = vec![1, 2, 3, 4, 5];
//         for i in my_vec {
//             tx.send(i).unwrap();
//             thread::sleep(Duration::from_secs(1));

//         }
//     });

//     thread::spawn(move || {
//         // will get an error here stating that tx has been moved.
//         let my_vec = vec![6, 7, 8, 9, 10];
//         for i in my_vec {
//             tx1.send(i).unwrap();  // later on change to tx1
//             thread::sleep(Duration::from_secs(1));

//         }
//     });

//     for recieved_vals in rx {
//         println!("I recieved the value of {}", recieved_vals);
//     }
// }

// 			Sharing States
// -------------------------------------------
use std::sync::Mutex;
fn main1() {
    let m = Mutex::new(5);

    /*
    {
        let mut num = m.lock().unwrap();
        *num =  10;
    }

    println!("m = {:?}", m);
    */

    let mut num = m.lock().unwrap();
    *num = 10;
    drop(num); // we need to drop the value if not it will hangs becouse the nex lock is
               // waiting for this one to releases

    let mut num1 = m.lock().unwrap();
    *num1 = 15;
    drop(num1);
} //

// Example 2: A nice usecase of the Arc for cloning types which are not clone (does not implement clone)

use std::sync::Arc;

struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

fn main2() {
    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("Rust"));

    for i in 0..5 {
        let some_str = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", some_str.0, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join();
    }
}

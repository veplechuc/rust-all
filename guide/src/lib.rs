#![allow(dead_code)]
// must refer to a file
// rust will try to find a file_mod1.rs  or a file_mod/mod.rs dir

// use dir/mod.rs when we have a module with submodules

// on mod.rs we define a module definition/func or whatever and otrher modules references

//example inside of some_dir/mod.rs

// fn some_fn(){}
// mod otro;
// mod otro_mas;

//  inside lib.rs
//  mod some_dir;
// ...
//  mod main_mod {}


// project/
// ├── Cargo.toml
// ├── some_dir/
// │   ├── mod.rs
// │   └── otro.rs
// │   └── otro_mas.rs
// │
// ├── lib.rs

// use module.rs if only have a simple

pub mod file_mod1;
// this case refers to a folder an inside a mod.rs
pub mod maths;

pub mod my_mod;

#[path = "bin/another_mod/math2.rs"]
pub mod math2;

pub mod structs_def;

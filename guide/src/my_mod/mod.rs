#![allow(dead_code)]

// Creates a module
// Contains other modules which hold functions,
// structs, enums, constants, traits
// You use modules to organize your code and to make
// parts of it private (Everything is Private by Default)
// Parent modules can't access private items in child modules
// but children can always access parent items
pub fn func1() {
    println!("Function 1 from my_mod");
}

pub mod my_mod1 {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }
    // nested module
    pub mod nested {
        //define more func
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }
}

mod my_mod2 {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

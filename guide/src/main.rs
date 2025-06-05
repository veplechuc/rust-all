use std::io;

fn main() {
    println!("Hello, world!");
    let x: i32 = 5; //variables ARE inmutable!!!;
    println!("{}", x);

    // x= 10; error because it is inmutable

    let x = "six"; // shadowing variable x

    let _variable: i128; // '_' prevent the compiler complains about unused variable
    let gg = 66;

    println!("{}", x);

    //// read from console and prints the value
    let mut input = String::new();
    println!("Enter a value:");
    io::stdin().read_line(&mut input).expect("Failed to read!!");
    println!("the input -> {}", input);

    //using matching pattern
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("the input -> {}", input)
        }
        Err(error) => println!("error: {error}"),
    }

    println!("Value 1 % 2 {}", 1 % 2);
}

// MANAGING RUST VERSIONS

// A particular version of Rust (and it's associated version of the standard library and other tools) is called a toolchain. rustup can manage multiple toolchains on your machine at once. https://rust-lang.github.io/rustup/concepts/toolchains.html Then, you can use directory overrides (https://rust-lang.github.io/rustup/overrides.html#directory-overrides)
// to customize which versions of Rust you use locally, To use to a specific nightly for a directory:
// rustup override set nightly-2014-12-18
// Or a specific stable release:
// rustup override set 1.0.0
// To see the active toolchain use "rustup show" To remove the override and use the default toolchain again,
// "rustup override unset"

// specific version of Rust (https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)

// MAIN STRUCTURE OF RUST PROJECT
// .
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src/
// │   ├── lib.rs
// │   ├── main.rs
// │   └── bin/
// │       ├── named-executable.rs
// │       ├── another-executable.rs
// │       └── multi-file-executable/
// │           ├── main.rs
// │           └── some_module.rs
// ├── benches/                         this is for benchmarking propouse
// │   ├── large-input.rs
// │   └── multi-file-bench/
// │       ├── main.rs
// │       └── bench_module.rs
// ├── examples/
// │   ├── simple.rs
// │   └── multi-file-example/
// │       ├── main.rs
// │       └── ex_module.rs
// └── tests/
//     ├── some-integration-tests.rs
//     └── multi-file-test/
//         ├── main.rs
//         └── test_module.rs

// Another way of having multiple main

// project/
// ├── Cargo.toml
// ├── app1/
// │   ├── Cargo.toml
// │   └── src/
// │       └── main.rs
// ├── app2/
// │   ├── Cargo.toml
// │   └── src/
//           └── main.rs

// define in Cargo.toml

// [workspace]
// members = [ "app1", "app2"]

// running cargo build will generate
// Compiling app1 v0.1.0 (/path/project/app1)
// Compiling app2 v0.1.0 (/path/project/app2) ...

// then run specific
// $ cargo run -p app1

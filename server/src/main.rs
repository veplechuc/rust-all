use libs::print_fn;
use rand::Rng;

fn main() {
    println!("Hello, FROM SERVER CRATE!!!! 😄");
    // Generate a random i32 integer between -100 and 100
    let random_integer = rand::thread_rng().gen_range(-100..=100);
    println!("Random Integer: {}", random_integer);
    print_fn();
}

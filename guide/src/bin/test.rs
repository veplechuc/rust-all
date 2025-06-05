const Y: i32 = X + X;
const X: i32 = 5;

fn main() {
    let maybe_string: Option<String> = Some("data".to_string());

    let Some(value) = maybe_string else {
        panic!("die horribly")
    };
    println!("{X}");
    println!("{}", oh_boy());
}

fn oh_boy() -> i32 {
    return X;
    const X: i32 = 5;
    // ^ this compiles and works. No warning!
}

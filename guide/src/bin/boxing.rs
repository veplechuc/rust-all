// Box obj is a pointer to the heap

use guide::structs_def;

fn main() {

    let bx = Box::new(99);
    println!("value of box -> {}", *bx);
    // on println we can use just bx
    
    // if we want to use the value we need to dereference
    let v = *bx;
    println!("value of v from box {}", v);

    //box does not copy.. move the ownership
    let bx1 = bx;
    //println!("{}", bx);
    // implements the drop to deallocate memory
    drop(bx1);

    // Box ussually used to represent a chain, but next needs to be a box
    // instead of referencing a Node itself.. that will be a infinte loop
    struct Node {
        data : i32,
        next: Option<Box<Node>> // Option could be Some or None
    }

    // reference counter - cant be used on threading.. Arc yes

    // unsafe
    run();

}

extern "C" {
    fn abs(n: i32)-> i32;
    fn fabs(n: f64)-> f64;
}

#[no_mangle]
pub extern "C"
fn call_ext(){
    println!("inside rust func...")
}

fn run(){
    println!("Some integration");

    unsafe {
        let r1 = abs(-42);
        println!("calling abs C on -42 ->{}", r1);
        let r2 = fabs(-42.5);
        println!("calling fabs C on -42.5 ->{}", r2);
        
        let r3 = unsafe_func();
        println!("calling unsafe func ->{}", r3);

    }
    call_ext();
}

unsafe fn unsafe_func() -> i32 {
    99
}
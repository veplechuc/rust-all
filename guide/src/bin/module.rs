// access modules through the library name (lib.rs) and then the path to the submodule.
// Assuming that in your Cargo.toml you have name = "myproject", then

use guide::file_mod1 as pprt;
use guide::maths::basic_m::multi;

use guide::my_mod::my_mod1::function;

use guide::math2::ptr;

fn main() {
    // Easier access to `deeply::nested::function`
    // ext_mod::function();
    pprt::printin();
    let n1 = 10;
    let n2 = 58;
    println!("{}", multi(&n1, &n2));
    println!("Entering block");
}

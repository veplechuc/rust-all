// submodules must be declared within the parent module -> lib.rs

pub mod basic_m {
    pub fn multi(n1: &i32, n2: &i32) -> i32 {
        n1 * n2
    }
    fn ptr(num: i32) {
        println!("pPrint frm mod -> {}", num);
    }
}

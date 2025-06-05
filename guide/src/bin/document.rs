/// comment on the function
fn some_func() {
    //comment inside func but not for doc
}
fn main() {
    /// some docs
    /// we can add markdown
    /// # Title of doc
    /// - clickeable link: ['Use the tick for links']
    /// 'this should appears as a code'  

    pub const ALGO: i32 = 45;
}

/*
CREATE DOCUMENTATION

documentation uses 3 /

//! inner documentation comments  !//

cargo doc --no-deps --open
--no-deps not include the dependecies libraries
--open: opoens index on the default browser

 */

// RUST CORE DESIGN PHILOSOPHIES
// Safety --var used in does not change so no bugs
// concurrency --  shared on different threads  -
// speed -- the compiler know the value have optimized the machine code produced

// -compiled
// - strongly typed
// - secure
// - no garbage collector
// - no implicit casting
// - variables are inmutable by default
// - no null values

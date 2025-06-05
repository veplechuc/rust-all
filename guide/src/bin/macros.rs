macro_rules! hello {
    () => {
        // matches no argument
        // all call to "hello!()" will be expanded to this
        println!("hello from the macro!!");
    };
}

//declaratives
// func like vec! or println

// procedural
// using derive implement trait automatically

macro_rules! create_fn {
    ($fn_name: ident, $input: ident, $type_input: ty, $type_output: ty) => {
        fn $fn_name($input: $type_input) -> $type_output {
            println!("the double of {} is {}", $input, $input * 2);
            $input * 2
        }
    };
}

/// repeated values
/// to define a repeated values we use ($($some_str:expr), + --> means one or more separated by a comma
///                                                         * --> means 0 or more
///                                                         ? --> means 1 or 0 )
macro_rules! concatenar {
    ($($some_str:expr), *) => {{ // need to use double { to be able to assign the value
        let mut tmp_str = String::new();
        $(
            tmp_str.push_str($some_str);
        )* //repeats the func push
        tmp_str

    }};
}

create_fn!(double, x, i32, i32);
fn main() {
    hello!();
    let phrase = concatenar!("this", " is ", " the phrase");
    println!("{}", phrase);
}

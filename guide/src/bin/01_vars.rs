fn main() {
    let var1 = 10.2;
    let var2 = 5;

    let s = 22;

    let result = var1 / var2 as f32;

    println!("{}", result);

    let var_name = true;
    let x = var_name;
    let mut str1 = String::from("some string");
    priint(&mut str1);
    println!("{}", x);

    println!("desde main {}", str1);
}

fn priint(st: &mut String) -> &mut String {
    st.push_str("gkkll"); // we can add a string between double cuotes
    st.push('h'); // or we can add a char ibetween simple coutes
    println!("from inside the func {}", st);
    st
}

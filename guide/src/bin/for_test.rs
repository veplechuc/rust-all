#![allow(dead_code)]
#![allow(unused_mut)]
fn main() {
    println!("Hello, world!");
    let x = 5; // variables ARE inmutable!!!
    println!("{}", x);
    // x= 10; error because it is inmutabñe

    let _variable: i128; // '_' previene que el compilador diga que no esta usada

    const DATA: i32 = 600_000; // const MUST be annotated can not be mutable
                               //const DATA = 600_000; Syntax Error: missing type for `const` or `static`

    let dat: u8 = 255; //when it reaches the maximun values then 256 -> 0, 257 -> 1 .....
    println!("{}", dat);

    let str = String::from("string");
    let tr = str.clone(); // need to clone it in order to have a copy

    // String is mutable most types does not have Copy trait dont copy the value it moves
    let mut str = String::from("string");
    str.push_str("...another"); // str = "string...another"

    // ------------------- BORRROWING
    //implicit
    let s = String::from("hola");
    let st = &s; // borrows but does not owns s

    //-------- explicit
    let s = String::from("hola");
    let _st: &String = &s; // borrows but does not owns s-- I wrote &String

    //borrow it is like  when passed an objec to a function as reference

    let mut s = String::from("hola");
    let st = &mut s; // borrows but does not owns s
                     // st is mutable
    st.push('g');

    // compound types
    let tup = ("some tuple", 120);
    println!("{}", tup.0);

    let (tup1, tup11) = tup;
    println!("tup.0 -> {}--- tup1->{} tup11->{}", tup.0, tup1, tup11);

    // ---------- ARRAYS --------------

    let arr = [1, 2, 3];
    let arr1 = [1; 5]; // [1,1,1,1,1,]
    for x in arr1 {
        print!("{}", x);
    }

    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
    }
    let x2 = x1;
    // let y2 = y1; value used here after move

    println!(" value of x2 ->{}", x2);
    // println!(" value of y2 -> {}", y2);

    // ARRAYs TUPLES
    let lista = [1, 2, 3]; // use mut to change the values of the array

    let mut vec = Vec::<i32>::new();
    let vec2 = vec![lista];

    println!("vec2 value -> {:?}", vec2);
    vec.push(100);

    //vec.pop... or vec[0] .. vec.len()
    let index = 3;
    let op = vec.get(index);
    match op {
        Some(_value) => println!("something"),
        None => {}
    }

    // ternary
    let condition = true;
    let _numb = if condition { 7 } else { 10 };

    /////////loop

    // loop {
    //     println!("inside loop");
    //     break;
    // }

    // break uses conditional names to breaks
    // same for continue

    // 'bob: loop {
    //     loop {
    //         loop {
    //             break 'bob;
    //         }
    //     }
    // }

    let mut coun = 0;

    let result = loop {
        coun += 1;
        if coun == 10 {
            break coun;
        }
    };
    println!("valor de coun {}", coun);
    println!("valor de coun {}", result);

    let ele = [1, 2, 3, 4, 5];

    println!("using loop with iter ");
    for el in ele.iter() {
        println!("{}", el);
    }

    println!("using loop with for.. in .. ");
    for n in 1..10 {
        // last number is exclusive
        println!("{}", n);
    }

    // let mut stri = "Hola como va";
    // // let str2: String = stri + " hola";
    // let st = stri.to_string();
    // st.push_str("otra cosa");
    // println!("{}", st);

    let mut num = 1;
    println!("using loop while..");
    while num != 0 {
        num -= 1;
        println!("{}", num);
    }

    /* **********Lifetime **************/
    let mut x = Box::new(42);
    let mut z = &x; // 'a
                    //
    for i in 0..100 {
        println!("{}", z); //'a

        x = Box::new(i);
        z = &x; //'a
    }
    // println!("variable z ->{}", z); //'a
}

fn main() {
    println!("{}", suma(15, 5));
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    println!("{}", sum);
    assert!(sum == -5);

    let y = 56;
    let call_suma = suma(sum, y);
    println!("despues de llamar a suma ->{}", call_suma);
}

fn suma(x: i32, y: i32) -> i32 {
    println!("algo {} ", x);
    println!("{}", x + y);
    // o sin return la ultima sentencia
    x + y
}

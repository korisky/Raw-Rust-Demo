const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The const: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = 2.0 * 2.5;
        println!("The value of y in the inner scope is: {y}");
    }
    // by using let -> after the inner scope, the value would result back
    println!("The value of y in the outer scope is: {y}");

    // also, by using let -> we can even change the original type of a value
    let y = 4.5;
    println!("The value of y now become: {y}");

    // different declare
    let _z = 100; // auto select data type
    let _z: u32 = 100; // declare data type explicitly

    // tuple
    let tup: (i32, f64, u8) = (500, 5.3, 0);
    let (a, b, c) = tup;
    println!("Tuple inner value a: {a}, b:{b}, c:{c}");
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("Tuple value explicitly first: {first}, second:{second}, third:{third}");

    // array
    let months = ["January", "February", "March", "April"];
    let arr: [u8; 5]; // [type;size] could also be used to declare an array
    arr = [1, 2, 3, 4, 5];
    let single_month = months[0];
    println!("Months first:{single_month}");
}

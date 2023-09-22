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
}

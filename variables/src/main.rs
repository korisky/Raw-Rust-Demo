const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The const: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = 100;
    println!("The value of y now is: {y}");
}

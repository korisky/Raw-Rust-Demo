fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("function result: {x}");

    // in Rust -> Calling a function is an expression
    // the {} also means an expression, thus, y could get value
    let y = {
        let x_1 = 3;
        x_1 + 1
    };
    println!("The value of y is {y}")
}

// five() function with return value -> i32
fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

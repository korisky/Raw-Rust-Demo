fn main() {
    let num = 3;
    if num != 0 {
        println!("Num is not 0");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 }; // single line statement in rust
    println!("number value is:{number}")
}

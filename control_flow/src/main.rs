fn main() {
    let num = 3;
    if num != 0 {
        println!("Num is not 0");
    }

    // single line statement in rust
    // but we cannot set different data type, like Java, must know the type of a variable
    let number = if 3 != 4 { 5 } else { 6 };
    println!("number value is:{number}")
}

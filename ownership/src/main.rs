// ownership principle:
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
//
// for gc, rust does not have, but
// Rust takes a different path: the memory is automatically returned once the
// variable that owns it goes out of scope.
fn main() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into function, is no longer valid here
    // println!("{}", s); // compiler would throw exception, let pass s.clone() instead

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, x is still valid
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn simple_collection() {
    {
        // mutable & dynamic size param
        let mut s = String::from("Hello");
        // append into String s
        s.push_str(", Rust");
        // way to print string
        println!("{}", s)
    }
    // out of the scope, Rust would call drop() automatically at the closing bracket

    // for String, does not work
    let s1 = String::from("S1");
    // let s2 = s1; // not working, Rust compiler prevent on compiling
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // but for integers, they works, because
    // If a type implements the Copy trait, variables that use it do not 'move' (point to same slot),
    // but rather are trivially copied, making them still valid after assignment to another variable
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

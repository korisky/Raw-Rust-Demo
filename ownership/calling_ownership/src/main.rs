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

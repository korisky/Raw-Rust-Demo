// ownership principle:
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
//
// for gc, rust does not have, but
// Rust takes a different path: the memory is automatically returned once the
// variable that owns it goes out of scope.
fn main() {
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

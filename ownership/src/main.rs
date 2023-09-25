// ownership principle:
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
fn main() {
    // mutable & dynamic size param
    let mut s = String::from("Hello");
    // append into String s
    s.push_str(", Rust");
    // way to print string
    println!("{}", s)
}

// If we want to edit the value without passing ownership, mutable reference could be used
// But mutable reference has one restriction:
// - if you have a mutable reference to a value, you can have no other references to that value
// - which means: you cannot let r1 = &mut s; let r2 = &mut s; at the same scope
fn main() {
    let mut s = String::from("Hello");
    let r1 = &mut s;
    // let r2 = &mut s; // this line would cause compiler exception -> could not has more than one
    // mutable reference for the same value at certain scope

    // pass mutable reference
    change(r1);

    println!("{}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", mutable reference");
}

// here is a dangle coding -> without consider the lifetime of function & value
// when function returns, the value would be deallocated
// fn dangle() -> &String {
//     let s = String::from("dangle");
//     &s
// }

// here is a no-dangled function, return the val mean pass the ownership
fn no_dangle() -> String {
    let s = String::from("dangle");
    s
}

// instead of passing all ownership of a value
// we can pass the im_reference, which mean we can still hold it
// the action called: BORROWING
fn main() {
    let s1 = String::from("hello");

    // &s1 can be seem as 'create a im_reference' that refers to the value of s1, but does not own it,
    // because does not own it, the value it points to will not be dropped when the im_reference stops being used
    let len = calculate_length(&s1);

    // after we called the calculate_length -> we now still have the ownership of s1,
    // s1 value is not being dropped
    println!("The length of '{}' is {}.", s1, len)
}

// here &String means only get it's pointer
fn calculate_length(s: &String) -> usize {
    // here we can not changed the im_reference value -> it's not mutable
    s.len()
} // here, s got out of scope. But because it does not have ownership
// of what it refers to, it is not dropped

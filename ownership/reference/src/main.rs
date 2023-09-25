// instead of passing all ownership of a value
// we can pass the reference, which mean we can still hold it
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // after we called the calculate_length -> we now still have the ownership of s1
    println!("The length of '{}' is {}.", s1, len)
}

// here &String means only get it's pointer
fn calculate_length(s: &String) -> usize {
    s.len()
}

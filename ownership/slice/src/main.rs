fn main() {
    let s = String::from("enumerate stuff");
    let word = first_word(&s[..]);
    println!("{}", word); // could still use the length

    let s2 = "hello world";
    let word2 = first_word(&s2);
    println!("{}", word2);

    // other slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// update to return a slice (reference)
// as we change the input from &String to &str, it allows use pass both String or simple immutable String
fn first_word(s: &str) -> &str {

    // for traversing through the String, we convert String into a bytes array
    let bytes = s.as_bytes();  // if s is not mutable, we could split it as bytes

    // // by using {:?}, we can print it out
    println!("{:?}", bytes.iter()); // Iter([72, 101, 108, 108, 111])
    println!("{:?}", bytes.iter().enumerate()); // Enumerate { iter: Iter([72, 101, 108, 108, 111]), count: 0 }

    // iter() is a method returns 'each element' in a COLLECTION
    // enumerate() wraps the result of iter() and returns each element as part of a tuple instead
    for (i, &item) in bytes.iter().enumerate() {
        // first element i, is the index
        // second element is the reference of the element
        if item == b' ' { // here b' ', means byte of ' ', would get the value 0
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let mut s = String::from("enumerate stuff");

    let word = first_word(&s);

    s.clear(); // clear the String, free the space
    println!("{}", word); // could still use the length
}

fn first_word(s: &String) -> usize {

    // for traversing through the String, we convert String into a bytes array
    let bytes = s.as_bytes();

    // by using {:?}, we can print it out
    println!("{:?}", bytes.iter()); // Iter([72, 101, 108, 108, 111])
    println!("{:?}", bytes.iter().enumerate()); // Enumerate { iter: Iter([72, 101, 108, 108, 111]), count: 0 }

    // iter() is a method returns 'each element' in a COLLECTION
    // enumerate() wraps the result of iter() and returns each element as part of a tuple instead
    for (i, &item) in bytes.iter().enumerate() {
        // first element i, is the index
        // second element is the reference of the element
        if item == b' ' { // here b' ', means byte of ' ', would get the value 0
            return i;
        }
    }

    s.len()
}
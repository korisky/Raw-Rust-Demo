fn main() {
    let s1 = gives_ownership(); // gives_ownership moves it's return value into s1
    println!("{}", s1); // which means, now main() fn can use s1

    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2); // cause we pass s2 to takes_and_gives_back fn, main() could no longer use it
    println!("{}", s3); // but as receiving the return val, ownership turn to main() again
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

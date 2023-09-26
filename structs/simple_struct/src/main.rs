#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut usr = build_user(String::from("first@open.mail"), String::from("Tom"));
    usr.email = String::from("second@open.mail");
    println!("{:?}", usr); // by setting structs entity mutable, the change take effect

    // build another entity with most param in existed one
    let user2 = User {
        username: String::from("Jerry"),
        ..usr
    };
    println!("{:?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // we can use shorthand, change username:username to simply username
        email,
        sign_in_count: 1,
    }
}

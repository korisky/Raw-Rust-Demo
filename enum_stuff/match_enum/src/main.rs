#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // as mentioned before, enum in Rust can be combined
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { 1 }
        Coin::Nickel => { 5 }
        Coin::Dime => { 10 }
        Coin::Quarter(state) => {
            println!("State from {:?}", state);
            25
        }
    }
}

// plus_one is for handling the Option enum + logic
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // handle the None situation
        Some(i) => Some(i + 1), // handle with normal logic
    }
}


fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    println!("{}", plus_one(five).unwrap());
}

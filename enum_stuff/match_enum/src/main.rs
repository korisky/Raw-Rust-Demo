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
    // Rust compiler would force us handle ALL CASES
    match x {
        None => None, // handle the None situation
        Some(i) => Some(i + 1), // handle with normal logic
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    println!("{}", plus_one(five).unwrap());

    // even integer stuff can be used with match
    let dice_roll = 9;
    match dice_roll {
        3 => {}
        7 => {}
        _ => () // _ means others, the number we not care
    }

    // if we only care one matched situation, we can use if let
    let mut count = 0;
    if let Coin::Quarter(state) = Coin::Dime {
        println!("State from {:?}", state);
    } else {
        // all other situations would go into here
        count += 1;
    }
    println!("{}", count);
}

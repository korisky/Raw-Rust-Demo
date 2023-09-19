// standard Rust library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn declare a new function
fn main() {
    println!("Guess the number!");

    // start..=end is the pattern with lower & upper bound
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    // in Rust, variables are immutable by default -> let abc = 10, abc could not be changed
    // if we want it to be mutable, need to use 'mut' before variable's name
    // :: means that new() is an associative function of String type
    let mut guess = String::new();

    // we could use std::io::stdin if we not import std::io
    io::stdin()
        // & indicate it's a reference
        // also, need to use '&mut guess' rather than '&guess' to make it mutable
        .read_line(&mut guess)
        // expect if for handling the error, would get a warning if missing
        .expect("Failed to read line");

    // convert String type to unsigned-32 bytes integer, the parse() func -> convert String to another type
    // the : after guess, let Rust know we need to annotate
    // Rust can allow use to 'Shadowing' the previous value with a new one, rather than create two unique variables
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    // in Rust, {} is the placeholder
    println!("You guessed: {guess}");

    // Ordering is an Enum with 3 values -> match is little bit similar to case in switch
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Correct"),
    }
}

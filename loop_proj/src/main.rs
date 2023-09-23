fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2 // multiply 2 then pass the return val
        }
    };

    println!("The counter result is {result}");
}

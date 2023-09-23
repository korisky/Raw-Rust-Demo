fn main() {
    multi_loop_fn();
}

// fn simple_loop_fn() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2; // multiply 2 then pass the return val
//         }
//     }; // assign the result of the loop to variable result (immutable)
//     println!("The counter result is {result}");
// }

fn multi_loop_fn() {
    let mut count = 0;

    // here using '_name_: loop{...}, we set a name for this loop
    // then inside of the loop, even we have embedded loops,
    // we could determine which one we a breaking (i.e. jump out to the loop we want)
    'counting_up: loop {
        println!("\ncount = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up; // calling it's name, we could jump out of it
            }
            remaining -= 1;
        }
        count += 1;
    }

    // Rust also support While for looping
    println!("\n count now = {count}");
    while count < 3 {
        count += 1;
    }
    println!("\nEnd count = {count}\n");

    // for the best use on traversing on an array, Rust provide in similar to range in Golang
    let arr = [10, 20, 30, 40, 50];
    for ele in arr {
        print!("{ele} ");
    }
    println!();

    // below range (0..8), would running 0 - 7, not 8
    for number in (0..8).rev() {
        print!("{number} ");
    }
    println!();
    // 1 - 4
    for number in 1..5 {
        print!("{number} ");
    }
}

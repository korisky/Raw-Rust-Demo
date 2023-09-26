#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    println!("{:?}", black);
    println!("{}", black.0);

    // using the calling with tuple
    println!(
        "The area of the rectangle is {} square pixels.",
        area((30, 50))
    );
}

// by
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

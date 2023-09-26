#[derive(Debug)]
struct Color(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let black = Color(0, 0, 0);
    println!("{:?}", black);
    println!("{}", black.0);

    // calling with tuple
    println!(
        "The area of the rectangle is {} square pixels.",
        area((30, 50))
    );

    // calling with struct
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
}

// using tuple as input
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// using structs' reference as input
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

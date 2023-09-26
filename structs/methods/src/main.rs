#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// every thing within the impl block, will be associated with Rectangle type
impl Rectangle {
    // method must have &self as the first param of a method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn pass_ownership(self) -> u32 {
        self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 1000,
    };

    println!("Square pixels: {}", rect1.area()); // here we can use method syntax to call the method
    println!("The width: {}", rect1.pass_ownership());
    // println!("Call again: {}", rect1.width); // after passing the ownership, again, main() could not access rect1 again
}

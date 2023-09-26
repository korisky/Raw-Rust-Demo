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

    // method compare with another entity
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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
    println!("Square pixels: {}", &rect1.area()); // actually when you call rect1.area(), Rust compile it as &rect1.area()
    // println!("The width: {}", rect1.pass_ownership());
    // println!("Call again: {}", rect1.width); // after passing the ownership, again, main() could not access rect1 again

    println!();
    let rect2 = Rectangle { width: 11, height: 1001 };
    if rect2.can_hold(&rect1) {
        println!("rect2 can hold rect1");
    }
}

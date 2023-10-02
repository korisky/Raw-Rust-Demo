// #[derive(Debug)]
// enum IpAddKind {
//     V4,
//     V6,
// }

// rather than define pure enum without any val,
// the kind of combined enum below is much more useful
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}


fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("{:?}", home);
}


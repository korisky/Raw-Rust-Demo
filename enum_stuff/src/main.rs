// #[derive(Debug)]
// enum IpAddKind {
//     V4,
//     V6,
// }

// rather than define pure enum without any val,
// the kind of combined enum below is much more useful,
// but it's still different with a 'struct'
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let home_v4 = IpAddr::V4(127, 0, 0, 1);
    let home_v6 = IpAddr::V6(String::from("127.0.0.1"));
    println!("{:?}", home_v4);
    println!("{:?}", home_v6);
}


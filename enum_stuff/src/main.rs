// #[derive(Debug)]
// enum IpAddKind {
//     V4,
//     V6,
// }

// rather than define pure enum without any val,
// the kind of combined enum below is much more useful,
// but it's still different with a 'struct'
// can be seem as:
// struct Ipv4Addr {
//     // --snip--
// }
//
// struct Ipv6Addr {
//     // --snip--
// }
//
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
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


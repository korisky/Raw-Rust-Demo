#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

fn main() {
    let loopback = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("::1"),
    };

    route(loopback.kind);
}

fn route(ip_kind: IpAddKind) {
    println!("{:?}", ip_kind);
}

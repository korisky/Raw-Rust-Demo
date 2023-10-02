#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    route(four);
    route(six)
}

fn route(ip_kind: IpAddKind) {
    println!("{:?}", ip_kind);
}


enum IpAddressKind {
    V4(String),
    V6(String),
}

fn main() {
    route(IpAddressKind::V4(String::from("127.0.0.1")));
    route(IpAddressKind::V6(String::from("::1")));
}

fn route(_ip: IpAddressKind) { }

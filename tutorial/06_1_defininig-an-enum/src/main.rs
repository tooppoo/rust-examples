
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    route(IpAddressKind::V4(127,0,0,0));
    route(IpAddressKind::V6(String::from("::1")));
}

fn route(_ip: IpAddressKind) { }


enum IpAddressKind {
    V4,
    V6,
}

fn main() {
    route(IpAddressKind::V4);
    route(IpAddressKind::V6);
}

fn route(_ip: IpAddressKind) { }

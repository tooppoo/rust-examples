
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) -> &str {
        &"some message"
    }
}

fn main() {
    route(IpAddressKind::V4(127,0,0,0));
    route(IpAddressKind::V6(String::from("::1")));

    let m: Message = Message::Move { x: 10, y: 20 };
    println!("{}", m.call());
}

fn route(_ip: IpAddressKind) { }

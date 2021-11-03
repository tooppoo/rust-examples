
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

    let some_n = Some(5);
    let some_s = Some("a string");
    let absent_n: Option<i32> = None;

    let sum = some_n.unwrap_or(0) + 10;
    println!("sum = {}", sum);

    let mes = some_s.unwrap_or("none");
    println!("mes = {}", mes);

    let sum = absent_n.unwrap_or(0) + 10;
    println!("sum = {}", sum);
}

fn route(_ip: IpAddressKind) { }

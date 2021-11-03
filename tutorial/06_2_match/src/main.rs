
#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl Coin {
    fn to_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state quarter from {:?}", state);

                25
            },
        }  
    }
}

fn main() {
    println!("penny = {}c", Coin::Penny.to_cents());
    println!("nickel = {}c", Coin::Nickel.to_cents());
    println!("dime = {}c", Coin::Dime.to_cents());
    println!("quarter = {}c", Coin::Quarter(UsState::Alabama).to_cents());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}", six);
    println!("none = {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1)
    }
}

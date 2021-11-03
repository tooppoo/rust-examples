
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

fn main() {
    println!("penny = {}c", value_in_cents(Coin::Penny));
    println!("nickel = {}c", value_in_cents(Coin::Nickel));
    println!("dime = {}c", value_in_cents(Coin::Dime));
    println!("quarter = {}c", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);

            25
        },
    }
}

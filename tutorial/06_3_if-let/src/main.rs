
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
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let count = count_up_if_quarter_via_match(coin, 0);
    println!("count is {}", count);

    let coin = Coin::Dime;
    let count = count_up_if_quarter_via_match(coin, 0);
    println!("count is {}", count);

    let coin = Coin::Quarter(UsState::Alabama);
    let count = count_up_if_quarter_via_if_let(coin, 0);
    println!("count is {}", count);

    let coin = Coin::Dime;
    let count = count_up_if_quarter_via_if_let(coin, 0);
    println!("count is {}", count);
}

fn count_up_if_quarter_via_match(coin: Coin, count: u32) -> u32 {
    match coin {
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);

            count
        },
        _ => count + 1,
    }
}

fn count_up_if_quarter_via_if_let(coin: Coin, count: u32) -> u32 {
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {:?}", state);

        count
    } else {
        count + 1
    }
}

fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 4, 3 or 2");
    }

    let condition: bool = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("the value of number is {}", number);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFT OFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("a[{}] value is {}", index, a[index]);

        index = index + 1;
    }
    for elem in a.iter() {
        println!("elem value is {}", elem);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFT OFF!");

    let f = 104.0;
    let c = 40.0;
    println!(
        "{}°F should be {}°C, actual is {}°C", f, fahrenheit_to_celsius(f), c
    );

    println!("fibo(3) should be 2, actutal is {}", fibo(3));
    println!("fibo(5) should be 5, actutal is {}", fibo(5));
    println!("fibo(10) should be 55, actutal is {}", fibo(10));

    println!("{}", twelve_days_of_christmas());
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn fibo(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibo(n - 1) + fibo(n - 2)
    }
}

fn twelve_days_of_christmas() -> String {
    fn cardinal_to_ordinal(car: u32) -> String {
        match car {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => panic!(car.to_string() + " is not supported")
        }.to_string()
    }
    fn present_of(month: u32) -> String {
        match month {
            1 => "a partridge",
            2 => "two turtle doves",
            3 => "three French hens",
            4 => "four calling birds",
            5 => "five golden rings",
            6 => "six geese a-laying",
            7 => "seven swans a-swimming",
            8 => "eight maids a-milking",
            9 => "nine ladies dancing",
            10 => "ten lords a-leaping",
            11 => "eleven pipers piping",
            12 => "twelve drummers drumming",
            _ => panic!(month.to_string() + " is not supported")
        }.to_string()
    }
    fn lyric_of(month: u32) -> String {
        let ordinal = cardinal_to_ordinal(month);
        let mut head = String::new();
        head.push_str("On the ");
        head.push_str(&ordinal);
        head.push_str(" day of Christmas my true love sent to me\n");

        let mut presents = String::new();

        match month {
            1 => {
                presents.push_str(&present_of(month));
                presents.push_str(".");
            },
            _ => {
                for m in (1..month + 1).rev() {
                    let present = if m == 1 {
                        String::from("and ") + &present_of(m) + "."
                    } else {
                        present_of(m) + ","
                    };

                    presents.push_str(&present);
                    presents.push_str("\n");
                };
            }
        }

        let mut body = String::new();
        body.push_str(&head);
        body.push_str(&presents);

        body
    }

    let mut lyric = String::new();
    for m in 1..13 {
        lyric.push_str(&lyric_of(m));
        lyric.push_str("\n");
    };

    lyric
}

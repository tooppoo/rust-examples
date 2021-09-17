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

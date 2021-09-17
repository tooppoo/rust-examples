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
}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is {}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    let z: f64 = x;
    println!("x is {}, y is {}, z is {}", x, y, z);

    let sum: i32 = 5 + 10;
    let difference: f64 = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient: f32 = 56.7 / 32.2;
    let remainder: usize = 43 % 5;
    println!(
        "sum is {}, difference is {}, product is {}, quotient is {}, remainde is {}",
        sum, difference, product, quotient, remainder
    );

    let t = true;
    let f: bool = false;
    println!("t is {}, f is {}", t, f);

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat: char = '😻';
    println!("c is {}, z is {}, cat is {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 255);
    let (x, y, z) = tup;
    println!("tup is ({}, {}, {})", x, y, z);
    println!("tup is ({}, {}, {})", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first is {}, second is {}", first, second);
}

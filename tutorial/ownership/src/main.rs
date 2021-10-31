fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = "hello";
    let s2 = s1;

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s1 = String::from(s1);
    let s2 = s1.clone();
    let s3 = s1;

    // println!("{}, world!", s1); error
    println!("{}, world!", s2);
    println!("{}, world!", s3);

    let s = String::from("hello");

    takes_ownership(s);
    // println!("{}", s); error

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
fn makes_copy(i: i32) {
    println!("{}", i);
}

fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = "hello";
    let s2 = s1;

    println!("s1 {}, world!", s1);
    println!("s2 {}, world!", s2);

    let s1 = String::from(s1);
    let s2 = s1.clone();
    let s3 = s1;

    // println!("{}, world!", s1); error
    println!("s2 {}, world!", s2);
    println!("s3 {}, world!", s3);

    let s = String::from("hello");

    takes_ownership(s);
    // println!("{}", s); error

    let x = 5;
    makes_copy(x);
    println!("x = {}", x);

    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    let s2 = String::from("takes and gives back");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
fn makes_copy(i: i32) {
    println!("{}", i);
}
fn gives_ownership() -> String {
    let s = String::from("give ownership");

    s
}
fn takes_and_gives_back(s: String) -> String {
    s
}

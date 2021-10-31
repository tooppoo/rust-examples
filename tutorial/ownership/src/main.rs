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
}

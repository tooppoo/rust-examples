fn main() {
    let s1 = String::from("calculate length");
    let len = calculate_length(&s1);

    println!("the length of '{}' is  {}.", s1, len);

    let mut s = String::from("hello");
    println!("s before change is '{}'", s);
    change_borrowing(&mut s);
    println!("s after change is '{}'", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change_borrowing(s: &mut String) {
    s.push_str(", world!");
}

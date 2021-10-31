fn main() {
    let s1 = String::from("calculate length");
    let len = calculate_length(&s1);

    println!("the length of '{}' is  {}.", s1, len);

    let mut s = String::from("hello");
    println!("s before change is '{}'", s);
    change_borrowing(&mut s);
    println!("s after change is '{}'", s);

    let mut s = String::from("hello");
    {
        let mut_s1 = &mut s;
        mut_s1.push_str(", in scope");
        println!("mut_s1 = '{}'", mut_s1);
    }
    // println!("s1 = '{}'", mut_s_1); // error
    let mut_s2 = &mut s;
    mut_s2.push_str(", out of scope");
    println!("mut_s2 = '{}'", mut_s2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // error
    println!("r1 = '{}', r2 = '{}'", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change_borrowing(s: &mut String) {
    s.push_str(", world!");
}

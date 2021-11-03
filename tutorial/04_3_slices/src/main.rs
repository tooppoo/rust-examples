fn main() {
    let mut s = String::from("Hello, world!");
    let first_at = first_word_at(&s);

    s.clear();

    println!("first position of '{}' is {}", s, first_at);

    let s = String::from("cogito, ergo sum");
    let first = first_word(&s);
    println!("first of '{}' is '{}'", s, first);
}

fn first_word_at(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

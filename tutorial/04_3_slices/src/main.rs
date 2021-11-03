fn main() {
    let mut s = String::from("Hello, world!");
    let first_at = first_word_at(&s);

    s.clear();

    println!("first position of '{}' is {}", s, first_at);

    let s = String::from("cogito, ergo sum");
    let first = first_word(&s);
    println!("first of '{}' is '{}'", s, first);

    let string = String::from("hello world");
    let word = first_word(&string[..]);
    println!("word is '{}'", word);

    let literal = "hello world";

    let word = first_word(&literal[..]);
    println!("word is '{}'", word);

    let word = first_word(literal);
    println!("word is '{}'", word);
}

fn first_word_at(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

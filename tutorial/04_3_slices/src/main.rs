fn main() {
    let mut s = String::from("Hello, world!");
    let first_at = first_word_at(&s);

    s.clear();

    println!("first of '{}' is {}", s, first_at);
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

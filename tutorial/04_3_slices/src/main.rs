fn main() {
    let s = String::from("Hello, world!");
    let first_at = first_word(&s);

    println!("first of '{}' is {}", s, first_at);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

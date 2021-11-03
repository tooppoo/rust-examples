
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    println!("user name is '{}', email is '{}'", user.username, user.email);

    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    user.email = String::from("anotermail@example.com");
    println!("user name is '{}', email is '{}'", user.username, user.email);
}

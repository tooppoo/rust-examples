
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        name: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    println!("user name is '{}', email is '{}'", user.name, user.email);

    let mut user = User {
        email: String::from("someone@example.com"),
        name: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    user.email = String::from("anotermail@example.com");
    println!("user name is '{}', email is '{}'", user.name, user.email);

    let user = new_user(
        String::from("someone@example.com"),
        String::from("someone")
    );
    println!("user name is '{}', email is '{}'", user.name, user.email);
}

fn new_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

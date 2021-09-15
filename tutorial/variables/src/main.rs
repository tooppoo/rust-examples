fn main() {
    mutate();
    shadowing();
}

fn mutate() {
    let mut x = 5;
    println!("mutate:value of x is {}", x);
    x = 6;
    println!("mutate:value of x is {}", x);
}

fn shadowing() {
    let x = 5;
    println!("shadowing:value of x is {}", x);
    let x = x + 3;
    println!("shadowing:value of x is {}", x);
    let x = x * 2;
    println!("shadowing:value of x is {}", x);
}

fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    // let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("value of y is {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("value of x is {}, y is {}", x, y);
}
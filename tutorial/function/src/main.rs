fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    // let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("value of y is {}", y);

    let x = five();
    println!("value of x is {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("value of x is {}, y is {}", x, y);
}

fn five() -> i32 {
    5
}

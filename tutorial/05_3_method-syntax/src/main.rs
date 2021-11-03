#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let r1 = Rectangle { width: 30, height: 50 };
    let r2 = Rectangle { width: 10, height: 40 };
    let r3 = Rectangle { width: 60, height: 45 };

    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));
}

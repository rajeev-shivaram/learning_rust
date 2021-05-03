#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self)-> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { height: 5, width: 6 };
    println!("{:#?}", rect1);
    println!("Area of Rectangle is {}", rect1.area());
}

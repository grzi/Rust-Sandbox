/*
    This project used to use struct / tuples ...
*/

fn main() {
    let rect = Rectangle::from(10,10);
    let rect2 = Rectangle::from(4,4);
    let rect3 = Rectangle::from(10,12);
    println!("The area of the rectangle is {}", rect.area());
    println!("rect can fit in rect2 ? {}", rect.can_hold(&rect2));
    println!("rect can fit in rect3 ? {}", rect.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn from(width: u32, height: u32) -> Rectangle {
        return Rectangle { width, height };
    }
}



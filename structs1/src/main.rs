#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.length * self.width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.length > other.length && self.width > other.width;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle { length: size, width: size }
    }
}
fn main() {
    let rect1 = Rectangle {length: 50, width: 30};
    let rect2 = Rectangle {length: 20, width: 10};
    let rect3 = Rectangle {length: 100, width: 200};

    println!("The area of the rect is {} pixels", rect1.area());
    println!("rect is {:?}", rect1);

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(5);
    println!("Square is {:?}", square);

    
}


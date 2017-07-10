#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect = Rectangle {length: 50, width: 30};
    println!("The area of the rect is {} pixels", area(&rect));
    println!("rect is {:?}", rect)
}

fn area(rect: &Rectangle) -> u32 {
    return rect.length * rect.width;
}

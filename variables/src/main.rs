fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let a = 2.0; //f64
    let b: f32 = 3.0; //f32

    let tup: (i32, f64, u8) = (500, 2.13, 10);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    let five_hundred = tup.0;
    println!("{}",five_hundred);
}

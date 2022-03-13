struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 30
    };

    println!("area of rectangle: {}", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}

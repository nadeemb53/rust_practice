struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30
    };

    let rect2 = Rectangle {
        width: 40,
        height: 40
    };

    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));

    println!("can rect2 hold rect1: {}", rect2.can_hold(&rect1));

}
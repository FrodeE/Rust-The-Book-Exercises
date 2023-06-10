struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width * self.height >= other.width*other.height{
            return true
        }
        false
    }
    
    //Associated Function
    fn create_rect() -> Rectangle {
        Rectangle {
            width: 10,
            height: 40,
        }
    }

    //Associated function 2
    fn create_square() -> Self {
        Rectangle {
            width: 40,
            height: 40,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //let rect2 = create_rect();

    let rect2 = Rectangle::create_rect();

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::create_square();

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square dimensions are {}x{}", rect4.width, rect4.height)
}

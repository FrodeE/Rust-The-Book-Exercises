#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area_3(&self) -> i32 {
        self.width*self.height
    }
    fn rec_diff(&self) -> i32{
        (self.width-self.height).abs()
    }
}

fn main() {
    //  WAY 1
    let rect1 = (30,50);
    println!("The area of the rectangle 1 is {} square pixels.",
    area_1(rect1));

    // WAY 2
    let rect2 = Rectangle {
        width: 10,
        height: 50,
    };

    println!("The area of the rectangle 2 is {} square pixels.",
    area_2(&rect2));
    println!("Rect2 is {:#?}", rect2);

    println!("The area of the rectangle 3 is {} square pixels.",
    rect2.area_3());

    println!("Width and height difference is {}", rect2.rec_diff());

}

fn area_1(dimensions: (u32, u32)) -> u32 {
    dimensions.0*dimensions.1
}

fn area_2(rec: &Rectangle) -> i32 {
    rec.width*rec.height
}

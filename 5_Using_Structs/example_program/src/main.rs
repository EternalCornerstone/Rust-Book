//program to calculate the area of a rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
// method syntax
impl Rectangle {
    // new functionality to instantiate type of rectangle easier with 1 parameter if it's a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect = Rectangle {
        width: width1,
        height: height1
    };
    println!("rect is {rect:#?}");
    println!("The area of the rectangle is {} square pixels.", area_from_variables(width1, height1));
    println!("The area of the rectangle is {} square pixels.", area_from_tuple((width1, height1)));
    println!("The area of the rectangle is {} square pixels.", area_from_struct(&rect));
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // using the method syntax with impl's
    let rect2 = Rectangle {
        width: 400,
        height: 200,
    };
    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }
    println!("The area of the rectangle is: {} square pixels.", rect2.area());

    println!("Can rect hold rect1? {}", rect1.can_hold(&rect));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect));
    println!("Can rect2 hold rect? {}", rect2.can_hold(&rect));

    // using square syntax
    let sq = Rectangle::square(3);

}

fn area_from_variables(width: u32, height: u32) -> u32 {
    width * height
}

fn area_from_tuple(dimentions:(u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

fn area_from_struct(dimentions: &Rectangle) -> u32 {
    dimentions.width * dimentions.height
}


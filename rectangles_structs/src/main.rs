
// able to provide proper display possibilities
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// will be added to the rectangles type.
impl Rectangle {
    /// The function takes a reference to a Rectangle and returns a u32
    /// 
    /// Returns:
    /// 
    /// A tuple containing the width and height of the rectangle.
    fn calculate_area(&self) -> u32 {
        // Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.calculate_area() > other_rect.calculate_area()
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    // calculate the area of a rectangle
    println!("The area of the rectangle: {:#?} is {} squared pixels.", rect1, rect1.calculate_area());

    // can rectangle fit in each other
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect1))
}

// fn calculate_area(dimensions: &Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }

// able to provide proper display possibilities
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // calculate the area of a rectangle
    println!("The area of the rectangle: {:#?} is {} squared pixels.", rect1, calculate_area(&rect1));
}

fn calculate_area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
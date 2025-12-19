// An Example Program Using Structs
// Calculate the area of a rectangle
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {}", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//  Refactoring with tuples
fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

fn area(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

// Refactoring with Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.", area(rect1));

    println!("rect1 is {rect1:#?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

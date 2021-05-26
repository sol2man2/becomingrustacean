fn main() {
    println!("ch5-2!");

    // An Example Program Using Structs

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactoring with Tuples

    fn area1(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    let rect1 = (30, 50);

    println!(
        "The area1 of the rectangle is {} square pixels.",
        area1(rect1)
    );

    // Refactoring with Structs: Adding More Meaning

    fn area2(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area2 of the rectangle is {} square pixels.",
        area2(&rect1)
    );

    // Adding Useful Functionality with Derived Traits
    // println!("rect1 is {}", rect1);
    // println!("rect1 is {:?}", rect1);
    // println!("rect1 is {:#?}", rect1);
}

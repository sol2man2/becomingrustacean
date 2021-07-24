fn main() {
    println!("ch3-1!");

    // Variables and Mutability
    let mut x = 5;
    println!("x, {}", x);
    x = 6;
    println!("x, {}", x);

    // Differences Between Variables and Constants
    const MAX_INT: u32 = 3;

    // Shadowing
    let y = 5;
    println!("y, {}", y);
    let y = "test";
    println!("y, {}", y);

    // let mut z = 5;
    // z = "test";
}

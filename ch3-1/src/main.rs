
fn main() {
    println!("ch3-1!");

    // Variables and Mutability
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);

    // Differences Between Variables and Constants
    const MAX_INT: u32 = 3;

    // Shadowing
    let y = 5;
    let y = "test";

    // let mut z = 5;
    // z = "test";
}

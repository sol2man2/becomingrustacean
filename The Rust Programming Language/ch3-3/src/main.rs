fn main() {
    println!("ch3-3!");

    // Functions
    fn another_function() {
        println!("another function.");
    }

    another_function();

    // Function Parameters
    fn other(x: i32, y: i32) {
        println!("other, {} {}", x, y);
    }

    other(3, 4);

    // Function Bodies Contain Statements and Expressions
    let x = 2;
    // let y = (let z = 6);

    let a = {
        let x = 3;
        x + 1
    };
    println!("a: {}", a);

    // Functions with Return Values
    fn five() -> i32 {
        5
    }
    println!("five: {}", five());

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    println!("plus: {}", plus_one(5));
}

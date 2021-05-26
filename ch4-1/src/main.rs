fn main() {
    println!("ch4-1!");

    // The Stack and the Heap

    // Ownership Rules
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // Variable Scope
    {
        let mut _s = "hello";
        //
    }

    // The String Type
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!(": {}", s);

    // Memory and Allocation
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we’re done with our String.
    {
        let s = String::from("hello");
        //
    }

    // Ways Variables and Data Interact: Move
    let x = 5;
    let y = x;
    println!("x, {}", x);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1, {}", s1);

    // Ways Variables and Data Interact: Clone
    let s3 = s2.clone();
    println!("s2, s3 = {}, {}", s2, s3);

    // Stack-Only Data: Copy
    println!("x: {}, y: {}", x, y);

    // Ownership and Functions
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s, {}", s);
    let x = 5;
    makes_copy(x);
    println!("x, {}", x);

    fn takes_ownership(some_string: String) {
        println!("some_string, {}", some_string);
    }
    fn makes_copy(some_integer: i32) {
        println!("some_integer, {}", some_integer);
    }

    // Return Values and Scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }
}

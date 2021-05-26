fn main() {
    println!("ch3-5!");

    // if expr
    let number = 7;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    if number == 0 {
        println!("if expr");
    }

    // Handling Multiple Conditions with else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let x = if number == 0 { 5 } else { 6 };
    // let y = if number == 0 { 5 } else { "6" };

    // Repeating Code with loop
    // loop {
    //     println!("again!");
    // }

    // Returning Values from Loops
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop break, {}", res);

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("while: {}", number);
        number -= 1;
    }

    println!();

    // Looping Through a Collection with for
    let arr1 = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("while: {}", arr1[index]);
        index += 1;
    }

    for elem in arr1.iter() {
        println!("for: {}", elem);
    }
}

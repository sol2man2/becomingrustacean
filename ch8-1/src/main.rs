fn main() {
    println!("ch8-1!");

    // Storing Lists of Values with Vectors
    let v: Vec<i32> = Vec::new();

    // Creating a New Vector
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("1 The third element is {}", third);

    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    let third: &String = &v[1];

    match v.get(2) {
        Some(third) => println!("2 The third element is {}", third),
        None => println!("3 There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("4 The first element is: {}", first);

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

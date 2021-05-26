fn main() {
    println!("ch4-2!");

    // References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // Mutable References
    let mut s = "hello".to_string();
    change(&mut s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    println!("return: {}", s);

    let mut s = "hello".to_string();
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // println!("s: {}", s);
    // println!("r1: {}", r1);
    // println!("s,r1: {}, {}", s, r1);

    // data race, race condition
    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.

    // {
    //     let r1 = &mut s;
    //     println!("r1 in block, {}", r1);
    // }

    // {
    //     let r3 = &mut s;
    //     println!("r3 in block, {}", r1);
    // }

    // combining mutable and immutable ref.
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // println!("r3, {}", r3);

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{}, and {}", r1, r2);
    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    // Dangling References
    // let reference_to_nothing = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }

    // The Rules of References
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
}

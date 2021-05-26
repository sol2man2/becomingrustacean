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
    let mut s = "world".to_string();
    change(&mut s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.

    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{}, and {}", r1, r2);
    // let r3 = &mut s; // BIG PROBLEM
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

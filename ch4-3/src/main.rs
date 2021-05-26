fn main() {
    println!("ch4-3!");

    // The Slice Type
    // fn first_word(s: &String) -> ?

    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //

    // fn first_word(s: &String) -> usize {
    //     let bytes = s.as_bytes();
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return i;
    //         }
    //     }
    //     s.len()
    // }
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("s: {}", s);
    // println!("the first word index is: {}", word);

    // String Slices
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    // fn first_word(s: &String) -> &str {
    //     let bytes = s.as_bytes();
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return &s[0..i];
    //         }
    //     }
    //     &s[..]
    // }
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("s: {}", s);
    // println!("the first word index is: {}", s);
    // println!("the first word index is: {}", word);

    // String Literals Are Slices
    let s = "Hello, world!";

    // String Slices as Parameters
    // fn first_word(s: &String) -> &str {
    // fn first_word(s: &str) -> &str {

    // fn first_word(s: &str) -> &str {
    //     let bytes = s.as_bytes();
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return &s[0..i];
    //         }
    //     }
    //     &s[..]
    // }
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // println!("s: {}", s);
    // println!("the first word: {}", word);

    // let s = "hello world";
    // let word = first_word(&s);
    // println!("s: {}", s);
    // println!("the first word: {}", word);

    // let mut s = String::from("hello world");
    // let word = first_word(&my_string[..]);
    // let my_string_literal = "hello world";
    // let word = first_word(&my_string_literal[..]);
    // let word = first_word(my_string_literal);

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

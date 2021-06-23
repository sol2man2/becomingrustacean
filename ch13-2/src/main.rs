fn main() {
    println!("ch13-2!");
    // Processing a Series of Items with Iterators
    //
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    // move on lib.rs

    // Methods that Produce Other Iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // move on to lib.rs
}

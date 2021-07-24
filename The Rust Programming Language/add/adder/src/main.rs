use add_one;
use rand;

fn main() {
    println!("Hello, world!");
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}

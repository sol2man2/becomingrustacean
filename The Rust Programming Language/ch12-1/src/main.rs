use std::env;

fn main() {
    println!("ch12-1!");

    // Accepting Command Line Arguments

    // Reading the Argument Values
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // The args Function and Invalid Unicode
    // std::env::args_os

    // Saving the Argument Values in Variables
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

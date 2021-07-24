use std::env;
use std::fs;

fn main() {
    println!("ch12-3!");

    // Refactoring to Improve Modularity and Error Handling
    // 1. our main function now performs two tasks
    // 2. The longer main becomes, the more variables we’ll need to bring into scope;
    // the more variables we have in scope,
    // the harder it will be to keep track of the purpose of each
    // 3. Reading a file can fail in a number of ways
    // 4. we use expect repeatedly to handle different errors

    // Separation of Concerns for Binary Projects
    // a guideline for splitting the separate concerns
    // * Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
    // * As long as your command line parsing logic is small, it can remain in main.rs.
    // * When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

    // The responsibilities of main is the following:
    // * Calling the command line parsing logic with the argument values
    // * Setting up any other configuration
    // * Calling a run function in lib.rs
    // * Handling the error if run returns an error

    // Extracting the Argument Parser
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    fn parse_config(args: &[String]) -> (&str, &str) {
        let query = &args[1];
        let filename = &args[2];
        (query, filename)
    }

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    println!("With text:\n{}", contents);

    // Grouping Configuration Values

    // The Trade-Offs of Using clone

    // Creating a Constructor for Config

    // Fixing the Error Handling

    // Improving the Error Message

    // Returning a Result from new Instead of Calling panic!

    // Calling Config::new and Handling Errors

    // Extracting Logic from main

    // Returning Errors from the run Function

    // Handling Errors Returned from run in main

    // Splitting Code into a Library Crate
}

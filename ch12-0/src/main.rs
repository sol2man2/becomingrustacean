//////////////////////////////////////////////////////////////////

use std::env;
use std::process;

mod lib;

use crate::lib::minigrep;
use crate::lib::minigrep::Config;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("problem {}", err);
    process::exit(1);
  });
  println!("searching for {}", config.q);
  println!("filename: {}", config.f);

  if let Err(e) = minigrep::run(config) {
    eprintln!("App err: {}", e);
    process::exit(1);
  }
}

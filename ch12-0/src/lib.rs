pub mod minigrep {
  use std::env;
  use std::error::Error;
  use std::fs;

  pub struct Config {
    pub q: String,
    pub f: String,
    pub c_s: bool,
  }
  impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
        return Err("not enough args");
      }

      Ok(Config {
        q: args[1].clone(),
        f: args[2].clone(),
        c_s: env::var("C_S").is_err(),
      })
    }
  }

  pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let cs = fs::read_to_string(config.f)?;

    let res = if config.c_s {
      search(&config.q, &cs)
    } else {
      search_case_insensitive(&config.q, &cs)
    };

    for line in res {
      println!("{}", line);
    }
    Ok(())
  }
  pub fn search<'a>(q: &str, cs: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in cs.lines() {
      if line.contains(q) {
        res.push(line);
      }
    }
    res
  }
  pub fn search_case_insensitive<'a>(q: &str, cs: &'a str) -> Vec<&'a str> {
    let q = q.to_lowercase();
    let mut res = Vec::new();
    for line in cs.lines() {
      if line.to_lowercase().contains(&q) {
        res.push(line);
      }
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      minigrep::search(query, contents)
    );
  }
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      minigrep::search_case_insensitive(query, contents)
    );
  }
}

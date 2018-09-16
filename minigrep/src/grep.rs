use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
    contents: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, contents: String::from("") })
    }

    fn load_contents(&mut self) -> Result<(), Box<Error>> {
        let mut f = File::open(self.filename.clone())?;
        self.contents = String::new();
        f.read_to_string(&mut self.contents)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        self.load_contents()?;

        for line in search(&self.query, &self.contents) {
            println!("{}", line);
        }

        Ok(())
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
                Rust:
                safe, fast, productive.
                Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
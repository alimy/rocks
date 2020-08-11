use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
    contents: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, contents: String::from(""), case_sensitive })
    }

    fn load_contents(&mut self) -> Result<(), Box<Error>> {
        let mut f = File::open(self.filename.clone())?;
        self.contents = String::new();
        f.read_to_string(&mut self.contents)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        self.load_contents()?;

        let results = if self.case_sensitive {
            self.search_case_sensitive()
        } else {
            self.search_case_insensitive()
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }

    fn search_case_sensitive(&self) -> Vec<&str> {
        let mut results = Vec::new();

        for line in self.contents.lines() {
            if line.contains(&self.query) {
                results.push(line.trim());
            }
        }
        results
    }

    fn search_case_insensitive(&self) -> Vec<&str> {
        let query = self.query.to_lowercase();
        let mut results = Vec::new();

        for line in self.contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line.trim());
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_work() {
        println!("it's work");
    }
}
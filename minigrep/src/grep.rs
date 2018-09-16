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
        println!("With text:\n{}", self.contents);
        Ok(())
    }
}

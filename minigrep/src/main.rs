extern crate minigrep;

use minigrep::grep;
use std::env;
use std::process;

fn run_minigrep() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run_minigrep_grep() {
    let mut config = grep::Config::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = config.run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn main() {
    // run_minigrep();
    run_minigrep_grep();
}

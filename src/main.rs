extern crate clap;
extern crate guessing_game;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("rocks")
        .author("Micheal Li")
        .about("Rust tutorial")
        .subcommand(SubCommand::with_name("hello_world")
            .about("hello world"))
        .subcommand(SubCommand::with_name("guessing_game")
            .about("a simple guessing game")
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("hello_world") {
        println!("Hello, world!");
    } else if let Some(matches) = matches.subcommand_matches("guessing_game") {
        guessing_game::start();
    }
}

extern crate clap;
extern crate guessing_game;
extern crate ownership;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("rocks")
        .author("Micheal Li")
        .about("Rust tutorial")
        .subcommand(SubCommand::with_name("hello_world")
            .about("hello world"))
        .subcommand(SubCommand::with_name("guessing_game")
            .about("a simple guessing game")
        )
        .subcommand(SubCommand::with_name("ownership")
            .about("something about ownership")
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("hello_world") {
        println!("Hello, world!");
    } else if let Some(_matches) = matches.subcommand_matches("guessing_game") {
        guessing_game::start();
    } else if let Some(_matches) = matches.subcommand_matches("ownership") {
        ownership::run();
    }
}

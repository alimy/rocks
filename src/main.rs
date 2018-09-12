extern crate clap;
extern crate communicator;
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
        .subcommand(SubCommand::with_name("communicator")
            .about("someting about module system in rust")
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("hello_world") {
        println!("Hello, world!");
    } else if let Some(_matches) = matches.subcommand_matches("guessing_game") {
        let guess = guessing_game::Guess::new(32);
        println!("guess.value(): {}", guess.value());
        guessing_game::run();
    } else if let Some(_matches) = matches.subcommand_matches("ownership") {
        ownership::run();
    } else if let Some(_matches) = matches.subcommand_matches("communicator") {
        communicator::run();
        communicator::client::connect();
        communicator::network::connect();
        communicator::network::server::connect();
        communicator::server::connect();
    }
}

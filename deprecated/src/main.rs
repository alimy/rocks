extern crate clap;
extern crate collections;
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
        .subcommand(SubCommand::with_name("collections")
            .about("someting about collections")
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("hello_world") => {
            println!("Hello, world!");
        },
        Some("guessing_game") => {
            let guess = guessing_game::Guess::new(32);
            println!("guess.value(): {}", guess.value());
            guessing_game::run();
        },
        Some("ownership") => {
            ownership::run();
        },
        Some("communicator") => {
            communicator::run();
            communicator::client::connect();
            communicator::network::connect();
            communicator::network::server::connect();
            communicator::server::connect();
        },
        Some("collections") => {
            collections::run();
        },
        _ => {},
    }
}

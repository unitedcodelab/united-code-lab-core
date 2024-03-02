pub mod sanic;
use clap::{arg, command, Command};

pub fn load() {
    let matches = command!()
        .subcommand(
            Command::new("run")
                .about("Run some project")
                .subcommand(Command::new("sanic").about("Run sanic project")),
                // ...
        )

        .subcommand(
            Command::new("build")
                .about("Build some project")
                .subcommand(Command::new("sanic").about("Build sanic project"))
                // ...
        )
        .get_matches();


    match matches.subcommand() {
        Some(("run", sub_match)) => {
            match sub_match.subcommand() {
                Some(("sanic", _)) => sanic::run(),
                // ...
                _ => fails::unvailable_technology()
            }
        }

        Some(("build", sub_match)) => {
            match sub_match.subcommand() {
                Some(("sanic", _)) => sanic::build(),
                // ...
                _ => fails::unvailable_technology()
            }
        }

        _ => println!("Run 'ucl --help' to see more"),
    }
}


mod fails {
    pub fn unvailable_technology() {
        println!("Select some available technology:");
        println!("   sanic");
    }
}
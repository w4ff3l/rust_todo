use std::{env, error::Error, process};

use config::Config;

mod action_handler;
mod parser;
mod parse_file_error;
mod action;
mod config;
mod task;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::build(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    action_handler::handle_action(config);
    Ok(())
}

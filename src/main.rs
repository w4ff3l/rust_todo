use std::{env, error::Error, process};

use config::Config;

mod action;
mod action_handler;
mod config;
mod parser;
mod task;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let file_directory_buff = env::current_dir().unwrap();

    let config = Config::build(file_directory_buff, &arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    action_handler::handle_action(config)?;
    Ok(())
}

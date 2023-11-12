use std::{env, error::Error, process};

use config::Config;

mod action;
mod action_handler;
mod config;
mod parser;
mod task;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let file_directory_buff = env::current_dir().unwrap().to_str().unwrap().to_string();

    let config = Config::build(file_directory_buff, &arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _ = action_handler::handle_action(config)?;
    Ok(())
}

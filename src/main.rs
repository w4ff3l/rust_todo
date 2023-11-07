use std::{env, str::FromStr};

use action::Action;

mod action;
mod action_handler;
mod task;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        panic!("No arguments read");
    }

    let raw_action = Action::from_str(&arguments[1].as_str());
    let action = match raw_action {
        Ok(action) => action,
        Err(error) => panic!("Error while determining action: {:?}", error),
    };

    let action_parameters = &arguments[1..arguments.len()];
    action_handler::handle_action(action, action_parameters);
}

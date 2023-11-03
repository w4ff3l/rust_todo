use std::{env, str::FromStr};

use action::Action;

mod action;
mod arguments;
mod task;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    parse_arguments(&arguments);
    dbg!(arguments);
}

fn parse_arguments(arguments: &Vec<String>) {
    let first_argument_index = 1;
    let action_result = Action::from_str(arguments[first_argument_index].as_str());
    let action = match action_result {
        Ok(action) => action,
        Err(error) => panic!("Problem parsing action: {:?}", error),
    };

    if Action::Add.eq(&action) {
        print!("Action is add");
    }
}

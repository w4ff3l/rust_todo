use std::{fs::File, path::Path};
use std::io::prelude::*;

use uuid::Uuid;

use crate::{action::Action, task::Task};

pub fn handle_action(action: Action, action_parameters: &[String]) {
    match action {
        Action::Add => hadle_add(action_parameters),
        Action::Remove => todo!(),
        Action::Complete => todo!(),
    }
}

fn hadle_add(action_parameters: &[String]) {
    if action_parameters.len() != 3 {
        panic!("Incorrect number of arguments. Provide precicesly one description and one priority to add a task.");
    }

    let priority_result = action_parameters[1].parse::<i32>();

    let priority = match priority_result {
        Ok(priority) => priority,
        Err(error) => panic!("Could not parse priority: {:?}", error),
    };

    let task = Task {
        id: Uuid::new_v4(),
        priority,
        description: action_parameters[2].clone(),
    };

    let _ = match write_task_to_file(task) {
        Ok(_) => println!("Task successfully written to file."),
        Err(error) => panic!("Unable to write to file: {:?}", error),
    };
}

fn write_task_to_file(task: Task) -> std::io::Result<()>{
    let path = Path::new("todos.txt");

    let mut file = File::create(path)?;

    file.write_all(task.id.to_string().as_bytes())?;
    file.write_all(b" ")?;
    file.write_all(task.priority.to_string().as_bytes())?;
    file.write_all(b" ")?;
    file.write_all(task.description.as_bytes())?;
    Ok(())
}

#[test]
#[should_panic]
fn panic_too_much_arguments() {
    let action_parameters = vec![
        String::from("add"),
        String::from("1"),
        String::from("Description"),
        String::from("Invalid"),
    ];

    handle_action(Action::Add, &action_parameters);
}

#[test]
#[should_panic]
fn panic_not_enough_arguments() {
    let action_parameters = vec![String::from("add"), String::from("1")];

    handle_action(Action::Add, &action_parameters)
}

use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use crate::parser;
use crate::{action::Action, task::Task, Config};

const TASK_FILE: &str = "todo.txt";

pub fn handle_action(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    match config.action {
        Action::Add => handle_add(config.file_directory, config.action_parameters),
        Action::Remove => todo!(),
        Action::Complete => todo!(),
    }
}

fn handle_add(
    file_directory: String,
    action_parameters: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_string = file_directory.clone() + TASK_FILE;
    let path = Path::new(&path_string);

    let mut tasks = if path.exists() {
        parser::parse_task_file(path)?
    } else {
        Vec::new()
    };

    let next_id = tasks.len() + 1;

    let task_to_add = Task::build(next_id as u32, action_parameters)?;
    tasks.push(task_to_add);
    println!("{:?}", tasks);
    write_tasks(path.to_path_buf(), tasks)?;

    Ok(())
}

fn write_tasks(path: PathBuf, tasks: Vec<Task>) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    for task in tasks {
        write_task(task, &mut file)?;
    }

    Ok(())
}

fn write_task(task: Task, file: &mut File) -> std::io::Result<()> {
    file.write_all(task.id.to_string().as_bytes())?;
    file.write_all(b" ")?;
    file.write_all(task.priority.to_string().as_bytes())?;
    file.write_all(b" ")?;
    file.write_all(task.description.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use tempfile::TempDir;

    use crate::parser;

    use super::*;

    #[test]
    fn tasks_get_written_to_file() {
        let tempdir = TempDir::new().unwrap().path().to_str().unwrap().to_string();
        let path_string = tempdir.clone() + TASK_FILE;
        let path = Path::new(&path_string);

        let task_one = Task {
            id: 1,
            priority: 1,
            description: "First".to_string(),
        };

        let task_two = Task {
            id: 2,
            priority: 2,
            description: "Second".to_string(),
        };

        let action_parameters_task_one = task_one.to_vector();
        let action_parameters_task_two = task_two.to_vector();

        handle_action(Config {
            action: Action::Add,
            action_parameters: action_parameters_task_one,
            file_directory: tempdir.clone(),
        })
        .unwrap();
        handle_action(Config {
            action: Action::Add,
            action_parameters: action_parameters_task_two,
            file_directory: tempdir.clone(),
        })
        .unwrap();

        let tasks = parser::parse_task_file(path).unwrap();
        drop(tempdir);

        assert!(tasks.len() == 2);
        asserst_task(&task_one, &tasks[0]);
        asserst_task(&task_two, &tasks[1]);
    }

    fn asserst_task(expected_task: &Task, actual_task: &Task) {
        assert_eq!(expected_task.id, actual_task.id);
        assert_eq!(expected_task.priority, actual_task.priority);
        assert_str_eq!(expected_task.description, actual_task.description);
    }

    impl Task {
        pub fn to_vector(&self) -> Vec<String> {
            vec![
                self.id.to_string(),
                self.priority.to_string(),
                self.description.clone(),
            ]
        }
    }
}

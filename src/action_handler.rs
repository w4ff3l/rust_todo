use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::{Path, PathBuf};

use ascii_table::{Align, AsciiTable};

use crate::parser;
use crate::{action::Action, task::Task, Config};

const TASK_FILE: &str = "todo.txt";

pub fn handle_action(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    match config.action {
        Action::List => handle_list(config.file_directory),
        Action::Add => handle_add(config.file_directory, config.action_parameters),
        Action::Remove => handle_remove(config.file_directory, config.action_parameters),
        Action::Complete => todo!(),
    }
}

/// Handles the list action.
///
/// Reads all the tasks from disk and prints them formatted as ascii table to the console.
/// To be able to interact with the list, each task is prepended with a virtual id which is determined solely by sorting.
///
/// # Arguments
///
/// * `file_directory` - Directory the tasks file is created in.
fn handle_list(mut file_directory: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    file_directory.push(Path::new(TASK_FILE));

    let mut tasks = parser::parse_task_file(&file_directory)?;
    tasks.sort();

    let ascii_table = create_ascii_table();
    let table_data = create_table_data(tasks);
    ascii_table.print(table_data);

    Ok(())
}

/// Handles the add action.
///
/// To add a task:
/// * The current tasks are read from disk.
/// * The new task is added to that list.
/// * The list of tasks is sorted by priority.
/// * The list is written back to disk.
///
/// # Arguments
///
/// * `file_directory` - Directory the tasks file is created in.
/// * `action_parameters` - Parameters representing the task.
fn handle_add(
    mut file_directory: PathBuf,
    action_parameters: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    file_directory.push(Path::new(TASK_FILE));

    let mut tasks = if file_directory.exists() {
        parser::parse_task_file(&file_directory)?
    } else {
        Vec::new()
    };

    let task_to_add = Task::build(action_parameters)?;

    tasks.push(task_to_add);
    tasks.sort();
    write_tasks(&tasks, file_directory)?;

    Ok(())
}

/// Handles the remove action.
///
/// Deletes a single entry in the file identified by its id.
///
/// # Arguments
///
/// * `file_directory` - Directory the tasks file is created in.
/// * `action_parameters` - Parameter representing the id.
fn handle_remove(
    mut file_directory: PathBuf,
    action_parameters: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if action_parameters.len() > 2 {
        return Err("Too much arguments".into());
    }

    file_directory.push(Path::new(TASK_FILE));
    let mut tasks = parser::parse_task_file(&file_directory)?;
    tasks.sort();

    tasks.remove(action_parameters[1].parse::<usize>()?);

    write_tasks(&tasks, file_directory)?;

    Ok(())
}

/// Writes all tasks to the specified path in the given order.
///
/// # Arguments
///
/// * `path` - The path to the file to be written to. The file will be created if it is not present.
/// * `tasks` - The tasks to be written to the file.
fn write_tasks(tasks: &Vec<Task>, path: PathBuf) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    for task in tasks {
        write_task(task, &mut file)?;
    }

    Ok(())
}

/// Writes a single task to the specified file.
///
/// # Arguments
///
/// * `task` - The task to be written to the file.
/// * `file` - The file bo be written to.
fn write_task(task: &Task, file: &mut File) -> std::io::Result<()> {
    let mut buf_writer = BufWriter::new(file);

    write!(buf_writer, "{} ", task.priority)?;
    writeln!(buf_writer, "{}", task.description)?;

    Ok(())
}

fn create_table_data(tasks: Vec<Task>) -> Vec<Vec<String>> {
    tasks
        .into_iter()
        .enumerate()
        .map(|(index, task)| {
            let mut task_vec = vec![index.to_string()];
            task_vec.append(&mut task.to_string_vector());
            task_vec
        })
        .collect::<Vec<Vec<String>>>()
}

fn create_ascii_table() -> AsciiTable {
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(50);
    ascii_table
        .column(0)
        .set_header("Id")
        .set_align(Align::Center);
    ascii_table
        .column(1)
        .set_header("Priority")
        .set_align(Align::Center);
    ascii_table
        .column(2)
        .set_header("Description")
        .set_align(Align::Left);
    ascii_table
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use tempfile::tempdir;

    use crate::parser;

    use super::*;

    #[test]
    fn tasks_get_written_to_file() {
        let tempdir = tempdir().unwrap().into_path();
        let tasks_file = tempdir.join(TASK_FILE);

        let task_one = Task {
            priority: 1,
            description: "First".to_string(),
        };
        let task_two = Task {
            priority: 2,
            description: "Second".to_string(),
        };

        let action_parameters_task_one = task_one.to_string_vector();
        let action_parameters_task_two = task_two.to_string_vector();

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

        let tasks = parser::parse_task_file(&tasks_file).unwrap();

        assert!(tasks.len() == 2);
        assert_task(&task_one, &tasks[0]);
        assert_task(&task_two, &tasks[1]);
    }

    #[test]
    fn creates_table_data() {
        let task_one = Task {
            priority: 1,
            description: "First".to_string(),
        };
        let task_two = Task {
            priority: 2,
            description: "Second".to_string(),
        };
        let tasks = vec![task_one.clone(), task_two.clone()];

        let table_data = create_table_data(tasks.clone());

        assert_eq!(2, table_data.len());
        assert_eq!(tasks.len(), table_data.len());
        assert_row(&table_data[0], task_one, 0);
        assert_row(&table_data[1], task_two, 1);
    }

    fn assert_row(table_row: &Vec<String>, task: Task, index: i32) {
        assert_eq!(3, table_row.len());
        assert_eq!(index.to_string(), table_row[0]);
        assert_eq!(task.priority.to_string(), table_row[1]);
        assert_eq!(task.description, table_row[2]);
    }

    fn assert_task(expected_task: &Task, actual_task: &Task) {
        assert_eq!(expected_task.priority, actual_task.priority);
        assert_str_eq!(expected_task.description, actual_task.description);
    }
}

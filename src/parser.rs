use std::{fs, io, num::ParseIntError, path::PathBuf};

use thiserror::Error;

use crate::task::Task;

#[derive(Error, Debug)]
pub enum ParseFileError {
    #[error("Cannot read tasks file.")]
    InvalidRead(io::Error),
    #[error("Cannot read line of tasks file.")]
    InvalidLine(ParseIntError),
}

impl From<io::Error> for ParseFileError {
    fn from(error: io::Error) -> Self {
        ParseFileError::InvalidRead(error)
    }
}

impl From<ParseIntError> for ParseFileError {
    fn from(error: ParseIntError) -> Self {
        ParseFileError::InvalidLine(error)
    }
}

pub fn parse_task_file(path: &PathBuf) -> Result<Vec<Task>, ParseFileError> {
    let content = fs::read_to_string(path)?;

    let mut tasks = Vec::new();

    for line in content.lines() {
        let task = parse_task(line)?;
        tasks.push(task);
    }

    Ok(tasks)
}

fn parse_task(line: &str) -> Result<Task, ParseFileError> {
    let tokens: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

    let priority = tokens[0].to_string().parse::<i32>()?;
    let description = tokens[1..tokens.len()].join(" ");

    Ok(Task {
        priority,
        description,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Seek, SeekFrom, Write},
        path::PathBuf,
    };

    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use crate::{
        parser::{parse_task_file, ParseFileError},
        task::Task,
    };

    #[test]
    fn parses_task_file_correctly() {
        let tempfile = NamedTempFile::new().unwrap();
        let path = tempfile.path().to_owned();

        let task1 = Task {
            priority: 1,
            description: "First Task".to_string(),
        };
        let task2 = Task {
            priority: 1,
            description: "Second Task".to_string(),
        };
        let task3 = Task {
            priority: 2,
            description: "Third Task".to_string(),
        };

        writeln!(
            tempfile.as_file(),
            "{} {}",
            task1.priority.to_string(),
            task1.description
        )
        .unwrap();
        writeln!(
            tempfile.as_file(),
            "{} {}",
            task2.priority.to_string(),
            task2.description
        )
        .unwrap();
        writeln!(
            tempfile.as_file(),
            "{} {}",
            task3.priority.to_string(),
            task3.description
        )
        .unwrap();
        tempfile.as_file().seek(SeekFrom::Start(0)).unwrap();

        let tasks = parse_task_file(&path).unwrap();
        drop(tempfile);

        assert!(tasks.len() == 3);
        assert_eq!(task1, tasks[0]);
        assert_eq!(task2, tasks[1]);
        assert_eq!(task3, tasks[2]);
    }

    #[test]
    fn returns_correct_error_if_file_parsing_fails() {
        let tasks_error = parse_task_file(&PathBuf::from("Unknown path"));

        assert!(tasks_error.is_err());
    }

    #[test]
    fn returns_correct_error_if_line_parsing_fails() {
        let tempfile = NamedTempFile::new().unwrap();
        let path = tempfile.path().to_owned();
        writeln!(tempfile.as_file(), "{} {} {}", "a", "b", "description",).unwrap();

        let tasks_error = parse_task_file(&path);

        if let Err(ParseFileError::InvalidLine(_)) = tasks_error {
            assert!(true);
        } else {
            panic!("Expected an InvalidLine error");
        }
    }
}

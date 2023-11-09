use std::{fs, path::Path};

use crate::{parse_file_error::ParseFileError, task::Task};

pub fn parse_task_file(path: &Path) -> Result<Vec<Task>, ParseFileError> {
    let content = fs::read_to_string(path)?;

    let mut tasks = Vec::new();

    for line in content.lines() {
        let task = parse_task(line)?;
        tasks.push(task);
    }

    Ok(tasks)
}

fn parse_task(line: &str) -> Result<Task, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
    if tokens.len() < 3 {
        return Err("Line in tasks file malformed.".to_string());
    }

    let msg = |_| "Failed to parse string";
    let id = tokens[0].to_string().parse::<u32>().map_err(msg)?;
    let priority = tokens[1].to_string().parse::<i32>().map_err(msg)?;
    let description = tokens[2..tokens.len()].join(" ");

    Ok(Task {
        id,
        priority,
        description,
    })
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use crate::{parser::parse_task_file, task::Task};

    #[test]
    fn test_tempfile() {
        let tempfile = NamedTempFile::new().unwrap();
        let path = tempfile.path();
        let task1 = Task { id: 1, priority: 1, description: "First Task".to_string(), };
        let task2 = Task { id: 2, priority: 1, description: "Second Task".to_string(), };
        let task3 = Task { id: 3, priority: 2, description: "Third Task".to_string(), };

        writeln!( tempfile.as_file(), "{} {} {}", task1.id.to_string(), task1.priority.to_string(), task1.description).unwrap();
        writeln!( tempfile.as_file(), "{} {} {}", task2.id.to_string(), task2.priority.to_string(), task2.description).unwrap();
        writeln!( tempfile.as_file(), "{} {} {}", task3.id.to_string(), task3.priority.to_string(), task3.description).unwrap();
        tempfile.as_file().seek(SeekFrom::Start(0)).unwrap();

        let tasks = parse_task_file(path).unwrap();
        drop(tempfile);

        assert!(tasks.len() == 3);
        assert_eq!(task1, tasks[0]);
        assert_eq!(task2, tasks[1]);
        assert_eq!(task3, tasks[2]);
    }
}

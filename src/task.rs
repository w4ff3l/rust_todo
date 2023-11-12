use std::{u32, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: u32,
    pub priority: i32,
    pub description: String,
}

impl Task {
    pub fn build(id: u32, components: Vec<String>) -> Result<Task, ParseIntError> {
        let priority = components[1].parse::<i32>()?;
        let task = Task {
            id,
            priority,
            description: components[2].clone(),
        };
        Ok(task)
    }
}

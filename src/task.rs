use std::{u32, num::ParseIntError};

#[derive(Debug, PartialEq, Clone)]
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

    pub fn to_string_vector(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.priority.to_string(),
            self.description.clone(),
        ]
    }
}

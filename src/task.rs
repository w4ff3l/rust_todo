use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Task {
    pub priority: i32,
    pub description: String,
}

impl Task {
    pub fn build(components: Vec<String>) -> Result<Task, ParseIntError> {
        let priority = components[1].parse::<i32>()?;
        let description = components[2..components.len()].join(" ");
        let task = Task {
            priority,
            description,
        };
        Ok(task)
    }

    pub fn to_string_vector(&self) -> Vec<String> {
        vec![self.priority.to_string(), self.description.clone()]
    }
}


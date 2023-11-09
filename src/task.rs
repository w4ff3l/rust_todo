use std::u32;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: u32,
    pub priority: i32,
    pub description: String,
}

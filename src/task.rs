use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub priority: i32,
    pub description: String,
}

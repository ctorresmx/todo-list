use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Pending,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i64,
    pub content: String,
    pub status: Status,
}

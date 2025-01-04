use std::fs::{self};

use crate::model::Todo;

const DATABASE_PATH: &str = "todo_list.json";

pub fn write(todos: Vec<Todo>) {
    let serialized_list = serde_json::to_string(&todos);
    fs::write(DATABASE_PATH, serialized_list.unwrap()).unwrap();
}

pub fn read() -> Vec<Todo> {
    let serialized_list = fs::read_to_string(DATABASE_PATH).unwrap();
    serde_json::from_str(&serialized_list).unwrap()
}

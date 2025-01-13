//! # Persistance
//!
//! `persistance` is a collection of functions to read/write Todo items
use crate::model::Todo;
use std::fs::{self};

// Path where the items will be written in disk.
const DATABASE_PATH: &str = "todo_list.json";

/// Writes the Todo items to the file path defined in `DATABASE_PATH`
pub fn write(todos: Vec<Todo>) {
    let serialized_list = serde_json::to_string(&todos);
    fs::write(DATABASE_PATH, serialized_list.unwrap()).unwrap();
}

/// Reads the Todo items from the file path defined in `DATABASE_PATH`.
pub fn read() -> Vec<Todo> {
    let serialized_list = fs::read_to_string(DATABASE_PATH).unwrap();
    serde_json::from_str(&serialized_list).unwrap()
}

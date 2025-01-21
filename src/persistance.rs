//! # Persistance
//!
//! `persistance` is a collection of functions to read/write Todo items
use crate::model::Todo;
use dirs::home_dir;
use std::fs::{self};

const FILE_NAME: &str = ".todo-list";

/// Writes the Todo items to the file path defined in `DATABASE_PATH`
pub fn write(todos: Vec<Todo>) {
    let mut file_path = home_dir().unwrap();
    file_path.push(FILE_NAME);

    let serialized_list = serde_json::to_string(&todos);
    fs::write(file_path, serialized_list.unwrap()).unwrap();
}

/// Reads the Todo items from the file path defined in `DATABASE_PATH`.
pub fn read() -> Vec<Todo> {
    let mut file_path = home_dir().unwrap();
    file_path.push(FILE_NAME);

    if !file_path.exists() {
        fs::write(&file_path, "[]").unwrap();
    }

    let serialized_list = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&serialized_list).unwrap()
}

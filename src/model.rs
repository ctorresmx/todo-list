//! # Model
//!
//! `model` contains the data models used by the crate.
use serde::{Deserialize, Serialize};

/// Status of a given Todo note.
///
/// This can be toggled between status.
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    /// The Todo item hasn't been completed.
    Pending,
    /// The Todo item has been completed.
    Completed,
}

/// Base struct for a Todo item
///
/// This will hold the information for a single Todo item
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    /// A unique identifier
    pub id: i64,
    /// The text related to the Todo item
    pub content: String,
    /// The status that can be toggled between Status
    pub status: Status,
}

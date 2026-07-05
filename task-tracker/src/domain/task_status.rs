use mockall::predicate::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // Ensures serde automatically formats variants as lowercase
pub enum TaskStatus {
    Todo,
    #[serde(rename = "in-progress")]
    InProgress,
    Done,
}

// Implement Display for TaskStatus to provide a string representation
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in-progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

// Automatically maps Display into Serde's serialization
impl From<TaskStatus> for String {
    fn from(t: TaskStatus) -> Self {
        t.to_string()
    }
}

use crate::domain::task::TaskStatus;
use std::fmt::{self};

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

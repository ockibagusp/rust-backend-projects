use crate::cmd::domain::models::TaskStatus;
use std::fmt::{self};

pub const TODO: &str = "todo";
pub const IN_PROGRESS: &str = "in-progress";
pub const DONE: &str = "done";

const MEMO: &str = "📝"; // TODO
const HOURGLASS: &str = "⏳"; // waiting
const OK: &str = "✅"; // check mark

// Implement Display for TaskStatus to provide a string representation
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "{TODO} {MEMO}"),
            TaskStatus::InProgress => write!(f, "{IN_PROGRESS} {HOURGLASS}"),
            TaskStatus::Done => write!(f, "{DONE} {OK}"),
        }
    }
}

// Automatically maps Display into Serde's serialization
impl From<TaskStatus> for String {
    fn from(t: TaskStatus) -> Self {
        t.to_string()
    }
}

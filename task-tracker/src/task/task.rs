use chrono::DateTime;
use chrono::prelude::FixedOffset;
use core::result::Result;
use serde::{Deserialize, Serialize};
use std::io::Error;

use mockall::predicate::*;
use mockall::*;

// #[derive(PartialEq, Clone, Debug, SerializeDisplay, DeserializeFromStr)]
// pub enum TaskStatus {
//     Todo,
//     InProgress,
//     Done,
// }

// impl fmt::Display for TaskStatus {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TaskStatus::Todo => write!(f, "todo"),
//             TaskStatus::InProgress => write!(f, "in-progress"),
//             TaskStatus::Done => write!(f, "done"),
//         }
//     }
// }
pub const VALID_STATUSES: [&str; 3] = ["todo", "in-progress", "done"];

// #[derive(PartialEq, Clone, Debug, SerializeDisplay, DeserializeDisplay)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    // status: todo, in-progress, done
    pub status: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[automock]
pub trait TaskTrait {
    fn is_validation(&self) -> Result<(), Error>;
}

impl TaskTrait for Task {
    fn is_validation(&self) -> Result<(), Error> {
        let invalid_input = std::io::ErrorKind::InvalidInput;
        if self.id.is_negative() {
            return Err(Error::new(invalid_input, "`id` is negative"));
        }
        if self.description.trim().is_empty() || self.description.len() > 26 {
            return Err(Error::new(
                invalid_input,
                "`description` is empty or too long",
            ));
        }
        let _valid_statuses = VALID_STATUSES;
        if !matches!(&self.status, _valid_statuses) {
            return Err(Error::new(
                invalid_input,
                format!("`status` is invalid `{}`", &self.status),
            ));
        }
        Ok(())
    }
}

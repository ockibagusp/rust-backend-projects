use chrono::DateTime;
use chrono::prelude::FixedOffset;
use core::result::Result;
use serde::{Deserialize, Serialize};
use std::io::Error;

use mockall::predicate::*;
use mockall::*;

// TDD
// ✅ ❔ ❌
// 2.1. buatlah struktur data Task dengan field id, description, status, created_at, updated_at
// => 2.1. create the Task data structure with fields id, description, status, created_at, updated_at
// ------------------------------------------------
// 1. buat field `id` bertipe integer ✅
// => 1. create the `id` field with integer type
// 2. buat field `description` bertipe string ✅
// => 2. create the `description` field with string type
// 3. buat field `status` bertipe string dengan nilai 'todo', 'in-progress', 'done' ✅
// => 3. create the `status` field with string type with values 'todo', 'in-progress', 'done'
// 4. buat field `created_at` bertipe DateTime dengan zona waktu tetap ✅
// => 4. create the `created_at` field with DateTime type with fixed timezone
// 5. buat field `updated_at` bertipe DateTime dengan zona waktu tetap ✅
// => 5. create the `updated_at` field with DateTime type with fixed timezone
// ------------------------------------------------

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

// TDD
// ✅ ❔ ❌
// 2.2. buatlah validasi untuk struktur data Task ✅
// => 2.2. create the validation for the the Task data structure
// ------------------------------------------------
// 1. pastikan field `id` harus memiliki nilai positif ✅
// => 1. make the `id` field should have a positive value
// 2. pastikan field `description` tidak boleh kosong dan maksimal 26 karakter ✅
// => 2. make the `description` field should not be empty and maximum 26 characters
// 3. pastikan field `status` hanya boleh memiliki nilai 'todo', 'in-progress', atau 'done' ✅
// => 3. make the `status` field should only have values 'todo', 'in-progress', or 'done'
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
        if self.description.trim().is_empty()
            || self.description.len() < 2
            || self.description.len() > 50
        {
            return Err(Error::new(
                invalid_input,
                "`description` is empty or too short(min. 2 chars) or too long(max. 50 chars)",
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

use chrono::{DateTime, prelude::FixedOffset};
use core::result::Result;
use mockall::{automock, predicate::*};
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

// TDD
// ✅ ❔ ❌
// 2.2. buatlah validasi untuk struktur data Task ✅
// => 2.2. create the validation for the Task data structure
// ------------------------------------------------
// 1. pastikan field `id` harus memiliki nilai positif ✅
// => 1. ensure that the `id` field should have a positive value
// 2. pastikan field `description` harus memiliki:
//      - minimal 2 karakter
//      - maksimal 50 karakter ✅
// => 2. ensure that the `description` field should have:
//      - a minimum of 2 characters
//      - a maximum of 50 characters
// 3. pastikan field `status` hanya boleh memiliki nilai 'todo', 'in-progress', atau 'done' ✅
// => 3. ensure that the `status` field should only have values 'todo', 'in-progress', or 'done'
// 4. pastikan field `created_at` harus berisi waktu pertama kali dimasukkan ✅
// => 4. ensure that the `created_at` field should contain the first time it was inserted
// 5. pastikan field `updated_at` harus berisi waktu terakhir kali diubah ✅
// => 5. ensure that the `updated_at` field should contain the last time it was updated
// ------------------------------------------------

// #[derive(PartialEq, Clone, Debug, SerializeDisplay, DeserializeDisplay)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    // status: todo, in-progress, done
    pub status: TaskStatus,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

// TDD
// ✅ ❔ ❌
// 2.3. buatlah trait untuk validasi Task ✅
// => 2.3. create a trait for Task validation
// ------------------------------------------------
// 1. buat trait `TaskTrait` dengan method `is_validation` yang mengembalikan nil atau status kesalahan teks  ✅
// => 1. create the `TaskTrait` trait with the `is_validation` method that returns nil or a text error status
// 2. implementasikan trait `TaskTrait` untuk struct `Task` dengan:
// => 2. implement the `TaskTrait` trait for the `Task` struct with:
//      - validasi untuk field `description`sesuai dengan poin 2.2
//      -> validation for the `description` field according to point 2.2
//      - validasi untuk field `id` harus tidak boleh negatif
//      -> validation for the `id` field should not be negative
//      - validasi untuk field `status` seharusnya hanya memiliki nilai 'todo', 'in-progress', atau 'done'
//      -> validation for the `status` field should only have the values 'todo', 'in-progress', or 'done'
// ------------------------------------------------
#[automock]
pub trait TaskTrait {
    fn is_validation(&self) -> Result<(), &'static str>;
}

impl TaskTrait for Task {
    fn is_validation(&self) -> Result<(), &'static str> {
        if self.id.is_negative() {
            return Err("ID is negative");
        }
        if self.description.trim().is_empty()
            || self.description.len() < 2
            || self.description.len() > 50
        {
            return Err("DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)");
        }
        match self.status {
            TaskStatus::Todo | TaskStatus::InProgress | TaskStatus::Done => (),
        }
        Ok(())
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ id: {}, description: \"{}\", status: {}, created_at: {}, updated_at: {} }}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

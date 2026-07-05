use crate::domain::task_status::TaskStatus;
use chrono::{DateTime, Local, prelude::FixedOffset};
use core::result::Result;
use mockall::{automock, predicate::*};
use serde::{Deserialize, Serialize};
use std::fmt::{self};

// TDD
// ✅ ❔ ❌
// 2.1. buatlah struktur data Task dengan field id, description, status, created_at, updated_at
// => 2.1. create the Task data structure with fields id, description, status, created_at, updated_at
// ------------------------------------------------
// 1. buat field `id` bertipe integer ✅
// => 1. create an `id` field of type integer
// 2. buat field `description` bertipe string ✅
// => 2. create a `description` field of type string
// 3. buat field `status` bertipe TaskStatus dengan nilai-nilai 'todo', 'in-progress', 'done' ✅
// => 3. create a `status` field of type TaskStatus with the values 'todo', 'in-progress', 'done'
// 4. buat field `created_at` bertipe DateTime dengan zona waktu tetap ✅
// => 4. create a `created_at` field of type DateTime with fixed timezone
// 5. buat field `updated_at` bertipe DateTime dengan zona waktu tetap ✅
// => 5. create an `updated_at` field of type DateTime with fixed timezone
// ------------------------------------------------
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
#[automock]
pub trait TaskTrait {
    fn new(id: i32, description: String) -> Self;
    fn is_validation(&self) -> Result<(), &'static str>;
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
impl TaskTrait for Task {
    fn new(id: i32, description: String) -> Self {
        let now: DateTime<FixedOffset> = Local::now().into();
        let status = TaskStatus::Todo; // Default status is 'todo'

        Self {
            id,
            description,
            status,
            created_at: now,
            updated_at: now,
        }
    }

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

// Implement Display for Task to provide a string representation
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ id: {}, description: \"{}\", status: {}, created_at: {}, updated_at: {} }}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

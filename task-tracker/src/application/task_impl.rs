use crate::domain::task::{Task, TaskBuilder, TaskStatus, TaskTrait};
use chrono::{DateTime, FixedOffset, Local};
use core::result::Result;
use std::{
    fmt::{self},
    write,
};

// TDD
// ✅ ❔ ❌
// 4.1. buatlah trait untuk validasi Task ✅
// => 4.1. create a trait for Task validation
// ------------------------------------------------
// 2.1 buat trait `TaskTrait` dengan method `is_validation` yang mengembalikan nil atau status kesalahan teks  ✅
// => 2.1. create the `TaskTrait` trait with the `is_validation` method that returns nil or a text error status
// 2.2. implementasikan trait `TaskTrait` untuk struct `Task` dengan:
// => 2.2. implement the `TaskTrait` trait for the `Task` struct with:
//      - validasi untuk field `description`sesuai dengan poin 2.2
//      -> validation for the `description` field according to point 2.2
//      - validasi untuk field `id` harus tidak boleh negatif
//      -> validation for the `id` field should not be negative
//      - validasi untuk field `status` seharusnya hanya memiliki nilai 'todo', 'in-progress', atau 'done'
//      -> validation for the `status` field should only have the values 'todo', 'in-progress', or 'done'
// ------------------------------------------------
impl TaskTrait for Task {
    fn new(id: i32, description: &str) -> Self {
        let now: DateTime<FixedOffset> = Local::now().into();
        let status = TaskStatus::Todo; // Default status is 'todo'

        Self {
            id: id,
            description: description.to_string(),
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
            "Task {{ id: {:?}, description: {:?}, status: {}, created_at: {}, updated_at: {} }}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

// Automatically maps Display into a String
impl From<Task> for String {
    fn from(t: Task) -> Self {
        t.to_string()
    }
}

// impl TaskTraitBuilder
impl TaskBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }

    pub fn updated_at_new(mut self) -> Self {
        let now: DateTime<FixedOffset> = Local::now().into();

        self.updated_at = now;
        self
    }

    pub fn build(self) -> Task {
        Task {
            id: self.id,
            description: self.description,
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// Implement Display for Task to provide a string representation
impl fmt::Display for TaskBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ id: {}, description: {}, status: {}, created_at: {}, updated_at: {} }}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

// Automatically maps Display into a String
impl From<TaskBuilder> for String {
    fn from(t: TaskBuilder) -> Self {
        t.to_string()
    }
}

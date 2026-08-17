use chrono::{DateTime, prelude::FixedOffset};
use core::result::Result;
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
// // by default, serde renames of all in case-sensitive, except for spaces; for example: "...-..."
// #[serde(rename_all = "lowercase")] // Ensures serde automatically formats variants as lowercase
#[serde(rename_all = "kebab-case")] // Ensures serde automatically formats variants as kebab-case
pub enum TaskStatus {
    Todo,
    // // serde: the name change is "in-progress", not "InProgress" or "inprogress"
    // #[serde(rename = "in-progress")]
    InProgress,
    Done,
}

// TDD
// ✅ ❔ ❌
// 1.1. buatlah struktur data Task dengan field id, description, status, created_at, updated_at
// => 1.1. create the Task data structure with fields id, description, status, created_at, updated_at
// ------------------------------------------------
// 1.1.1. buat field `id` bertipe integer
//    - pastikan field `id` harus memiliki nilai positif ✅
// => 1.1.1. create an `id` field of type integer
//    - ensure that the `id` field should have a positive value
// 1.1.2. buat field `description` bertipe string
//    + pastikan field `description` harus memiliki:
//      - minimal 2 karakter
//      - maksimal 50 karakter ✅
// => 1.1.2. create a `description` field of type string
//    + ensure that the `description` field should have:
//      - a minimum of 2 characters
//      - a maximum of 50 characters
// 1.1.3. buat field `status` bertipe TaskStatus dengan nilai-nilai berikut: 'todo', 'in-progress', 'done' ✅
// => 1.1.3. create a `status` field of type TaskStatus with the following values: 'todo', 'in-progress', 'done'
// 1.1.4. buat field `created_at` bertipe DateTime dengan zona waktu tetap ✅
// => 1.1.4. create a `created_at` field of type DateTime with fixed timezone
// 1.1.5. buat field `updated_at` bertipe DateTime dengan zona waktu tetap ✅
// => 1.1.5. create an `updated_at` field of type DateTime with fixed timezone
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
// 1.2. buatlah method baru dengan parameter `id` dan `description` untuk menginstansiasi dari struct Task ✅
// => 1.2. create a new method with `id` and `description` parameters to an instance of the Task struct
// 2.2. buatlah validasi untuk struktur data Task ✅
// => 2.2. create the validation for the Task data structure
// ------------------------------------------------
// 1. implementasikan trait `TaskTrait` untuk struct `Task` dengan:
// => 1. implement the `TaskTrait` trait for the `Task` struct with:
//      - validasi untuk field `description`sesuai dengan poin 2.2
//      -> validation for the `description` field according to point 2.2
//      - validasi untuk field `id` harus tidak boleh negatif
//      -> validation for the `id` field should not be negative
//      - validasi untuk field `status` seharusnya hanya memiliki nilai 'todo', 'in-progress', atau 'done'
//      -> validation for the `status` field should only have the values 'todo', 'in-progress', or 'done'
// ------------------------------------------------
#[automock]
pub trait TaskTrait {
    fn new(id: i32, description: &str) -> Self;
    fn is_validation(&self) -> Result<(), &'static str>;
}

pub type TaskI32 = i32;

// TDD
// ✅ ❔ ❌
// 1.4. buatlah struct `TaskExtensions` untuk menambahkan method beberapa pada struct `Task` ✅
// => 1.4. create the `TaskExtensions` struct to add some methods to the `Task` struct
// ------------------------------------------------
pub struct TaskExtensions;

// TDD
// ✅ ❔ ❌
// 1.5. buatlah struct `TaskBuilder` untuk merepresentasikan sebuah objek, dengan menggunakan kode konstruksi yang sama seperti pada struct `Task`. ✅
// => 1.5. create the `TaskBuilder` struct to represent an object, using the same construction code as in the `Task` struct.
// ------------------------------------------------
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TaskBuilder {
    pub id: i32,
    pub description: String,
    // status: todo, in-progress, done
    pub status: TaskStatus,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

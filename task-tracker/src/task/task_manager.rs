use crate::file::files::File;
use crate::task::task::{Task, TaskTrait, VALID_STATUSES};
use chrono::{DateTime, Local};
use core::result::Result;
use mockall::*;
use std::io::Error;

// TDD
// ✅ ❔ ❌
// 2.3. buatlah struktur data Task Manager dengan field `file` dan `list`
// => 2.3. create the Task Manager data structure with field `file` and `list`
// ------------------------------------------------
// 1. field `file` di objek File ✅
// => 1. field `file` in File object
// 2. field `list` di daftar Task ✅
// => 2. field `list` in Task list
#[derive(PartialEq, Debug)]
pub struct TaskManager {
    pub file: File,
    pub list: Vec<Task>,
}

#[automock]
pub trait TaskManagerTrait {
    fn new(file_name: &'static str) -> Self;
    fn get_next_id(&self) -> i32;
    fn list(&self) -> Vec<Task>;
    fn add(&mut self, input: &str) -> Result<Task, Error>;
    fn update(&self, id: i32, update_task: &mut Task) -> Result<Task, Error>;
    fn delete(&self, id: i32) -> Result<(), Error>;
}

fn error_invalid_input(message: &str) -> Error {
    return Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("error: {}", message),
    );
}

fn error_not_found_input(message: &str) -> Error {
    Error::new(std::io::ErrorKind::NotFound, format!("error: {}", message))
}

// TDD
// ✅ ❔ ❌
// 2.4. implementasikan trait TaskManagerTrait untuk struct TaskManager ✅
// => 2.4. implement the TaskManagerTrait trait for the TaskManager struct
// ------------------------------------------------
// 1. fungsi `new` untuk inisialisasi TaskManager ✅
// => 1. `new` function for TaskManager initialization
// 2. fungsi `get_next_id` untuk mendapatkan ID berikutnya ✅
// => 2. `get_next_id` function to get the next ID
// 3. fungsi `list` untuk mendapatkan daftar Task ✅
// => 3. `list` function to get the Task list
// 4. fungsi `add` untuk menambahkan Task baru ✅
// => 4. `add` function to add a new Task
// 5. fungsi `update` untuk memperbarui Task yang ada ✅
// => 5. `update` function to update an existing Task
// 6. fungsi `delete` untuk menghapus Task berdasarkan ID ✅
// => 6. `delete` function to delete a Task by ID
impl TaskManagerTrait for TaskManager {
    fn new(file_name: &'static str) -> Self {
        let file = File::new(file_name);
        let _list = file.list();

        Self { file, list: _list }
    }

    fn get_next_id(&self) -> i32 {
        let mut max_id = 0;
        for task in &self.list {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        max_id + 1
    }

    fn list(&self) -> Vec<Task> {
        let tasks_list = self.file.list();
        tasks_list
    }

    fn add(&mut self, input: &str) -> Result<Task, Error> {
        let next_id = self.get_next_id();
        // Convert UTC to Jakarta time
        let now_created_at: DateTime<Local> = Local::now();

        let add_task = Task {
            id: next_id,
            description: String::from(input),
            // status: "todo"
            status: VALID_STATUSES[0].to_string(),
            created_at: now_created_at.into(),
            updated_at: now_created_at.into(),
        };

        let err = add_task.is_validation();
        // if let Err(e) = err {...}
        if err.is_err() {
            return Err(err.unwrap_err());
        }

        let _ = &self.list.push(add_task.clone());
        let _ = &self.file.add(add_task.clone());

        Ok(add_task)
    }

    fn update(&self, id: i32, update_task: &mut Task) -> Result<Task, Error> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(e);
        }

        let old_task = self.list.iter().find(|&task| task.id == id).unwrap();
        if old_task.id != update_task.id {
            return Err(error_invalid_input("`id` is not identical"));
        }
        if old_task.description == update_task.description {
            return Err(error_invalid_input("`description` is identical"));
        }
        update_task.updated_at = Local::now().into();

        let _ = &self.file.update(id, update_task);

        Ok(update_task.clone())
    }

    fn delete(&self, id: i32) -> Result<(), Error> {
        let task = self.list.iter().find(|&task| task.id == id);
        if task.is_none() {
            return Err(error_not_found_input("`id` is not found"));
        }

        let index = self.list().iter().position(|t| t.id == id).unwrap();
        self.list().remove(index);
        Ok(())
    }
}

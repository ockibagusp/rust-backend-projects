use crate::file::files::File;
use crate::task::task::{Task, TaskTrait, VALID_STATUSES};
use chrono::{DateTime, Local};
use core::result::Result;
use mockall::*;
use std::io::{Error, ErrorKind};

// TDD
// ✅ ❔ ❌
// 2.3. buatlah struktur data Task Manager dengan field `file` dan `list`
// => 2.3. create the Task Manager data structure with field `file` and `list`
// ------------------------------------------------
// 1. buat field `file` bertipe objek File ✅
// => 1. make a `file` field with an object type of File
// 2. buat field `list` dengan tipe array meliputi Task ✅
// => 2. make a `list` field with an array type of Tasks
#[derive(PartialEq, Debug)]
pub struct TaskManager {
    pub file: File,
    // func.: get_next_id() is only for getting the next id
    pub list: Vec<Task>,
}

// > It's me, not Github Copilot (AI)!
// fungsi (bukan impl...for...): untuk memberitahukan jika ada pesan error yang diinput salah
// => function (not impl...for...): to notify if an error message for an invalid input error
fn error_invalid_input(message: &str) -> Error {
    return Error::new(ErrorKind::InvalidInput, format!("error: {}", message));
}

// > It's me, not Github Copilot (AI)!
// fungsi: untuk memberitahukan bahwa jika pesan error yang input tidak ditemukan
// => function: to notify that if an error message for a not found input error
fn error_not_found_input(message: &str) -> Error {
    return Error::new(ErrorKind::NotFound, format!("error: {}", message));
}

fn error_kind(err: Error) -> Error {
    Error::new(err.kind(), format!("error: {}", err))
}

#[automock]
pub trait TaskManagerTrait {
    fn new(file_name: &'static str) -> Self;
    fn get_next_id(&self) -> i32;
    fn find_by_id(&self, id: i32) -> Result<Task, Error>;
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&mut self, input: &str) -> Result<Task, Error>;
    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, Error>;
    fn update(&mut self, id: i32, update_task: &mut Task) -> Result<Task, Error>;
    fn delete(&mut self, id: i32) -> Result<(), Error>;
}

// TDD
// ✅ ❔ ❌
// 2.4. implementasikan trait TaskManagerTrait untuk struct TaskManager ✅
// => 2.4. implement the TaskManagerTrait trait for the TaskManager struct
// ------------------------------------------------
// 1. method `new` untuk inisialisasi TaskManager ✅
// => 1. `new` method for TaskManager initialization
// 2. method `get_next_id` untuk mendapatkan ID berikutnya ✅
// => 2. `get_next_id` method to get the next ID
// 3. method `list` untuk mendapatkan daftar Task ✅
// => 3. `list` method to get the Task list
// 4. method `add` untuk menambahkan Task baru ✅
// => 4. `add` method to add a new Task
// 5. method `update_description` untuk memperbarui deskripsi Task berdasarkan ID ✅
// => 5. `update_description` method to update the Task description by ID
// 6. method `update` untuk memperbarui Task yang ada ✅
// => 6. `update` method to update an existing Task
// 7. method `delete` untuk menghapus Task berdasarkan ID ✅
// => 7. `delete` method to delete a Task by ID
impl TaskManagerTrait for TaskManager {
    fn new(file_name: &'static str) -> Self {
        let file = File::new(file_name);
        let list = file.list();

        Self { file, list }
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

    fn find_by_id(&self, id: i32) -> Result<Task, Error> {
        let task = self.list.iter().find(|&task| task.id == id).cloned();
        match task {
            Some(task) => Ok(task),
            None => Err(error_not_found_input("`id` is not found")),
        }
    }

    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> () {
    //     self.list.iter_mut().find(|task| task.id == id).map(|task| {
    //         *task = update_task.clone();
    //     });
    // }

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
            return Err(error_kind(err.unwrap_err()));
        }

        let _ = &self.file.add(add_task.clone());
        // ? let _ = &self.list.push(add_task.clone());

        Ok(add_task)
    }

    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, Error> {
        let task = self.find_by_id(id);
        if let Err(e) = task {
            return Err(error_kind(e));
        }

        let mut task_to_update = task.unwrap();
        task_to_update.description = description.to_string();
        match self.update(id, &mut task_to_update) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(error_kind(e)),
        }
    }

    fn update(&mut self, id: i32, update_task: &mut Task) -> Result<Task, Error> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(error_kind(e));
        }

        let old_task = self.find_by_id(id).unwrap();
        if old_task.id != update_task.id {
            return Err(error_invalid_input("`id` is not identical"));
        }
        update_task.updated_at = Local::now().into();

        let _ = self.file.update(id, update_task);
        // ? let _ = self.find_by_id_mut(id, update_task);

        Ok(update_task.clone())
    }

    fn delete(&mut self, id: i32) -> Result<(), Error> {
        let task = self.list.iter().find(|&task| task.id == id);
        if task.is_none() {
            return Err(error_not_found_input("`id` is not found"));
        }

        let _ = self.file.delete(id);
        // tidak perlu menghapus
        // ? self.list.remove(index);
        Ok(())
    }
}

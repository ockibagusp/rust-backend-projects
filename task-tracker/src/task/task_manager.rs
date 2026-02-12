use crate::error::{error_invalid_input, error_invalid_input_str, error_not_found_input};
use crate::file::files::File;
use crate::task::task::{Task, TaskTrait, VALID_STATUSES};
use chrono::{DateTime, Local};
use core::result::Result;
use mockall::*;
use std::io::Error;

const FILE_NAME: &str = "TASK_MANAGER";

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

#[automock]
pub trait TaskManagerTrait {
    fn new(file_name: &'static str) -> Self;
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&mut self, input: &str) -> Result<Task, Error>;
    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, Error>;
    fn updates(&mut self, id: i32, update_task: &mut Task) -> Result<Task, Error>;
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

    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> () {
    //     self.list.iter_mut().find(|task| task.id == id).map(|task| {
    //         *task = update_task.clone();
    //     });
    // }

    fn add(&mut self, input: &str) -> Result<Task, Error> {
        let add_task = get_next_task_of_add(&self.list, input);
        // if let Err(e) = err {...}
        if add_task.is_err() {
            let err_string = error_invalid_input_str(FILE_NAME, add_task.unwrap_err());
            return Err(error_not_found_input::<String>(
                FILE_NAME,
                &err_string.to_string(),
            ));
        }
        let add_task = add_task.unwrap();

        let _ = &self.file.add(&add_task);
        // ? let _ = &self.list.push(add_task);

        Ok(add_task)
    }

    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, Error> {
        let task = find_by_id(&self.list, id);
        if let Err(e) = task {
            return Err(error_not_found_input::<String>(FILE_NAME, e));
        }
        let mut task_to_update = task.unwrap();
        task_to_update.description = description.to_string();

        match self.updates(id, &mut task_to_update) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(e),
        }
    }

    fn updates(&mut self, id: i32, update_task: &mut Task) -> Result<Task, Error> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(error_invalid_input::<String>(FILE_NAME, e));
        }

        let is_valid = is_not_similar_to_task_of_description_update(&self.list, id, update_task);
        if is_valid {
            return Err(error_invalid_input_str(
                FILE_NAME,
                "`description` is not identical",
            ));
        }
        update_task.updated_at = Local::now().into();

        let _ = self.file.update(id, update_task);
        // ? let _ = self.find_by_id_mut(id, update_task);

        Ok(update_task.clone())
    }

    fn delete(&mut self, id: i32) -> Result<(), Error> {
        let task = find_by_id(&self.list, id);
        if !task.is_ok() {
            return Err(error_not_found_input::<&str>(FILE_NAME, task.unwrap_err()));
        }

        let _ = self.file.delete(id);
        // tidak perlu menghapus
        // ? self.list.remove(index);
        Ok(())
    }
}

// not trait
fn get_next_id(list: &Vec<Task>) -> i32 {
    let mut max_id = 0;
    for task in list {
        if task.id > max_id {
            max_id = task.id;
        }
    }
    max_id + 1
}

pub fn find_by_id(list: &Vec<Task>, id: i32) -> Result<Task, &str> {
    let task = list.iter().find(|&task| task.id == id).cloned();
    match task {
        Some(task) => Ok(task),
        None => Err("`id` is not found"),
    }
}

pub fn get_next_task_of_add(list: &Vec<Task>, description: &str) -> Result<Task, &'static str> {
    let next_id = get_next_id(list);
    // Convert UTC to Jakarta time
    let now_created_at: DateTime<Local> = Local::now();

    let add_task = Task {
        id: next_id,
        description: description.to_string(),
        // status: "todo"
        status: VALID_STATUSES[0].to_string(),
        created_at: now_created_at.into(),
        updated_at: now_created_at.into(),
    };

    match add_task.is_validation() {
        Ok(_) => Ok(add_task),
        Err(e) => Err(e),
    }
}

pub fn is_not_similar_to_task_of_description_update(
    list: &Vec<Task>,
    id: i32,
    update_task: &mut Task,
) -> bool {
    let old_task = find_by_id(list, id).unwrap();
    if old_task.description == update_task.description {
        return false;
    }
    true
}

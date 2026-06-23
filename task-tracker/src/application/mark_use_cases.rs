use crate::domain::task::{Task, TaskTrait, VALID_STATUSES};
use crate::error::{error_invalid_input, error_invalid_input_str};
use crate::infrastructure::storages::storage::StorageTrait;
use chrono::{DateTime, Local};
use core::result::Result;
use std::io::Error;

pub const FILE_NAME: &str = "MARK";

pub trait MarkRepositoryTrait {
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error>;
    fn mark_done(&mut self, id: i32) -> Result<Task, Error>;
}

pub trait MarkUseCaseTrait {
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error>;
    fn mark_done(&mut self, id: i32) -> Result<Task, Error>;
}

pub struct MarkUseCase {
    pub repository: Box<dyn MarkRepositoryTrait>,
    pub storage: Box<dyn StorageTrait>,
}
// TDD
// ✅ ❔ ❌
// 2.4. implementasikan trait MarkUseCaseTrait untuk struct Mark ✅
// => 2.4. implement the MarkUseCaseTrait trait for the Mark struct
// ------------------------------------------------
// 1. method `new` untuk inisialisasi Mark ✅
// => 1. `new` method for Mark initialization
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
impl MarkUseCaseTrait for MarkUseCase {
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error> {
        return self.repository.mark_in_progress(id);
    }

    fn mark_done(&mut self, id: i32) -> Result<Task, Error> {
        return self.repository.mark_done(id);
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

pub fn find_by_id(list: &Vec<Task>, id: i32) -> Result<Task, Error> {
    let task = list.iter().find(|&task| task.id == id).cloned();
    println!("find_by_id: id={}, task={:?}", id, task);
    match task {
        Some(task) => Ok(task),
        None => Err(error_invalid_input::<String>(FILE_NAME, "ID is not found")),
    }
}

pub fn get_next_task_of_add(list: &Vec<Task>, description: &str) -> Result<Task, Error> {
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
        Err(e) => Err(error_invalid_input::<String>(FILE_NAME, e)),
    }
}

type TaskI32 = i32;
pub const DESCRIPTION: TaskI32 = 0;
pub const STATUS: TaskI32 = 1;
pub fn is_valid_to_task_of_description_or_status_update(
    list: &Vec<Task>,
    id: i32,
    update_task: &Task,
    desc_status: TaskI32,
) -> bool {
    let old_task = find_by_id(list, id).unwrap();
    if desc_status == DESCRIPTION && old_task.description == update_task.description {
        return true;
    }
    if desc_status == STATUS && old_task.status == update_task.status {
        return true;
    }
    false
}

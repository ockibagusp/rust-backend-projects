use crate::domain::task::{Task, TaskStatus, TaskTrait};
use crate::error::AppError;
use chrono::{DateTime, Local};
use core::result::Result;

const FILE_NAME: &str = "TASK_MANAGER";

pub trait TaskManagerRepositoryTrait {
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&mut self, input: &str) -> Result<Task, AppError>;
    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, AppError>;
    fn updates(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError>;
    fn delete(&mut self, id: i32) -> Result<(), AppError>;

    fn _list(&self) -> Vec<Task>;
}

pub trait TaskManagerUseCaseTrait {
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&mut self, input: &str) -> Result<Task, AppError>;
    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, AppError>;
    fn updates(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError>;
    fn delete(&mut self, id: i32) -> Result<(), AppError>;
}

pub struct TaskManagerUseCase {
    pub repository: Box<dyn TaskManagerRepositoryTrait>,
}
// TDD
// ✅ ❔ ❌
// 2.4. implementasikan trait TaskManagerUseCaseTrait untuk struct TaskManager ✅
// => 2.4. implement the TaskManagerUseCaseTrait trait for the TaskManager struct
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
impl TaskManagerUseCaseTrait for TaskManagerUseCase {
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> () {
    //     self.list.iter_mut().find(|task| task.id == id).map(|task| {
    //         *task = update_task.clone();
    //     });
    // }

    fn add(&mut self, input: &str) -> Result<Task, AppError> {
        let add_task = get_next_task_of_add(&self.repository._list(), input);
        // if let Err(e) = err {...}
        if add_task.is_err() {
            return add_task;
        }
        let add_task = add_task.unwrap();

        let _ = &self.repository.add(&add_task.description);

        Ok(add_task)
    }

    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, AppError> {
        let mut task = find_by_id(&self.repository._list(), id)?;
        // if let Err(e) = task {
        //     return Err(e);
        // }
        // let mut task_to_update = task.unwrap();
        // task_to_update.description = description.to_string();
        task.description = description.to_string();

        match self.updates(id, &mut task, DESCRIPTION) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(e),
        }
    }

    fn updates(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(AppError::InvalidInput(FILE_NAME, e));
        }

        let is_valid = is_valid_to_task_of_description_or_status_update(
            &self.repository._list(),
            id,
            update_task,
            desc_status,
        );
        if is_valid {
            return Err(AppError::InvalidInput(
                FILE_NAME,
                "DESCRIPTION or STATUS is not identical",
            ));
        }
        update_task.updated_at = Local::now().into();

        let _ = &self.repository.updates(id, update_task, desc_status);
        // ? let _ = self.find_by_id_mut(id, update_task);

        Ok(update_task.clone())
    }

    fn delete(&mut self, id: i32) -> Result<(), AppError> {
        let task = find_by_id(&self.repository._list(), id);
        if !task.is_ok() {
            return Err(task.unwrap_err());
        }

        let _ = self.repository.delete(id);
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

pub fn find_by_id(list: &Vec<Task>, id: i32) -> Result<Task, AppError> {
    let task = list.iter().find(|&task| task.id == id).cloned();
    match task {
        Some(task) => Ok(task),
        None => Err(AppError::InvalidInput(FILE_NAME, "ID is not found")),
    }
}

pub fn get_next_task_of_add(list: &Vec<Task>, description: &str) -> Result<Task, AppError> {
    let next_id = get_next_id(list);
    // Convert UTC to Jakarta time
    let now_created_at: DateTime<Local> = Local::now();

    let add_task = Task {
        id: next_id,
        description: description.to_string(),
        // status: "todo"
        status: TaskStatus::Todo,
        created_at: now_created_at.into(),
        updated_at: now_created_at.into(),
    };

    match add_task.is_validation() {
        Ok(_) => Ok(add_task),
        Err(e) => Err(AppError::InvalidInput(FILE_NAME, e)),
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

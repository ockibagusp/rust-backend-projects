use crate::application::ports::task_manager_repository::TaskManagerRepository;
use crate::domain::{
    error::AppError,
    task::{Task, TaskExtensions, TaskTrait},
};
use core::result::Result;

const FILE_NAME: &str = "TASK_MANAGER";

pub struct TaskManagerUseCase {
    pub repository: Box<dyn TaskManagerRepository>,
}
// TDD
// ✅ ❔ ❌
// 3.2. implementasikan trait TaskManagerUseCaseTrait untuk struct TaskManager ✅
// => 3.2. implement the TaskManagerUseCaseTrait trait for the TaskManager struct
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
impl TaskManagerUseCase {
    // ? fn find_by_id_mut(&self, id: i32, update_task: &Task) -> () {
    //     self.list.iter_mut().find(|task| task.id == id).map(|task| {
    //         *task = update_task.clone();
    //     });
    // }

    pub fn add(&self, input: &str) -> Result<Task, AppError> {
        let add_task =
            TaskExtensions::get_next_task_of_add(FILE_NAME, &self.repository.find_by_list(), input);
        // if let Err(e) = err {...}
        if add_task.is_err() {
            return add_task;
        }
        let add_task = add_task.unwrap();

        let _ = self.repository.add(add_task.clone());

        Ok(add_task)
    }

    pub fn update_description(&self, id: i32, description: &str) -> Result<Task, AppError> {
        let mut task = TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), id)?;
        // if let Err(e) = task {
        //     return Err(e);
        // }
        // let mut task_to_update = task.unwrap();
        // task_to_update.description = description.to_string();
        task.description = description.to_string();

        match self.updates(id, &mut task, TaskExtensions::DESCRIPTION) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(e),
        }
    }

    pub fn updates(
        &self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(AppError::InvalidInput(FILE_NAME, e));
        }

        let is_valid = TaskExtensions::is_valid_to_task_of_description_or_status_update(
            FILE_NAME,
            &self.repository.find_by_list(),
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

        let _ = &self.repository.updates(id, update_task);
        // ? let _ = self.repository.updates(id, update_task);

        Ok(update_task.clone())
    }

    pub fn delete(&self, id: i32) -> Result<(), AppError> {
        let task = TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), id);
        if !task.is_ok() {
            return Err(task.unwrap_err());
        }

        let _ = self.repository.delete(id);
        // tidak perlu menghapus
        // ? self.repository.delete(id);
        Ok(())
    }
}

use crate::cmd::{
    domain::{
        error::AppError,
        models::{Task, TaskExtensions, TaskTrait},
    },
    infrastructure::storages::storage::StorageTrait,
};
use core::result::Result;

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

// TDD
// ✅ ❔ ❌
// 2.2. buatlah trait TaskManagerRepository dengan method add, update_description, updates, delete, find_by_list ✅
// => 2.2. create the TaskManagerRepository trait with method add, update_description, updates, delete, find_by_list
pub trait TaskManagerRepository {
    fn new(storage: Box<dyn StorageTrait>) -> Self
    where
        Self: Sized;
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&self, add_task: &Task) -> Result<Task, AppError>;
    fn update_description(&self, id: i32, update_task: &Task) -> Result<Task, AppError>;
    fn updates(&self, id: i32, update_task: &Task) -> Result<Task, AppError>;
    fn delete(&self, id: i32) -> Result<(), AppError>;

    fn find_by_list(&self) -> Vec<Task>;
}

const FILE_NAME: &str = "TASK_MANAGER";
const ERR_NOT_IDENTICAL: &str = "DESCRIPTION or STATUS is not identical";

// The DTO (Data Transfer Object) for this specific use case
pub struct CreateTaskManagerDto {
    pub description: String,
}

pub struct CreateTaskManagerUseCase<R>
where
    R: TaskManagerRepository,
{
    pub repository: R,
}

impl<R> CreateTaskManagerUseCase<R>
where
    R: TaskManagerRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, request: CreateTaskManagerDto) -> Result<Task, AppError> {
        let result = TaskExtensions::get_next_task_of_add(
            FILE_NAME,
            &self.repository.find_by_list(),
            &request.description,
        ); // not ? operator
        if let Err(e) = result {
            return Err(e);
        }

        let add_task = &result.unwrap();
        self.repository.add(add_task)
    }
}

pub struct UpdateTaskManagerDto {
    pub id: i32,
    pub description: String,
}

pub struct UpdateTaskManagerUseCase<R>
where
    R: TaskManagerRepository,
{
    pub repository: R,
}

impl<R> UpdateTaskManagerUseCase<R>
where
    R: TaskManagerRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute_update_desc(&self, request: UpdateTaskManagerDto) -> Result<Task, AppError> {
        let mut task =
            TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), request.id)?;
        // if let Err(e) = task {
        //     return Err(e);
        // }
        // let mut task_to_update = task.unwrap();
        // task_to_update.description = description.to_string();
        task.description = request.description;

        match self.execute_update(request.id, &mut task, TaskExtensions::DESCRIPTION) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(e),
        }
    }

    pub fn execute_update(
        &self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        let err = update_task.is_validation();
        // if let Err(e) = err {...}
        if err.is_err() {
            return Err(AppError::InvalidInput(FILE_NAME, err.unwrap_err()));
        }

        let is_valid = TaskExtensions::is_valid_to_task_of_description_or_status_update(
            FILE_NAME,
            &self.repository.find_by_list(),
            id,
            update_task,
            desc_status,
        );
        if is_valid {
            return Err(AppError::InvalidInput(FILE_NAME, ERR_NOT_IDENTICAL));
        }

        let _ = self.repository.updates(id, update_task);
        // ? let _ = self.repository.updates(id, update_task);

        let update_task = update_task.clone();
        Ok(update_task)
    }
}

pub struct DeleteTaskManagerDto {
    pub id: i32,
}

pub struct DeleteTaskManagerUseCase<R>
where
    R: TaskManagerRepository,
{
    pub repository: R,
}

impl<R> DeleteTaskManagerUseCase<R>
where
    R: TaskManagerRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, request: DeleteTaskManagerDto) -> Result<(), AppError> {
        let task =
            TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), request.id);
        if !task.is_ok() {
            return Err(task.unwrap_err());
        }

        let _ = self.repository.delete(request.id);
        // tidak perlu menghapus
        // ? self.repository.delete(id);
        Ok(())
    }
}

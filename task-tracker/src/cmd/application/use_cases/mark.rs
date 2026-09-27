use crate::cmd::domain::{
    error::AppError,
    models::{Task, TaskExtensions, TaskStatus},
};
use core::result::Result;

// TDD
// ✅ ❔ ❌
// 2.3. buatlah trait MarkRepository dengan method mark_in_progress, mark_done, find_by_list ✅
// => 2.3. create the MarkRepository trait with method mark_in_progress, mark_done, find_by_list
pub trait MarkRepository {
    fn in_progress(&self, id: i32, update_task: &mut Task) -> Result<Task, AppError>;
    fn done(&self, id: i32, update_task: &mut Task) -> Result<Task, AppError>;

    fn find_by_list(&self) -> Vec<Task>;
}

pub const FILE_NAME: &str = "MARK";
const ERR_IN_PROGRESS: &str = "task is already in 'in-progress' status";
const ERR_DONE: &str = "task is already in 'done' status";

// The DTO (Data Transfer Object) for this specific use case
pub struct MarkDto {
    pub id: i32,
}

pub struct MarkInProgressUseCase {
    pub repository: Box<dyn MarkRepository>,
}
// TDD
// ✅ ❔ ❌
// 3.3. implementasikan trait MarkUseCaseTrait untuk struct Mark ✅
// => 3.3. implement the MarkUseCaseTrait trait for the Mark struct
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
impl MarkInProgressUseCase {
    pub fn new(repository: Box<dyn MarkRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, request: MarkDto) -> Result<Task, AppError> {
        let mut task_to_update =
            TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), request.id)?;

        if task_to_update.status == TaskStatus::InProgress {
            return Err(AppError::InvalidInput(FILE_NAME, ERR_IN_PROGRESS));
        }

        task_to_update.status = TaskStatus::InProgress;
        return self.repository.in_progress(request.id, &mut task_to_update);
    }
}

// USE CASE: Orchestrates business logic
pub struct MarkDoneUseCase {
    pub repository: Box<dyn MarkRepository>,
}

impl MarkDoneUseCase {
    pub fn new(repository: Box<dyn MarkRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, request: MarkDto) -> Result<Task, AppError> {
        let mut task_to_update =
            TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), request.id)?;

        if task_to_update.status == TaskStatus::Done {
            return Err(AppError::InvalidInput(FILE_NAME, ERR_DONE));
        }

        task_to_update.status = TaskStatus::Done;
        return self.repository.done(request.id, &mut task_to_update);
    }
}

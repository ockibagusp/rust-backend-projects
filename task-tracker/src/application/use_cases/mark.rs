use crate::application::ports::mark_repository::MarkRepository;
use crate::domain::{
    error::AppError,
    task::{Task, TaskExtensions, TaskStatus},
};
use core::result::Result;

pub const FILE_NAME: &str = "MARK";

pub struct MarkUseCase {
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
impl MarkUseCase {
    pub fn mark_in_progress(&self, id: i32) -> Result<Task, AppError> {
        let mut task_to_update =
            TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), id)?;

        if task_to_update.status == TaskStatus::InProgress {
            return Err(AppError::InvalidInput(
                FILE_NAME,
                "task is already in 'in-progress' status",
            ));
        }

        task_to_update.status = TaskStatus::InProgress;
        return self.repository.mark_in_progress(id, &mut task_to_update);
    }

    pub fn mark_done(&self, id: i32) -> Result<Task, AppError> {
        let mut task_to_update =
            TaskExtensions::find_by_id(FILE_NAME, &self.repository.find_by_list(), id)?;

        if task_to_update.status == TaskStatus::Done {
            return Err(AppError::InvalidInput(
                FILE_NAME,
                "task is already in 'done' status",
            ));
        }

        task_to_update.status = TaskStatus::Done;
        return self.repository.mark_done(id, &mut task_to_update);
    }
}

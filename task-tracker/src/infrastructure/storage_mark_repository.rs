use crate::application::mark_use_cases::FILE_NAME as MARK_FILE_NAME;
use crate::domain::{mark_repository::MarkRepository, task::Task, task_status::TaskStatus};
use crate::error::AppError;
use crate::infrastructure::{
    storage_task_manager_repository::find_by_id, storages::storage::StorageTrait,
};

pub struct StorageMarkRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl MarkRepository for StorageMarkRepository {
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, AppError> {
        let mut task_to_update = find_by_id(&self.storage.find_by_list(), id, MARK_FILE_NAME)?;

        if task_to_update.status == TaskStatus::InProgress {
            return Err(AppError::InvalidInput(
                MARK_FILE_NAME,
                "task is already in 'in-progress' status",
            ));
        }

        task_to_update.status = TaskStatus::InProgress;
        let _ = self.storage.update(id, &mut task_to_update); // ? operator
        // if let Err(e) = self.task_manager.updates(id, &mut task_to_update) {
        //     return Err(e);
        // }
        // // link: https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try
        Ok(task_to_update)
    }

    fn mark_done(&mut self, id: i32) -> Result<Task, AppError> {
        let mut task_to_update = find_by_id(&self.storage.find_by_list(), id, MARK_FILE_NAME)?;

        if task_to_update.status == TaskStatus::Done {
            return Err(AppError::InvalidInput(
                MARK_FILE_NAME,
                "task is already in 'done' status",
            ));
        }

        task_to_update.status = TaskStatus::Done;

        let _ = self.storage.update(id, &mut task_to_update);
        Ok(task_to_update)
    }
}

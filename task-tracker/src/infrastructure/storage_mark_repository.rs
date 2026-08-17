use crate::application::ports::mark_repository::MarkRepository;
use crate::domain::{error::AppError, task::Task};
use chrono::{Local, Utc};

use crate::infrastructure::storages::storage::StorageTrait;

pub const FILE_NAME: &str = "STORAGE_MARK_REPOSITORY";

pub struct StorageMarkRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl MarkRepository for StorageMarkRepository {
    fn mark_in_progress(&self, id: i32, update_task: &mut Task) -> Result<Task, AppError> {
        update_task.updated_at = Utc::now().with_timezone(&Local).into();
        let _ = &self.storage.update(id, update_task);

        let task: Task = update_task.clone();
        Ok(task)
    }

    fn mark_done(&self, id: i32, update_task: &mut Task) -> Result<Task, AppError> {
        update_task.updated_at = Local::now().into();
        let _ = &self.storage.update(id, update_task);

        let task: Task = update_task.clone();
        Ok(task)
    }

    fn find_by_list(&self) -> Vec<Task> {
        self.storage.find_by_list()
    }
}

#[cfg(test)]
pub mod mark_tests {
    use super::*;
    use crate::{
        domain::task::TaskStatus, domain::task_test::setup_task_status,
        infrastructure::mock::mock::MockStorage,
    };
    #[test]
    fn test_mark_of_in_progress() {
        let repo = StorageMarkRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = repo
            .mark_in_progress(
                1,
                &mut setup_task_status(
                    1,
                    "test update description one is in-progress",
                    TaskStatus::InProgress,
                ),
            )
            .unwrap();
        assert_eq!(tasks.id, 1);
        assert_eq!(
            tasks.description,
            "test update description one is in-progress"
        );
        assert_eq!(tasks.status, TaskStatus::InProgress);
    }

    #[test]
    fn test_mark_of_done() {
        let repo = StorageMarkRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = repo
            .mark_done(
                1,
                &mut setup_task_status(1, "test update description one is done", TaskStatus::Done),
            )
            .unwrap();
        assert_eq!(tasks.id, 1);
        assert_eq!(tasks.description, "test update description one is done");
        assert_eq!(tasks.status, TaskStatus::Done);
    }
}

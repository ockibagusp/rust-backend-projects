use crate::application::ports::task_manager_repository::TaskManagerRepository;
use crate::domain::{error::AppError, task::Task};
use crate::infrastructure::storages::storage::StorageTrait;
use chrono::Local;

pub const FILE_NAME: &str = "STORAGE_TASK_MANAGER_REPOSITORY";

pub struct StorageTaskManagerRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl TaskManagerRepository for StorageTaskManagerRepository {
    // SQL: "INSERT INTO foo (foo_id, ..., updated_at)
    //      VALUES (1, ..., NOW())";
    fn add(&self, add_task: Task) -> Result<Task, AppError> {
        let add_task = Task {
            updated_at: Local::now().into(),
            ..add_task
        };
        let _ = self.storage.add(&add_task);
        Ok(add_task)
    }

    // SQL: "UPDATE foo SET description = 'new description', updated_at = NOW() WHERE foo_id = 1";
    fn update_description(&self, id: i32, update_task: &Task) -> Result<Task, AppError> {
        let update_task = Task {
            updated_at: Local::now().into(),
            ..update_task.clone()
        };
        match self.updates(id, &update_task) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(e),
        }
    }

    // SQL: "UPDATE foo SET description = 'updated description', status = 'todo', updated_at = NOW() WHERE foo_id = 1";
    fn updates(&self, id: i32, update_task: &Task) -> Result<Task, AppError> {
        let update_task = Task {
            updated_at: Local::now().into(),
            ..update_task.clone()
        };
        let _ = self.storage.update(id, &update_task);
        Ok(update_task)
    }

    // SQL: "DELETE FROM foo WHERE foo_id = 1";
    fn delete(&self, id: i32) -> Result<(), AppError> {
        let _ = self.storage.delete(id);
        // tidak perlu menghapus
        // ? self.list.remove(index);
        Ok(())
    }

    fn find_by_list(&self) -> Vec<Task> {
        self.storage.find_by_list()
    }
}

#[cfg(test)]
pub mod task_manager_tests {
    use super::*;
    use crate::{domain::task_test::setup_task, infrastructure::mock::mock::MockStorage};

    #[test]
    fn test_task_manager_of_add() {
        let repo = StorageTaskManagerRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = repo.add(setup_task(1, "test description one")).unwrap();
        assert_eq!(tasks.id, 1);
        assert_eq!(tasks.description, "test description one");
    }

    #[test]
    fn test_task_manager_of_update_description() {
        let repo = StorageTaskManagerRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = repo
            .update_description(1, &setup_task(1, "test update description one"))
            .unwrap();
        assert_eq!(tasks.id, 1);
        assert_eq!(tasks.description, "test update description one");
    }

    #[test]
    fn test_task_manager_of_delete() {
        let repo = StorageTaskManagerRepository {
            storage: Box::new(MockStorage),
        };

        let tasks = repo.delete(1).unwrap();
        assert_eq!(tasks, ());
    }
}

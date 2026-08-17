use crate::application::ports::list_repository::ListRepository;
use crate::domain::task::{Task, TaskExtensions, TaskStatus};
use crate::infrastructure::storages::storage::StorageTrait;

pub struct StorageListRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl ListRepository for StorageListRepository {
    fn all(&self) -> Vec<Task> {
        self.storage.find_by_list()
    }

    fn todo(&self) -> Vec<Task> {
        return TaskExtensions::get_status_tasks(&self.storage.find_by_list(), TaskStatus::Todo);
    }

    fn in_progress(&self) -> Vec<Task> {
        return TaskExtensions::get_status_tasks(
            &self.storage.find_by_list(),
            TaskStatus::InProgress,
        );
    }

    fn done(&self) -> Vec<Task> {
        return TaskExtensions::get_status_tasks(&self.storage.find_by_list(), TaskStatus::Done);
    }
}

#[cfg(test)]
pub mod list_tests {
    use super::*;
    use crate::infrastructure::mock::mock::MockStorage;

    #[test]
    fn test_list_of_all() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = list_repository.all();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn test_list_of_todo() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = list_repository.todo();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn test_list_of_in_progress() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = list_repository.in_progress();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::InProgress);
    }

    #[test]
    fn test_list_of_done() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorage),
        };
        let tasks = list_repository.done();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Done);
    }
}

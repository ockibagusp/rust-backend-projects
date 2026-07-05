use crate::domain::{list_repository::ListRepository, task::Task, task_status::TaskStatus};
use crate::infrastructure::storages::storage::StorageTrait;
pub struct StorageListRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl ListRepository for StorageListRepository {
    fn all(&self) -> Vec<Task> {
        self.storage.find_by_list()
    }

    fn todo(&self) -> Vec<Task> {
        return get_status_tasks(&self.storage.find_by_list(), TaskStatus::Todo);
    }

    fn in_progress(&self) -> Vec<Task> {
        return get_status_tasks(&self.storage.find_by_list(), TaskStatus::InProgress);
    }

    fn done(&self) -> Vec<Task> {
        return get_status_tasks(&self.storage.find_by_list(), TaskStatus::Done);
    }
}

// TODO
fn get_status_tasks(list: &Vec<Task>, status: TaskStatus) -> Vec<Task> {
    return list
        .iter()
        .filter(|&task| task.status == status)
        .cloned()
        .collect();
}

#[cfg(test)]
pub mod list_tests {
    use super::*;
    use crate::domain::task_test::{setup_task, setup_task_status};
    use crate::infrastructure::config;

    struct MockStorageTrait;
    impl StorageTrait for MockStorageTrait {
        fn new(_config: &config::Config) -> Self
        where
            Self: Sized,
        {
            todo!()
        }

        fn find_by_list(&self) -> Vec<Task> {
            return vec![
                setup_task(1, "test description one"),
                setup_task_status(2, "test description two", TaskStatus::InProgress),
                setup_task_status(3, "test description two", TaskStatus::Done),
            ];
        }

        fn add(&self, _add_task: &Task) -> Vec<Task> {
            todo!()
        }

        fn update(&self, _id: i32, _update_task: &mut Task) -> Vec<Task> {
            todo!()
        }

        fn delete(&self, _id: i32) -> Vec<Task> {
            todo!()
        }
    }

    #[test]
    fn test_list_of_all() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.all();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn test_list_of_todo() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.todo();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn test_list_of_in_progress() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.in_progress();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::InProgress);
    }

    #[test]
    fn test_list_of_done() {
        let list_repository = StorageListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.done();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Done);
    }
}

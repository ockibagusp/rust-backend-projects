use crate::cmd::{
    application::use_cases::list::ListRepository,
    domain::models::{Task, TaskExtensions, TaskStatus},
    infrastructure::storages::storage::StorageTrait,
};

pub struct StorageListRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl ListRepository for StorageListRepository {
    fn new(storage: Box<dyn StorageTrait>) -> Self {
        Self { storage }
    }

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

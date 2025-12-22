use mockall::automock;

use crate::task::task::Task;
use crate::task::task_manager::{TaskManager, TaskManagerTrait};

pub struct List {
    pub task_manager: TaskManager,
}

#[automock]
pub trait ListTrait {
    fn new(file_name: &'static str) -> Self;
    fn index(&self) -> Vec<Task>;
}

impl ListTrait for List {
    fn new(file_name: &'static str) -> Self {
        let _task_manager = TaskManager::new(file_name);
        List {
            task_manager: _task_manager,
        }
    }
    // Get all task list
    fn index(&self) -> Vec<Task> {
        return self.task_manager.list();
    }
}

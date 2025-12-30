use crate::file::files::File;
use crate::task::task::{Task, TaskTrait, VALID_STATUSES};
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use chrono::prelude::*;
use mockall::*;
use std::io::Error;

fn error_invalid_input(message: &str) -> Error {
    return Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("error: {}", message),
    );
}

fn error_not_found_input(message: &str) -> Error {
    Error::new(std::io::ErrorKind::NotFound, format!("error: {}", message))
}

#[derive(PartialEq, Debug)]
pub struct TaskMark {
    pub task_manager: TaskManager,
}

#[automock]
pub trait TaskMarkTrait {
    fn new(file_name: &'static str) -> Self;
    fn mark_in_progress(&self, id: i32) -> Result<Task, Error>;
    fn mark_done(&self, id: i32) -> Result<Task, Error>;
}

impl TaskMarkTrait for TaskMark {
    fn new(file_name: &'static str) -> Self {
        let task_manager = TaskManager::new(file_name);

        TaskMark {
            task_manager: task_manager,
        }
    }

    fn mark_in_progress(&self, id: i32) -> Result<Task, Error> {
        let mut task_to_update = self
            .task_manager
            .list
            .iter()
            .find(|&task| task.id == id)
            .ok_or_else(|| error_not_found_input("`id` is not found"))?
            .clone();

        if task_to_update.status == VALID_STATUSES[1] {
            return Err(error_invalid_input("Task is already in 'done' status"));
        }

        task_to_update.status = VALID_STATUSES[1].to_string();

        let _ = &self.task_manager.update(id, &mut task_to_update);
        Ok(task_to_update)
    }

    fn mark_done(&self, id: i32) -> Result<Task, Error> {
        let mut task_to_update = self
            .task_manager
            .list
            .iter()
            .find(|&task| task.id == id)
            .ok_or_else(|| error_not_found_input("`id` is not found"))?
            .clone();

        if task_to_update.status == VALID_STATUSES[2] {
            return Err(error_invalid_input("Task is already in 'done' status"));
        }
    }
}

use crate::task::task::{Task, VALID_STATUSES};
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use mockall::*;
use std::io::Error;

fn error_invalid_input(message: &str) -> Error {
    return Error::new(std::io::ErrorKind::InvalidInput, format!("{}", message));
}

#[derive(PartialEq, Debug)]
pub struct Mark {
    pub task_manager: TaskManager,
}

#[automock]
pub trait MarkTrait {
    fn new(file_name: &'static str) -> Self;
    fn find_by_id(&self, id: i32) -> Result<Task, Error>;
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error>;
    fn mark_done(&mut self, id: i32) -> Result<Task, Error>;
}

impl MarkTrait for Mark {
    fn new(file_name: &'static str) -> Self {
        Mark {
            task_manager: TaskManager::new(file_name),
        }
    }

    fn find_by_id(&self, id: i32) -> Result<Task, Error> {
        let task = self.task_manager.find_by_id(id);
        return task;
    }

    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error> {
        let mut task_to_update = self.find_by_id(id)?;

        if task_to_update.status == VALID_STATUSES[1] {
            return Err(error_invalid_input(
                "task is already in 'in-progress' status",
            ));
        }

        task_to_update.status = VALID_STATUSES[1].to_string();

        let _ = &self.task_manager.update(id, &mut task_to_update);
        Ok(task_to_update)
    }

    fn mark_done(&mut self, id: i32) -> Result<Task, Error> {
        let mut task_to_update = self.find_by_id(id)?;

        if task_to_update.status == VALID_STATUSES[2] {
            return Err(error_invalid_input("task is already in 'done' status"));
        }

        task_to_update.status = VALID_STATUSES[2].to_string();

        let _ = &self.task_manager.update(id, &mut task_to_update);
        Ok(task_to_update)
    }
}

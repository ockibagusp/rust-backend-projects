use crate::error::{error_invalid_input, error_not_found_input};
use crate::task::task::{Task, VALID_STATUSES};
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use mockall::*;
use std::io::Error;

const FILE_NAME: &str = "MARK";

#[derive(PartialEq, Debug)]
pub struct Mark {
    pub task_manager: TaskManager,
}

#[automock]
pub trait MarkTrait {
    fn new(file_name: &'static str) -> Self;
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error>;
    fn mark_done(&mut self, id: i32) -> Result<Task, Error>;
}

fn find_by_id(task_manager: &Vec<Task>, id: i32) -> Result<Task, Error> {
    let task = task_manager.iter().find(|&task| task.id == id).cloned();
    match task {
        Some(task) => Ok(task),
        None => Err(error_not_found_input::<&str>(FILE_NAME, "ID is not found")),
    }
}

impl MarkTrait for Mark {
    fn new(file_name: &'static str) -> Self {
        Mark {
            task_manager: TaskManager::new(file_name),
        }
    }

    fn mark_in_progress(&mut self, id: i32) -> Result<Task, Error> {
        let mut task_to_update = find_by_id(&self.task_manager.list, id)?;

        ///???
        if task_to_update.status == VALID_STATUSES[1] {
            return Err(error_invalid_input::<&str>(
                FILE_NAME,
                "task is already in 'in-progress' status",
            ));
        }

        task_to_update.status = VALID_STATUSES[1].to_string();

        let _ = &self.task_manager.updates(id, &mut task_to_update);
        Ok(task_to_update)
    }

    fn mark_done(&mut self, id: i32) -> Result<Task, Error> {
        let mut task_to_update = find_by_id(&self.task_manager.list, id)?;

        if task_to_update.status == VALID_STATUSES[2] {
            return Err(error_invalid_input::<&str>(
                FILE_NAME,
                "task is already in 'done' status",
            ));
        }

        task_to_update.status = VALID_STATUSES[2].to_string();

        let _ = &self.task_manager.updates(id, &mut task_to_update);
        Ok(task_to_update)
    }
}

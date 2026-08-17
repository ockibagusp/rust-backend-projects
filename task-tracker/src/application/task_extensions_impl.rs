use crate::domain::error::AppError;
use crate::domain::task::{Task, TaskExtensions, TaskI32, TaskStatus, TaskTrait};
use core::result::Result;

impl TaskExtensions {
    pub fn get_status_tasks(list: &Vec<Task>, status: TaskStatus) -> Vec<Task> {
        return list
            .iter()
            .filter(|&task| task.status == status)
            .cloned()
            .collect();
    }

    fn get_next_id(list: &Vec<Task>) -> i32 {
        let mut max_id = 0;
        for task in list {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        max_id + 1
    }

    pub fn find_by_id(
        file_name: &'static str,
        list: &Vec<Task>,
        id: i32,
    ) -> Result<Task, AppError> {
        let task = list.iter().find(|&task| task.id == id).cloned();
        match task {
            Some(task) => Ok(task),
            None => Err(AppError::NotFound(file_name, "ID is not found")),
        }
    }

    pub fn get_next_task_of_add(
        file_name: &'static str,
        list: &Vec<Task>,
        description: &str,
    ) -> Result<Task, AppError> {
        let next_id = Self::get_next_id(list);
        let add_task = Task::new(next_id, description);
        match add_task.is_validation() {
            Ok(_) => Ok(add_task),
            Err(e) => Err(AppError::InvalidInput(file_name, e)),
        }
    }

    // TODO: error handling for update description or status
    pub const DESCRIPTION: TaskI32 = 0;
    pub const STATUS: TaskI32 = 1;
    pub fn is_valid_to_task_of_description_or_status_update(
        file_name: &'static str,
        list: &Vec<Task>,
        id: i32,
        update_task: &Task,
        desc_status: TaskI32,
    ) -> bool {
        let old_task = Self::find_by_id(file_name, list, id).unwrap();
        if desc_status == Self::DESCRIPTION && old_task.description == update_task.description {
            return true;
        }
        if desc_status == Self::STATUS && old_task.status == update_task.status {
            return true;
        }
        false
    }
}

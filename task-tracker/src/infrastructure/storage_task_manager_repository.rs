use crate::domain::{
    task::{Task, TaskTrait},
    task_manager_repository::TaskManagerRepository,
    task_status::TaskStatus,
};
use crate::error::AppError;
use crate::infrastructure::storages::storage::FILE_NAME as STORAGE_FILE_NAME;
use crate::infrastructure::storages::storage::StorageTrait;
use chrono::{DateTime, Local};

pub struct StorageTaskManagerRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl TaskManagerRepository for StorageTaskManagerRepository {
    fn add(&mut self, description: &str) -> Result<Task, AppError> {
        let add_task = get_next_task_of_add(&self.storage.find_by_list(), description);
        // if let Err(e) = err {...}
        if add_task.is_err() {
            return add_task;
        }
        let add_task = add_task.unwrap();

        let _ = &self.storage.add(&add_task);
        // ? let _ = &self.list.push(add_task);

        Ok(add_task)
    }

    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, AppError> {
        let mut task = find_by_id(&self.storage.find_by_list(), id, STORAGE_FILE_NAME)?;
        // if let Err(e) = task {
        //     return Err(e);
        // }
        // let mut task_to_update = task.unwrap();
        // task_to_update.description = description.to_string();
        task.description = description.to_string();

        match self.updates(id, &mut task, DESCRIPTION) {
            Ok(updated_task) => Ok(updated_task),
            Err(e) => Err(e),
        }
    }

    fn updates(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(AppError::InvalidInput(STORAGE_FILE_NAME, e));
        }

        let is_valid = is_valid_to_task_of_description_or_status_update(
            &self.storage.find_by_list(),
            id,
            update_task,
            desc_status,
        );
        if is_valid {
            return Err(AppError::InvalidInput(
                STORAGE_FILE_NAME,
                "DESCRIPTION or STATUS is not identical",
            ));
        }
        update_task.updated_at = Local::now().into();

        let _ = self.storage.update(id, update_task);
        // ? let _ = self.find_by_id_mut(id, update_task);

        Ok(update_task.clone())
    }

    fn delete(&mut self, id: i32) -> Result<(), AppError> {
        let task = find_by_id(&self.storage.find_by_list(), id, STORAGE_FILE_NAME);
        if !task.is_ok() {
            return Err(task.unwrap_err());
        }

        let _ = self.storage.delete(id);
        // tidak perlu menghapus
        // ? self.list.remove(index);
        Ok(())
    }

    fn find_by_list(&self) -> Vec<Task> {
        self.storage.find_by_list()
    }
}

// not trait
fn get_next_id(list: &Vec<Task>) -> i32 {
    let mut max_id = 0;
    for task in list {
        if task.id > max_id {
            max_id = task.id;
        }
    }
    max_id + 1
}

pub fn find_by_id(list: &Vec<Task>, id: i32, file_name: &'static str) -> Result<Task, AppError> {
    let task = list.iter().find(|&task| task.id == id).cloned();
    match task {
        Some(task) => Ok(task),
        None => Err(AppError::NotFound(file_name, "ID is not found")),
    }
}

pub fn get_next_task_of_add(list: &Vec<Task>, description: &str) -> Result<Task, AppError> {
    let next_id = get_next_id(list);
    // Convert UTC to Jakarta time
    let now_created_at: DateTime<Local> = Local::now();

    let add_task = Task {
        id: next_id,
        description: description.to_string(),
        // status: "todo"
        status: TaskStatus::Todo,
        created_at: now_created_at.into(),
        updated_at: now_created_at.into(),
    };
    match add_task.is_validation() {
        Ok(_) => Ok(add_task),
        Err(e) => Err(AppError::InvalidInput(STORAGE_FILE_NAME, e)),
    }
}

type TaskI32 = i32;
pub const DESCRIPTION: TaskI32 = 0;
pub const STATUS: TaskI32 = 1;
pub fn is_valid_to_task_of_description_or_status_update(
    list: &Vec<Task>,
    id: i32,
    update_task: &Task,
    desc_status: TaskI32,
) -> bool {
    let old_task = find_by_id(list, id, STORAGE_FILE_NAME).unwrap();
    if desc_status == DESCRIPTION && old_task.description == update_task.description {
        return true;
    }
    if desc_status == STATUS && old_task.status == update_task.status {
        return true;
    }
    false
}

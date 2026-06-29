use crate::application::{
    list_use_cases::ListRepositoryTrait, mark_use_cases::FILE_NAME as MARK_FILE_NAME,
    mark_use_cases::MarkRepositoryTrait, task_manager_use_cases::TaskManagerRepositoryTrait,
};
use crate::domain::task::{Task, TaskStatus, TaskTrait};
use crate::error::AppError;
use crate::infrastructure::storages::storage::{FILE_NAME as STORAGE_FILE_NAME, StorageTrait};
use chrono::{DateTime, Local};

// Concrete database implementation
// ######## List Repository ########
pub struct ListRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl ListRepositoryTrait for ListRepository {
    fn all(&self) -> Vec<Task> {
        self.storage.list()
    }

    fn todo(&self) -> Vec<Task> {
        return get_status_tasks(&self.storage.list(), TaskStatus::Todo);
    }

    fn in_progress(&self) -> Vec<Task> {
        return get_status_tasks(&self.storage.list(), TaskStatus::InProgress);
    }

    fn done(&self) -> Vec<Task> {
        return get_status_tasks(&self.storage.list(), TaskStatus::Done);
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

    struct MockStorageTrait;
    impl StorageTrait for MockStorageTrait {
        fn new() -> Self
        where
            Self: Sized,
        {
            todo!()
        }

        fn list(&self) -> Vec<Task> {
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
        let list_repository = ListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.all();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn test_list_of_todo() {
        let list_repository = ListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.todo();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn test_list_of_in_progress() {
        let list_repository = ListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.in_progress();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::InProgress);
    }

    #[test]
    fn test_list_of_done() {
        let list_repository = ListRepository {
            storage: Box::new(MockStorageTrait),
        };
        let tasks = list_repository.done();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Done);
    }
}

// ######## Task Manager Repository ########

pub struct TaskManagerRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl TaskManagerRepositoryTrait for TaskManagerRepository {
    fn add(&mut self, description: &str) -> Result<Task, AppError> {
        let add_task = get_next_task_of_add(&self.storage.list(), description);
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
        let mut task = find_by_id(&self.storage.list(), id, STORAGE_FILE_NAME)?;
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
            &self.storage.list(),
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
        let task = find_by_id(&self.storage.list(), id, STORAGE_FILE_NAME);
        if !task.is_ok() {
            return Err(task.unwrap_err());
        }

        let _ = self.storage.delete(id);
        // tidak perlu menghapus
        // ? self.list.remove(index);
        Ok(())
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

// ######## Mark Repository ########
pub struct MarkRepository {
    pub storage: Box<dyn StorageTrait>,
}

impl MarkRepositoryTrait for MarkRepository {
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, AppError> {
        let mut task_to_update = find_by_id(&self.storage.list(), id, MARK_FILE_NAME)?;

        if task_to_update.status == TaskStatus::InProgress {
            return Err(AppError::InvalidInput(
                MARK_FILE_NAME,
                "task is already in 'in-progress' status",
            ));
        }

        task_to_update.status = TaskStatus::InProgress;
        let _ = self.storage.update(id, &mut task_to_update); // ? operator
        // if let Err(e) = self.task_manager.updates(id, &mut task_to_update) {
        //     return Err(e);
        // }
        // // link: https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try
        Ok(task_to_update)
    }

    fn mark_done(&mut self, id: i32) -> Result<Task, AppError> {
        let mut task_to_update = find_by_id(&self.storage.list(), id, MARK_FILE_NAME)?;

        if task_to_update.status == TaskStatus::Done {
            return Err(AppError::InvalidInput(
                MARK_FILE_NAME,
                "task is already in 'done' status",
            ));
        }

        task_to_update.status = TaskStatus::Done;

        let _ = self.storage.update(id, &mut task_to_update);
        Ok(task_to_update)
    }
}

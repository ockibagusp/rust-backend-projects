use crate::application::{list_use_cases, mark_use_cases, task_manager_use_cases};
use crate::cmd::cmd_printing::{open_task_list_for_title_str, open_task_title_str};
use crate::error::AppError;
use crate::infrastructure::{
    config, memory_storage,
    storages::storage::{FILE_NAME, Storage, StorageTrait},
};
use crate::presentation::{
    list_handler::CmdListHandler, mark_handler::CmdMarkHandler,
    task_manager_handler::CmdTaskManagerHandler,
};

/*
    List Task Operations
*/
pub fn subcmd_lists(status: &Option<String>, config: &config::Config) -> Result<String, AppError> {
    // 1. Instantiate the real infrastructure
    let repo = memory_storage::ListRepository {
        storage: Box::new(Storage::new(config)),
    };

    // 2. Inject infrastructure implementation into the usecase
    let use_case = list_use_cases::ListUseCase {
        repository: Box::new(repo),
    };

    // 3. Handle incoming API traffic payload
    let handler = CmdListHandler {
        use_case: Box::new(use_case),
    };

    // 4. Pass execution onto CMD controller
    if *status == None {
        // handle_list_of_all_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::handle_list_of_all_tasks(&handler);
        let list_str = open_task_list_for_title_str("All Lists", results);
        return Ok(list_str);
    }

    if *status == Some(String::from("todo")) {
        // handle_list_of_todo_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::handle_list_of_todo_tasks(&handler);
        let list_str = open_task_list_for_title_str("Todo Lists", results);
        return Ok(list_str);
    }

    if *status == Some(String::from("in-progress")) {
        // handle_list_of_in_progress_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::handle_list_of_in_progress_tasks(&handler);
        let list_str = open_task_list_for_title_str("In Progress Lists", results);
        return Ok(list_str);
    }

    // if *status == Some(String::from("done")) { ...}
    let results = CmdListHandler::handle_list_of_done_tasks(&handler);
    let list_str = open_task_list_for_title_str("Done Lists", results);
    return Ok(list_str);
}

/*
    Task Operations
*/
pub fn subcmd_add(description: &String, config: &config::Config) -> Result<String, AppError> {
    // 1. Instantiate the real infrastructure
    let repo = memory_storage::TaskManagerRepository {
        storage: Box::new(Storage::new(config)),
    };

    // 2. Inject infrastructure implementation into the usecase
    let use_case = task_manager_use_cases::TaskManagerUseCase {
        repository: Box::new(repo),
        storage: Box::new(Storage::new(config)),
    };

    // 3. Handle incoming API traffic payload
    let mut handler = CmdTaskManagerHandler {
        use_case: Box::new(use_case),
    };

    // 4. Pass execution onto CMD controller
    match CmdTaskManagerHandler::handle_add_tasks(&mut handler, &description) {
        Ok(task) => Ok(open_task_title_str("Add task", task)),
        Err(e) => Err(e),
    }
}

pub fn subcmd_update(
    id: &u32,
    description: &String,
    config: &config::Config,
) -> Result<String, AppError> {
    // 1. Instantiate the real infrastructure
    let repo = memory_storage::TaskManagerRepository {
        storage: Box::new(Storage::new(config)),
    };

    // 2. Inject infrastructure implementation into the usecase
    let use_case = task_manager_use_cases::TaskManagerUseCase {
        repository: Box::new(repo),
        storage: Box::new(Storage::new(config)),
    };

    // 3. Handle incoming API traffic payload
    let mut handler = CmdTaskManagerHandler {
        use_case: Box::new(use_case),
    };

    // 4. Pass execution onto CMD controller
    match CmdTaskManagerHandler::handle_update_description(&mut handler, *id as i32, &description) {
        Ok(task) => Ok(open_task_title_str("Update task", task)),
        Err(e) => Err(e),
    }
}

pub fn subcmd_delete(id: &u32, config: &config::Config) -> Result<String, AppError> {
    // 1. Instantiate the real infrastructure
    let repo = memory_storage::TaskManagerRepository {
        storage: Box::new(Storage::new(config)),
    };

    // 2. Inject infrastructure implementation into the usecase
    let use_case = task_manager_use_cases::TaskManagerUseCase {
        repository: Box::new(repo),
        storage: Box::new(Storage::new(config)),
    };

    // 3. Handle incoming API traffic payload
    let mut handler = CmdTaskManagerHandler {
        use_case: Box::new(use_case),
    };

    // 4. Pass execution onto CMD controller
    match CmdTaskManagerHandler::handle_delete(&mut handler, *id as i32) {
        Ok(_) => Ok(String::from("Delete task success")),
        Err(e) => Err(AppError::Aborted(
            FILE_NAME,
            Box::leak(format!("Error deleting task: {}", e).into_boxed_str()),
        )),
    }
}

// /*
//     Mark Task Operations
// */
pub fn subcmd_mark_in_progress(id: &u32, config: &config::Config) -> Result<String, AppError> {
    // 1. Instantiate the real infrastructure
    let repo = memory_storage::MarkRepository {
        storage: Box::new(Storage::new(config)),
    };

    // 2. Inject infrastructure implementation into the usecase
    let use_case = mark_use_cases::MarkUseCase {
        repository: Box::new(repo),
        storage: Box::new(Storage::new(config)),
    };

    // 3. Handle incoming API traffic payload
    let mut handler = CmdMarkHandler {
        use_case: Box::new(use_case),
    };

    // 4. Pass execution onto CMD controller
    match CmdMarkHandler::handle_mark_in_progress(&mut handler, *id as i32) {
        Ok(task) => Ok(open_task_title_str("Mark in progress", task)),
        Err(e) => Err(e),
    }
}

pub fn subcmd_mark_done(id: &u32, config: &config::Config) -> Result<String, AppError> {
    // 1. Instantiate the real infrastructure
    let repo = memory_storage::MarkRepository {
        storage: Box::new(Storage::new(config)),
    };

    // 2. Inject infrastructure implementation into the usecase
    let use_case = mark_use_cases::MarkUseCase {
        repository: Box::new(repo),
        storage: Box::new(Storage::new(config)),
    };

    // 3. Handle incoming API traffic payload
    let mut handler = CmdMarkHandler {
        use_case: Box::new(use_case),
    };

    // 4. Pass execution onto CMD controller
    match CmdMarkHandler::handle_mark_done(&mut handler, *id as i32) {
        Ok(task) => Ok(open_task_title_str("Mark done", task)),
        Err(e) => Err(e),
    }
}

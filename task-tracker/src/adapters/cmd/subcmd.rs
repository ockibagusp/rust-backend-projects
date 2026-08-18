use crate::adapters::{
    cmd::cmd_printing::{OpenTasks, OpenTasksTrait, open_task_title_str},
    presentation::{
        list_handler::CmdListHandler, mark_handler::CmdMarkHandler,
        task_manager_handler::CmdTaskManagerHandler,
    },
};
use crate::application::use_cases::{list, mark, task_manager};
use crate::domain::error::AppError;
use crate::infrastructure::{
    config,
    storage_list_repository::StorageListRepository,
    storage_mark_repository::StorageMarkRepository,
    storage_task_manager_repository::StorageTaskManagerRepository,
    storages::storage::{FILE_NAME, Storage, StorageTrait},
};

// &'a config not config.clone() because we want to avoid unnecessary cloning of the config object. Instead, we pass a reference to the config object to the SubCmd struct, which allows us to access the config data without creating a new copy. This is more efficient and avoids potential issues with data consistency if the config were to change during execution.
pub struct SubCmd<'a> {
    pub config: &'a config::Config,
}

// impl SubCmd {
//     pub fn new(config: config::Config) -> Self {
//         SubCmd { config.clone() }
//     }
// ....
// }
impl<'a> SubCmd<'a> {
    pub fn new(config: &'a config::Config) -> Self {
        SubCmd { config }
    }

    /*
        List Task Operations
    */
    pub fn lists(&self, status: &Option<String>) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageListRepository {
            storage: Box::new(Storage::new(&self.config)),
        };

        // 2. Inject infrastructure implementation into the usecase
        let use_case = list::ListUseCase {
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

            let list_str = OpenTasks::new(results).list();
            return Ok(list_str);
        }

        if *status == Some(String::from("todo")) {
            // handle_list_of_todo_tasks returns a Future; store and drop it to avoid unused-future warning
            let results = CmdListHandler::handle_list_of_todo_tasks(&handler);
            let list_str = OpenTasks::new(results).todo();
            return Ok(list_str);
        }

        if *status == Some(String::from("in-progress")) {
            // handle_list_of_in_progress_tasks returns a Future; store and drop it to avoid unused-future warning
            let results = CmdListHandler::handle_list_of_in_progress_tasks(&handler);
            let list_str = OpenTasks::new(results).in_progress();
            return Ok(list_str);
        }

        // if *status == Some(String::from("done")) { ...}
        let results = CmdListHandler::handle_list_of_done_tasks(&handler);
        let list_str = OpenTasks::new(results).done();
        return Ok(list_str);
    }

    /*
        Task Operations
    */
    pub fn add(&self, description: &String) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageTaskManagerRepository {
            storage: Box::new(Storage::new(&self.config)),
        };

        // 2. Inject infrastructure implementation into the usecase
        let use_case = task_manager::TaskManagerUseCase {
            repository: Box::new(repo),
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

    pub fn update(&self, id: &u32, description: &String) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageTaskManagerRepository {
            storage: Box::new(Storage::new(&self.config)),
        };

        // 2. Inject infrastructure implementation into the usecase
        let use_case = task_manager::TaskManagerUseCase {
            repository: Box::new(repo),
        };

        // 3. Handle incoming API traffic payload
        let mut handler = CmdTaskManagerHandler {
            use_case: Box::new(use_case),
        };

        // 4. Pass execution onto CMD controller
        match CmdTaskManagerHandler::handle_update_description(
            &mut handler,
            *id as i32,
            &description,
        ) {
            Ok(task) => Ok(open_task_title_str("Update task", task)),
            Err(e) => Err(e),
        }
    }

    pub fn delete(&self, id: &u32) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageTaskManagerRepository {
            storage: Box::new(Storage::new(&self.config)),
        };

        // 2. Inject infrastructure implementation into the usecase
        let use_case = task_manager::TaskManagerUseCase {
            repository: Box::new(repo),
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
    pub fn mark_in_progress(&self, id: &u32) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageMarkRepository {
            storage: Box::new(Storage::new(&self.config)),
        };

        // 2. Inject infrastructure implementation into the usecase
        let use_case = mark::MarkUseCase {
            repository: Box::new(repo),
        };

        // 3. Handle incoming API traffic payload
        let mut handler = CmdMarkHandler {
            use_case: Box::new(use_case),
        };

        // 4. Pass execution onto CMD controller
        match CmdMarkHandler::handle_mark_in_progress(&mut handler, *id as i32) {
            Ok(task) => Ok(open_task_title_str("Mark as \"in progress\"", task)),
            Err(e) => Err(e),
        }
    }

    pub fn mark_done(&self, id: &u32) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageMarkRepository {
            storage: Box::new(Storage::new(&self.config)),
        };

        // 2. Inject infrastructure implementation into the usecase
        let use_case = mark::MarkUseCase {
            repository: Box::new(repo),
        };

        // 3. Handle incoming API traffic payload
        let mut handler = CmdMarkHandler {
            use_case: Box::new(use_case),
        };

        // 4. Pass execution onto CMD controller
        match CmdMarkHandler::handle_mark_done(&mut handler, *id as i32) {
            Ok(task) => Ok(open_task_title_str("Mark as \"done\"", task)),
            Err(e) => Err(e),
        }
    }
}

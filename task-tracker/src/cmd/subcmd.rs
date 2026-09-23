use crate::cmd::{
    adapters::presentation::{
        list_handler::CmdListHandler, mark_handler::*, task_manager_handler::*,
    },
    application::{
        task_status_impl::{DONE, IN_PROGRESS, TODO},
        use_cases::{
            list::{self, ListRepository},
            mark,
            task_manager::*,
        },
    },
    cmd_printing::*,
    domain::error::AppError,
    infrastructure::{
        storage_list_repository::StorageListRepository,
        storage_mark_repository::StorageMarkRepository,
        storage_task_manager_repository::StorageTaskManagerRepository,
        storages::storage::{FILE_NAME, Storage, StorageTrait},
    },
};
use crate::config;

// &config not config.clone() because we want to avoid unnecessary cloning of the config object. Instead, we pass a reference to the config object to the SubCmd struct, which allows us to access the config data without creating a new copy. This is more efficient and avoids potential issues with data consistency if the config were to change during execution.
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
        Self { config }
    }

    /*
        List Task Operations
    */
    pub fn lists(&self, status: &Option<String>) -> Result<String, AppError> {
        let storage = Storage::new(&self.config);

        // 1. Instantiate the real infrastructure
        let repo = StorageListRepository::new(Box::new(storage));

        // 2. Inject infrastructure implementation into the usecase
        let use_case = list::ListUseCase::new(repo);

        // 3. Handle incoming API traffic payload
        let handler = CmdListHandler::new(use_case);

        // 4. Pass execution onto CMD controller

        // or,
        // if *status == None {...}
        // if *status == Some(String::from("todo")) => {...} ...

        // status.as_deref() turns Option<String> into Option<&str>
        match status.as_deref() {
            None => {
                let list_all_str = self.list_status_all(&handler);
                return Ok(list_all_str);
            }
            Some(TODO) => {
                let list_todo_str = self.list_status_todo(&handler);
                return Ok(list_todo_str);
            }
            Some(IN_PROGRESS) => {
                let list_in_progress_str = self.list_status_in_progress(&handler);
                return Ok(list_in_progress_str);
            }
            Some(DONE) => {
                let list_done_str = self.list_status_done(&handler);
                return Ok(list_done_str);
            }
            _ => {
                let status_str = status.as_deref().unwrap_or("unknown");
                let err = format!("Invalid status: '{}'", status_str);
                return Err(AppError::Aborted(FILE_NAME, err));
            }
        }
    }

    fn list_status_all(&self, handler: &CmdListHandler<StorageListRepository>) -> String {
        // handle_list_of_all_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::all(&handler);
        let mut open_tasks_list = OpenTasks::new();
        open_tasks_list.set_tasks(results);
        let list_str = open_tasks_list.list();
        return list_str;
    }

    fn list_status_todo(&self, handler: &CmdListHandler<StorageListRepository>) -> String {
        // handle_list_of_todo_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::todo(&handler);
        let mut open_tasks_list = OpenTasks::new();
        open_tasks_list.set_tasks(results);
        let list_str = open_tasks_list.todo();
        return list_str;
    }

    fn list_status_in_progress(&self, handler: &CmdListHandler<StorageListRepository>) -> String {
        // handle_list_of_in_progress_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::in_progress(&handler);
        let mut open_tasks_list = OpenTasks::new();
        open_tasks_list.set_tasks(results);
        let list_str = open_tasks_list.in_progress();
        return list_str;
    }

    fn list_status_done(&self, handler: &CmdListHandler<StorageListRepository>) -> String {
        // handle_list_of_done_tasks returns a Future; store and drop it to avoid unused-future warning
        let results = CmdListHandler::done(&handler);
        let mut open_tasks_list = OpenTasks::new();
        open_tasks_list.set_tasks(results);
        let list_str = open_tasks_list.done();
        return list_str;
    }

    /*
        Task Operations
    */
    pub fn add(&self, description: &String) -> Result<String, AppError> {
        let storage = Storage::new(&self.config);

        // 1. Instantiate the real infrastructure
        let repo = StorageTaskManagerRepository::new(Box::new(storage));

        // 2. Inject infrastructure implementation into the usecase
        let use_case = CreateTaskManagerUseCase::new(repo);

        // 3. Handle incoming API traffic payload
        let mut handler = CmdCreateTaskManagerHandler::new(use_case);

        let request = CreateTaskManagerDto {
            description: description.to_string(),
        };

        // 4. Pass execution onto CMD controller
        match handler.execute(request) {
            Ok(task) => Ok(OpenTask::add(&task)),
            Err(e) => Err(e),
        }
    }

    pub fn update(&self, id: &u32, description: &String) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageTaskManagerRepository::new(Box::new(Storage::new(&self.config)));

        // 2. Inject infrastructure implementation into the usecase
        let use_case = UpdateTaskManagerUseCase::new(repo);

        // 3. Handle incoming API traffic payload
        let mut handler = CmdUpdateTaskManagerHandler::new(use_case);

        // 4. Pass execution onto CMD controller
        let request = UpdateTaskManagerDto {
            id: *id as i32,
            description: description.to_string(),
        };
        match handler.execute_update_desc(request) {
            Ok(task) => Ok(OpenTask::update(&task)),
            Err(e) => Err(e),
        }
    }

    pub fn delete(&self, id: &u32) -> Result<String, AppError> {
        // 1. Instantiate the real infrastructure
        let repo = StorageTaskManagerRepository::new(Box::new(Storage::new(&self.config)));

        // 2. Inject infrastructure implementation into the usecase
        let use_case = DeleteTaskManagerUseCase::new(repo);

        // 3. Handle incoming API traffic payload
        let mut handler = CmdDeleteTaskManagerHandler::new(use_case);

        // 4. Pass execution onto CMD controller
        let request = DeleteTaskManagerDto { id: *id as i32 };
        match handler.execute(request) {
            Ok(_) => Ok(OpenTask::delete()),
            Err(e) => {
                let err = format!("Error deleting task: {}", e);
                Err(AppError::Aborted(FILE_NAME, err))
            }
        }
    }

    /*
        Mark Task Operations
    */
    pub fn mark_in_progress(&self, id: &u32) -> Result<String, AppError> {
        let storage = Storage::new(&self.config);

        // 1. Instantiate the real infrastructure
        let repo = StorageMarkRepository::new(Box::new(storage));

        // 2. Inject infrastructure implementation into the usecase
        let use_case = mark::MarkInProgressUseCase::new(Box::new(repo));

        // 3. Handle incoming API traffic payload
        let handler = CmdMarkInProgressHandler::new(Box::new(use_case));

        // 4. Pass execution onto CMD controller
        let request = mark::MarkDto { id: *id as i32 };
        match handler.execute(request) {
            Ok(task) => Ok(OpenMarkTask::in_progress(&task)),
            Err(e) => Err(e),
        }
    }

    pub fn mark_done(&self, id: &u32) -> Result<String, AppError> {
        let storage = Storage::new(&self.config);

        // 1. Instantiate the real infrastructure
        let repo = StorageMarkRepository::new(Box::new(storage));

        // 2. Inject infrastructure implementation into the usecase
        let use_case = mark::MarkDoneUseCase::new(Box::new(repo));

        // 3. Handle incoming API traffic payload
        let handler = CmdMarkDoneHandler::new(Box::new(use_case));

        // 4. Pass execution onto CMD controller
        let request = mark::MarkDto { id: *id as i32 };
        match handler.execute(request) {
            Ok(task) => Ok(OpenMarkTask::done(&task)),
            Err(e) => Err(e),
        }
    }
}

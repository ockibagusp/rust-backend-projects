use clap::{Parser, Subcommand};
use mockall::*;

use crate::cmd::cmd_printing::{open_task_list_for_title_str, open_task_title_str};
use crate::infrastructure::{
    memory_storage,
    storages::storage::{Storage, StorageTrait},
};
use core::result::Result;
use std::io::Error;

use crate::application::{list_use_cases, mark_use_cases, task_manager_use_cases};
use crate::error::error_kind_aborted;
use crate::presentation::{
    list_handler::CmdListHandler, mark_handler::CmdMarkHandler,
    task_manager_handler::CmdTaskManagerHandler,
};

const FILE_NAME: &str = "COMMAND";

#[derive(Parser, Debug, PartialEq)]
#[command(name = "task-cli", about = "A simple task management CLI application")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    List { status: Option<String> },
    Add { description: String },
    Update { id: u32, description: String },
    Delete { id: u32 },
    MarkInProgress { id: u32 },
    MarkDone { id: u32 },
}

// TDD
// ✅ ❔ ❌
// 3.1. buatlah struktur data Command
// => 3.1. create the Command Data Structure
// ------------------------------------------------
pub struct Command {}

// TDD
// ✅ ❔ ❌
// 3.2.
// => the CommandTrait trait for the Command struct
// ------------------------------------------------
#[automock]
pub trait CommandTrait {
    fn new() -> Self;
    fn run(&mut self) -> Result<String, Error>;
}

impl CommandTrait for Command {
    fn new() -> Self {
        Command {}
    }

    fn run(&mut self) -> Result<String, Error> {
        let cli = Cli::parse();
        match &cli.commands {
            /*
                Task Operations
            */
            // -------------------------------
            // $ task-cli add <description>
            // $ task-cli update <id> <description>
            // $ task-cli delete <id>
            Commands::Add { description } => {
                // 1. Instantiate the real infrastructure
                let repo = memory_storage::TaskManagerRepository {
                    storage: Box::new(Storage::new()),
                };

                // 2. Inject infrastructure implementation into the usecase
                let use_case = task_manager_use_cases::TaskManagerUseCase {
                    repository: Box::new(repo),
                    storage: Box::new(Storage::new()),
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
            Commands::Update { id, description } => {
                // 1. Instantiate the real infrastructure
                let repo = memory_storage::TaskManagerRepository {
                    storage: Box::new(Storage::new()),
                };

                // 2. Inject infrastructure implementation into the usecase
                let use_case = task_manager_use_cases::TaskManagerUseCase {
                    repository: Box::new(repo),
                    storage: Box::new(Storage::new()),
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
            Commands::Delete { id } => {
                // 1. Instantiate the real infrastructure
                let repo = memory_storage::TaskManagerRepository {
                    storage: Box::new(Storage::new()),
                };

                // 2. Inject infrastructure implementation into the usecase
                let use_case = task_manager_use_cases::TaskManagerUseCase {
                    repository: Box::new(repo),
                    storage: Box::new(Storage::new()),
                };

                // 3. Handle incoming API traffic payload
                let mut handler = CmdTaskManagerHandler {
                    use_case: Box::new(use_case),
                };

                // 4. Pass execution onto CMD controller
                match CmdTaskManagerHandler::handle_delete(&mut handler, *id as i32) {
                    Ok(_) => Ok(String::from("Delete task success")),
                    Err(e) => Err(error_kind_aborted::<&str>(
                        FILE_NAME,
                        &format!("Error deleting task: {}", e),
                    )),
                }
            }
            // /*
            //     Mark Task Operations
            // */
            // -------------------------------
            // $ task-cli [mark-in-progress|mark-done] <id>
            Commands::MarkInProgress { id } => {
                // 1. Instantiate the real infrastructure
                let repo = memory_storage::MarkRepository {
                    storage: Box::new(Storage::new()),
                };

                // 2. Inject infrastructure implementation into the usecase
                let use_case = mark_use_cases::MarkUseCase {
                    repository: Box::new(repo),
                    storage: Box::new(Storage::new()),
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
            Commands::MarkDone { id } => {
                // 1. Instantiate the real infrastructure
                let repo = memory_storage::MarkRepository {
                    storage: Box::new(Storage::new()),
                };

                // 2. Inject infrastructure implementation into the usecase
                let use_case = mark_use_cases::MarkUseCase {
                    repository: Box::new(repo),
                    storage: Box::new(Storage::new()),
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
            /*
                List Task Operations
            */
            // -------------------------------
            // $ task-cli list
            // $ task-cli list [todo|in-progress|done]
            Commands::List { status } => {
                // 1. Instantiate the real infrastructure
                let repo = memory_storage::ListRepository {
                    storage: Box::new(Storage::new()),
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
                    let lists = CmdListHandler::handle_list_of_all_tasks(&handler);
                    let list_str = open_task_list_for_title_str("All Lists", lists);
                    return Ok(list_str);
                }

                if *status == Some(String::from("todo")) {
                    // handle_list_of_todo_tasks returns a Future; store and drop it to avoid unused-future warning
                    let lists = CmdListHandler::handle_list_of_todo_tasks(&handler);
                    let list_str = open_task_list_for_title_str("Todo Lists", lists);
                    return Ok(list_str);
                }

                if *status == Some(String::from("in-progress")) {
                    // handle_list_of_in_progress_tasks returns a Future; store and drop it to avoid unused-future warning
                    let lists = CmdListHandler::handle_list_of_in_progress_tasks(&handler);
                    let list_str = open_task_list_for_title_str("In Progress Lists", lists);
                    return Ok(list_str);
                }

                // if *status == Some(String::from("done")) { ...}
                let lists = CmdListHandler::handle_list_of_done_tasks(&handler);
                let list_str = open_task_list_for_title_str("Done Lists", lists);
                return Ok(list_str);
            }
        }
    }
}

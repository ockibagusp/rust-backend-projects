use clap::{Parser, Subcommand};
use mockall::*;

use crate::cmd::cmd_printing::{open_task_list_for_title_str, open_task_title_str};
use crate::error::error_kind_aborted;
use crate::list::list::{ListManager, ListManagerTrait};
use crate::mark::mark::{Mark, MarkTrait};
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use core::result::Result;
use std::io::Error;

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
// 1. buat sebuah field `task manager` bertipe objek `TaskManager`
// => create the `task manager` field with an object type of `TaskManager`
// 2. buat sebuah field `mark` dengan tipe objek `Mark`
// => create the `mark` field with the `Mark` object type
// 3. buat sebuah field `list manager` dengan tipe objek `ListManager`
// => create the `list manager` field with the `List` object type
pub struct Command {
    file_name: &'static str,
}

// TDD
// ✅ ❔ ❌
// 3.2.
// => the CommandTrait trait for the Command struct
// ------------------------------------------------
#[automock]
pub trait CommandTrait {
    fn new(file_name: &'static str) -> Self;
    fn run(&mut self) -> Result<String, Error>;
}

impl CommandTrait for Command {
    fn new(file_name: &'static str) -> Self {
        Command { file_name }
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
                let mut task_manager = TaskManager::new(self.file_name);
                match task_manager.add(&description) {
                    Ok(task) => Ok(open_task_title_str("Add task", task)),
                    Err(e) => Err(e),
                }
            }
            Commands::Update { id, description } => {
                let mut task_manager = TaskManager::new(self.file_name);
                match task_manager.update_description(*id as i32, &description) {
                    Ok(task) => Ok(open_task_title_str("Update task", task)),
                    Err(e) => Err(e),
                }
            }
            Commands::Delete { id } => {
                let mut task_manager = TaskManager::new(self.file_name);
                match task_manager.delete(*id as i32) {
                    Ok(_) => Ok(String::from("Delete task success")),
                    Err(e) => Err(error_kind_aborted::<&str>(
                        FILE_NAME,
                        &format!("Error deleting task: {}", e),
                    )),
                }
            }
            /*
                Mark Task Operations
            */
            // -------------------------------
            // $ task-cli [mark-in-progress|mark-done] <id>
            Commands::MarkInProgress { id } => {
                let mut mark = Mark::new(self.file_name);
                match mark.mark_in_progress(*id as i32) {
                    Ok(task) => Ok(open_task_title_str("Mark in progress", task)),
                    Err(e) => Err(e),
                }
            }
            Commands::MarkDone { id } => {
                let mut mark = Mark::new(self.file_name);
                match mark.mark_done(*id as i32) {
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
                let list = ListManager::new(self.file_name);

                if *status == None {
                    let list_str = open_task_list_for_title_str("All Lists", list.index());
                    return Ok(list_str);
                }

                if *status == Some(String::from("todo")) {
                    let list_str = open_task_list_for_title_str("Todo Lists", list.todo());
                    return Ok(list_str);
                }

                if *status == Some(String::from("in-progress")) {
                    let list_str =
                        open_task_list_for_title_str("In Progress Lists", list.in_progress());
                    return Ok(list_str);
                }

                // if *status == Some(String::from("done")) { ...}
                let list_str = open_task_list_for_title_str("Done Lists", list.done());
                return Ok(list_str);
            }
        }
    }
}

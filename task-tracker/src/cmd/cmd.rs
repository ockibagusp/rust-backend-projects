use mockall::*;

use crate::help;
use crate::list::list::{ListManager, ListManagerTrait};
use crate::mark::mark::{Mark, MarkTrait};
use crate::task::task::Task;
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use core::result::Result;
use std::io::{Error, ErrorKind};
// => add(...); v

pub struct Command {
    task_manager: TaskManager,
    mark: Mark,
    list_manager: ListManager,
}

#[automock]
pub trait CommandTrait {
    fn new(file_name: &'static str) -> Self;
    fn to_task_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
    fn to_mark_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
    fn to_list_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
    fn run(&mut self, args: &Vec<String>) -> Result<String, Error>;
}

impl CommandTrait for Command {
    fn new(file_name: &'static str) -> Self {
        Command {
            task_manager: TaskManager::new(file_name),
            mark: Mark::new(file_name),
            list_manager: ListManager::new(file_name),
        }
    }

    /*
        Task Manager Operations
    */
    fn to_task_cli(&mut self, args: &Vec<String>) -> Result<String, Error> {
        if "add" == args.get(1).unwrap() && (args.len() == 2 || args.len() > 3) {
            return Err(error_kind_refused(help::help_add()));
        }
        if "update" == args.get(1).unwrap() && (args.len() == 2 || args.len() > 4) {
            return Err(error_kind_refused(help::help_update()));
        }
        if "delete" == args.get(1).unwrap() && (args.len() == 2 || args.len() > 3) {
            return Err(error_kind_refused(help::help_delete()));
        }

        let accept_args = args.get(1).unwrap();
        let input_args = args.get(2).unwrap();

        // $ task-cli add <task>
        if accept_args == "add" {
            let task_result = self.task_manager.add(input_args);
            if let Err(e) = &task_result {
                return Err(error_kind_aborted(format!(
                    "Error adding task: {}",
                    e.to_string()
                )));
            }
            let task = task_result.unwrap();
            return Ok(open_task_title_str("Add task", task));
        }

        // $ task-cli update <id> <task>
        if accept_args == "update" {
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(e) => {
                    return Err(error_kind_refused(format!("Error updating task: {}", e)));
                }
            };

            let update_args = args.get(3);
            if update_args.is_none() {
                return Err(error_kind_refused(help::help_update()));
            }
            let task_result = self
                .task_manager
                .update_description(id, update_args.unwrap());
            let Ok(task) = task_result else {
                return Err(error_kind_aborted(format!(
                    "Error updating task: {}",
                    task_result.unwrap_err()
                )));
            };

            return Ok(open_task_title_str("Update task", task));
        }

        // $ task-cli delete <id>
        if accept_args == "delete" {
            let parsed_id = input_args.parse::<i32>();
            if parsed_id.is_err() {
                return Err(error_kind_refused(
                    "Error deleting task: ID must be a number".to_string(),
                ));
            }
            let id = parsed_id.unwrap();

            let delete_task = self.task_manager.delete(id);
            if let Err(e) = delete_task {
                return Err(error_kind_aborted(format!("Error deleting task: {}", e)));
            }

            return Ok(String::from("Delete task success"));
        }

        return Err(error_kind_refused(help::help_all()));
    }

    /*
        Mark Task Operations
    */
    fn to_mark_cli(&mut self, args: &Vec<String>) -> Result<String, Error> {
        if "mark-in-progress" == args.get(1).unwrap() && (args.len() == 2 || args.len() > 3) {
            return Err(error_kind_refused(help::help_mark_in_progress()));
        }
        if "mark-done" == args.get(1).unwrap() && (args.len() == 2 || args.len() > 3) {
            return Err(error_kind_refused(help::help_mark_done()));
        }

        let accept_args = args.get(1).unwrap();
        let input_args = args.get(2).unwrap();

        // $ task-cli mark-in-progress <id>
        if accept_args == "mark-in-progress" {
            let parsed_id = input_args.parse::<i32>();
            if parsed_id.is_err() {
                return Err(error_kind_refused(
                    "Error marking task in progress: ID must be a number".to_string(),
                ));
            }
            let id = parsed_id.unwrap();

            let task_result = self.mark.mark_in_progress(id);
            if let Err(e) = &task_result {
                return Err(error_kind_aborted(format!(
                    "Error marking task in progress: {}",
                    e.to_string()
                )));
            }
            let task = task_result.unwrap();
            return Ok(open_task_title_str("Mark task in progress", task));
        } else if accept_args == "mark-done" {
            let parsed_id = input_args.parse::<i32>();
            if parsed_id.is_err() {
                return Err(error_kind_refused(
                    "Error marking task in progress: ID must be a number".to_string(),
                ));
            }
            let id = parsed_id.unwrap();

            let task_result = self.mark.mark_done(id);
            if let Err(e) = &task_result {
                return Err(error_kind_aborted(format!(
                    "Error marking task done: {}",
                    e.to_string()
                )));
            }
            let task = task_result.unwrap();
            return Ok(open_task_title_str("Mark task done", task));
        }

        Err(error_kind_refused(help::help_all()))
    }

    fn to_list_cli(&mut self, args: &Vec<String>) -> Result<String, Error> {
        // func => tasks::list(); v
        //         ^^^^^
        //      => add(...); v
        // func => list(); x
        if args.len() == 2 {
            let list = self.list_manager.index();
            let list_str = open_task_list_for_title_str("Lists", list);
            return Ok(list_str);
        }

        // $ task-cli list done
        let input_args = &args[2];
        if input_args == "todo" {
            let todo = self.list_manager.todo();
            let list_str = open_task_list_for_title_str("Todos Task", todo);
            return Ok(list_str);
        } else if input_args == "in-progress" {
            let in_progress = self.list_manager.in_progress();
            let list_str = open_task_list_for_title_str("In-Progresses Task", in_progress);
            return Ok(list_str);
        } else if input_args == "done" {
            let done = self.list_manager.done();
            let done_str = open_task_list_for_title_str("Dones Task", done);
            return Ok(done_str);
        }

        return Err(error_kind_refused(help::help_list()));
    }

    fn run(&mut self, args: &Vec<String>) -> Result<String, Error> {
        if args.len() == 1 {
            return Err(error_kind_refused(help::help_all()));
        }

        let accept_args = args.get(1).unwrap();

        /*
            Task Operations
        */
        // -------------------------------
        // $ task-cli add <description>
        // $ task-cli update <id> <description>
        // $ task-cli delete <id>
        if accept_args == "add" || accept_args == "update" || accept_args == "delete" {
            match self.to_task_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(error_kind_refused(e.to_string()));
                }
            }
        }
        /*
            Mark Task Operations
        */
        // -------------------------------
        // $ task-cli [mark-in-progress|mark-done] <id>
        else if accept_args == "mark-in-progress" || accept_args == "mark-done" {
            match self.to_mark_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(error_kind_refused(e.to_string()));
                }
            }
        }
        /*
            List Task Operations
        */
        // -------------------------------
        // $ task-cli list
        // $ task-cli list [todo|in-progress|done]
        else if accept_args == "list" {
            match self.to_list_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(error_kind_refused(e.to_string()));
                }
            }
        }

        return Ok(help::help_all());
    }
}

fn error_kind_refused(err_message: String) -> Error {
    return Error::new(ErrorKind::ConnectionRefused, err_message);
}

fn error_kind_aborted(err_message: String) -> Error {
    return Error::new(ErrorKind::ConnectionAborted, err_message);
}

fn open_task_str(task: &Task) -> String {
    let task_str: Vec<String> = vec![
        format!("ID: {}", task.id),
        format!("----- Description: {}", task.description),
        format!("----- Status: {}", task.status),
        format!("----- Created At: {:?}", task.created_at),
        format!("----- Updated At: {:?}", task.updated_at),
    ];

    return task_str.join("\n");
}

fn open_task_title_str(title: &str, task: Task) -> String {
    let task_str: Vec<String> = vec![
        String::from(title),
        String::from("------------------"),
        open_task_str(&task),
    ];

    return task_str.join("\n");
}

fn open_task_list_for_title_str(title: &str, list: Vec<Task>) -> String {
    let mut list_str: Vec<String> = vec![String::from(title), String::from("------------------")];
    if list.is_empty() {
        list_str.push(String::from("No lists found."));
    } else {
        for task in list {
            list_str.push(open_task_str(&task));
        }
        list_str.push(String::from("++++++++++++++++++"));
    }

    return list_str.join("\n");
}

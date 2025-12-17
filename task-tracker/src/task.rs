use crate::files::{self, File};
use chrono::prelude::FixedOffset;
use chrono::{DateTime, Local};
use core::result::Result;
use serde::{Deserialize, Serialize};
use std::io::Error;

use mockall::predicate::*;
use mockall::*;

pub(crate) static _FILE_NAME: &'static str = "task-cli.json";

fn error_invalid_input(message: &str) -> Error {
    Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("error : {}", message),
    )
}

/*
    Task
*/

// #[derive(PartialEq, Clone, Debug, SerializeDisplay, DeserializeFromStr)]
// pub enum TaskStatus {
//     Todo,
//     InProgress,
//     Done,
// }

// impl fmt::Display for TaskStatus {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TaskStatus::Todo => write!(f, "todo"),
//             TaskStatus::InProgress => write!(f, "in-progress"),
//             TaskStatus::Done => write!(f, "done"),
//         }
//     }
// }
pub const VALID_STATUSES: [&str; 3] = ["todo", "in-progress", "done"];

// #[derive(PartialEq, Clone, Debug, SerializeDisplay, DeserializeDisplay)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    // status: todo, in-progress, done
    pub status: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[automock]
pub trait TaskTrait {
    fn is_validation(&self) -> Result<(), Error>;
}

impl TaskTrait for Task {
    fn is_validation(&self) -> Result<(), Error> {
        let invalid_input = std::io::ErrorKind::InvalidInput;
        if self.id.is_negative() {
            return Err(Error::new(
                invalid_input,
                "Error: [TaskTrait] `id` is negative",
            ));
        }
        if self.description.trim().is_empty() || self.description.len() > 26 {
            return Err(Error::new(
                invalid_input,
                "Error: [TaskTrait] `description` is empty or too long",
            ));
        }
        let _valid_statuses = VALID_STATUSES;
        if !matches!(&self.status, _valid_statuses) {
            return Err(Error::new(
                invalid_input,
                format!("Error: [TaskTrait] `status` is invalid `{}`", &self.status),
            ));
        }
        Ok(())
    }
}

/*
    TaskManager
*/
#[derive(PartialEq, Debug)]
pub struct TaskManager {
    pub file: File,
    pub list: Vec<Task>,
}

#[automock]
pub trait TaskManagerTrait {
    fn new(file_name: &'static str) -> Self;
    fn get_next_id(&self) -> i32;
    fn list(&self) -> Vec<Task>;
    fn add(&mut self, input: &str) -> Result<Task, Error>;
    fn update(&self, id: i32, update_task: &mut Task) -> Result<Task, Error>;
    fn delete(&self, id: i32) -> Result<(), Error>;
}

impl TaskManagerTrait for TaskManager {
    fn new(file_name: &'static str) -> Self {
        let file = files::File::new(file_name);
        let _list = file.list();

        Self { file, list: _list }
    }

    fn get_next_id(&self) -> i32 {
        let mut max_id = 0;
        for task in &self.list {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        max_id + 1
    }

    fn list(&self) -> Vec<Task> {
        let tasks_list = self.file.list();
        tasks_list
    }

    fn add(&mut self, input: &str) -> Result<Task, Error> {
        let next_id = self.get_next_id();
        // Convert UTC to Jakarta time
        let now_created_at: DateTime<Local> = Local::now();

        let add_task = Task {
            id: next_id,
            description: String::from(input),
            // status: "todo"
            status: VALID_STATUSES[0].to_string(),
            created_at: now_created_at.into(),
            updated_at: now_created_at.into(),
        };

        let err = add_task.is_validation();
        // if let Err(e) = err {...}
        if err.is_err() {
            return Err(err.unwrap_err());
        }

        let _ = &self.list.push(add_task.clone());
        let _ = &self.file.add(add_task.clone());

        Ok(add_task)
    }

    fn update(&self, id: i32, update_task: &mut Task) -> Result<Task, Error> {
        let err = update_task.is_validation();
        if let Err(e) = err {
            return Err(e);
        }

        let old_task = self.list.iter().find(|&task| task.id == id).unwrap();
        if old_task.id != update_task.id {
            return Err(error_invalid_input("`id` is not identical"));
        }
        if old_task.description == update_task.description {
            return Err(error_invalid_input("`description` is identical"));
        }
        update_task.updated_at = Local::now().into();

        let _ = &self.file.update(id, update_task);

        Ok(update_task.clone())
    }

    fn delete(&self, id: i32) -> Result<(), Error> {
        let task = self.list.iter().find(|&task| task.id == id);
        if task.is_none() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Error: [TaskManagerTrait] `id` is not found",
            ));
        }

        let index = self.list().iter().position(|t| t.id == id).unwrap();
        self.list().remove(index);
        Ok(())
    }
}

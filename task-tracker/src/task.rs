use crate::files::{self, File};
use chrono::DateTime;
use chrono::prelude::FixedOffset;
use core::result::Result;
use serde::{Deserialize, Serialize};
// use serde_with::{DeserializeFromStr, SerializeDisplay};
// use std::fmt;
use std::io::Error;

use mockall::predicate::*;
use mockall::*;

pub(crate) static _FILE_NAME: &'static str = "task-cli.json";

/*
    Task
*/

// #[derive(PartialEq, Clone, Debug, SerializeDisplay, DeserializeFromStr)]
// #[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
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

pub const TODO: &str = "todo";
pub const IN_PROGRESS: &str = "in-progress";
pub const DONE: &str = "done";

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
    fn is_validation(&self) -> bool;
}

impl TaskTrait for Task {
    fn is_validation(&self) -> bool {
        if self.id.is_negative() {
            return false;
        }
        if self.description.trim().is_empty() || self.description.len() > 26 {
            return false;
        }
        let _valid_statuses = vec![TODO, IN_PROGRESS, DONE];
        if matches!(&self.status, _valid_statuses) {
            return true;
        }
        false
    }
}

/*
    TaskManager
*/
#[derive(PartialEq, Debug)]
pub struct TaskManager {
    pub file: File,
    pub list: Vec<Task>,
    pub next_id: i32,
}

#[automock]
pub trait TaskManagerTrait {
    fn new(file_name: &'static str) -> Self;
    fn get_next_id(&self) -> i32;
    fn list(&self) -> Result<Vec<Task>, Error>;
    fn add(&mut self, input: &str) -> Result<Task, Error>;
}

impl TaskManagerTrait for TaskManager {
    fn new(file_name: &'static str) -> Self {
        let file = files::File::new(file_name);
        TaskManager {
            file: file,
            list: vec![],
            next_id: 0,
        }
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

    fn list(&self) -> Result<Vec<Task>, Error> {
        let tasks_list = self.file.list();
        Ok(tasks_list)
    }

    fn add(&mut self, input: &str) -> Result<Task, Error> {
        let next_id = self.get_next_id();

        let created_at =
            DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .into();

        let add_task = Task {
            id: next_id,
            description: String::from(input),
            status: TODO.to_string(),
            created_at: created_at,
            updated_at: created_at,
        };

        if !add_task.is_validation() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid task data",
            ));
        }

        let _ = &self.list.push(add_task.clone());

        Ok(add_task)
    }

    // pub fn update(&self, up_task: &Task) -> Result<Task, &str> {
    //     let err = self.validate(&up_task);
    //     if let Err(e) = err {
    //         return Err(e);
    //     }

    //     let update_task = Task {
    //         id: up_task.id,
    //         description: up_task.description.to_string(),
    //         status: up_task.status.clone(),
    //         created_at: up_task.created_at,
    //         updated_at: DateTime::parse_from_str(
    //             "2025-04-12 12:10:10.000000 +07:00",
    //             "%Y-%m-%d %H:%M:%S%.6f %z",
    //         )
    //         .unwrap()
    //         .into(),
    //     };
    //     Ok(update_task)
    // }

    // pub fn delete(id: i32) -> bool {
    //     if id != 1 {
    //         return false;
    //     }
    //     true
    // }
}

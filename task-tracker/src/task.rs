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
            return Err(Error::new(invalid_input, "error: `id` is negative"));
        }
        if self.description.trim().is_empty() || self.description.len() > 26 {
            return Err(Error::new(
                invalid_input,
                "error: `description` is empty or too long",
            ));
        }
        let _valid_statuses = VALID_STATUSES;
        if !matches!(&self.status, _valid_statuses) {
            return Err(Error::new(
                invalid_input,
                format!("error: `status` is invalid `{}`", &self.status),
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
    pub is_file: bool,
    pub list: Vec<Task>,
    pub next_id: i32,
}

#[automock]
pub trait TaskManagerTrait {
    fn set_static_file(&mut self, file_name: &'static str) -> ();
    fn valid_file(&self) -> ();
    fn get_next_id(&self) -> i32;
    fn list(&self) -> Vec<Task>;
    fn add(&mut self, input: &str) -> Result<Task, Error>;
}

impl TaskManagerTrait for TaskManager {
    fn set_static_file(&mut self, file_name: &'static str) -> () {
        let file = files::File::new(file_name);
        self.file = file;
        self.is_file = true;
    }

    fn valid_file(&self) -> () {
        if self.is_file {
            panic!("Error: File is not set")
        }
    }

    fn get_next_id(&self) -> i32 {
        self.valid_file();

        let mut max_id = 0;
        for task in &self.list {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        max_id + 1
    }

    fn list(&self) -> Vec<Task> {
        self.valid_file();

        let tasks_list = self.file.list();
        tasks_list
    }

    fn add(&mut self, input: &str) -> Result<Task, Error> {
        self.valid_file();

        let next_id = self.get_next_id();

        let created_at =
            DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .into();

        let add_task = Task {
            id: next_id,
            description: String::from(input),
            // status: "todo"
            status: VALID_STATUSES[0].to_string(),
            created_at: created_at,
            updated_at: created_at,
        };

        let is_valid = add_task.is_validation();
        if is_valid.is_err() {
            return Err(is_valid.unwrap_err());
        }

        let _ = &self.list.push(add_task.clone());

        Ok(add_task)
    }

    // pub fn update(&self, up_task: &Task) -> Result<Task, &str> {
    //     self.valid_file();
    //
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
    //     self.valid_file();
    //
    //     if id != 1 {
    //         return false;
    //     }
    //     true
    // }
}

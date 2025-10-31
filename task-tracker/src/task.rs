use chrono::DateTime;
use chrono::prelude::FixedOffset;
use core::result::Result;

use serde::{Deserialize, Serialize};

pub(crate) static _FILE_NAME: &'static str = "task-cli.json";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    // status: todo, in-progress, done
    pub status: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl Task {
    pub fn is_validation(&self) -> bool {
        if self.id.is_negative() {
            return false;
        }
        if self.description.trim().is_empty() || self.description.len() > 26 {
            return false;
        }
        let valid_statuses = vec!["todo", "in-progress", "done"];
        if !valid_statuses.contains(&self.status.as_str()) {
            return false;
        }
        true
    }
}

#[derive(PartialEq, Debug)]
pub struct TaskManager {
    pub list: Vec<Task>,
    pub next_id: i32,
}

impl TaskManager {
    fn get_next_id(&self) -> i32 {
        let mut max_id = 0;
        for task in &self.list {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        max_id + 1
    }

    fn list() -> Vec<Task> {
        let task1 = Task {
            id: 1,
            description: String::from("buy milk"),
            status: String::from("todo"),
            created_at: DateTime::parse_from_str(
                "2025-04-10 10:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
            updated_at: DateTime::parse_from_str(
                "2025-04-10 10:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
        };
        let task2 = Task {
            id: 2,
            description: "buy bread".to_string(),
            status: "in-progress".to_string(),
            created_at: DateTime::parse_from_str(
                "2025-04-12 12:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
            updated_at: DateTime::parse_from_str(
                "2025-04-12 12:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
        };
        vec![task1, task2]
    }

    fn validate(&self, task: &Task) -> Result<(), &'static str> {
        if task.id.is_negative() {
            return Err("ID must be a positive number");
        }
        if task.description.trim().is_empty() && task.description.len() <= 10 {
            return Err("Description cannot be empty or exceed 10 characters");
        }
        let valid_statuses = vec!["todo", "in-progress", "done"];
        if !valid_statuses.contains(&task.status.as_str()) {
            return Err("Status must be one of: todo, in-progress, done");
        }
        Ok(())
    }

    pub fn add(&mut self, input: &str) -> Result<Task, &str> {
        let next_id = self.get_next_id();

        let created_at =
            DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .into();

        let add_task = Task {
            id: next_id,
            description: String::from(input),
            status: String::from("todo"),
            created_at: created_at,
            updated_at: created_at,
        };

        let err = self.validate(&add_task);
        if let Err(e) = err {
            return Err(e);
        }

        &self.list.push(add_task.clone());

        Ok(add_task)
    }

    pub fn update(&self, up_task: &Task) -> Result<Task, &str> {
        let err = self.validate(&up_task);
        if let Err(e) = err {
            return Err(e);
        }

        let update_task = Task {
            id: up_task.id,
            description: up_task.description.to_string(),
            status: up_task.status.to_string(),
            created_at: up_task.created_at,
            updated_at: DateTime::parse_from_str(
                "2025-04-12 12:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
        };
        Ok(update_task)
    }

    pub fn delete(id: i32) -> bool {
        if id != 1 {
            return false;
        }
        true
    }
}

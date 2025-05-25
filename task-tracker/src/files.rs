use std::io::Write;
use tasks::_FILE_NAME;

use chrono::prelude::FixedOffset;
use chrono::{DateTime, Utc};

use crate::tasks;

// https://medium.com/@aleksej.gudkov/rust-write-to-file-example-a-practical-guide-51c24695aa80

#[allow(dead_code)]
pub struct File {
    pub file_name: String,
}

#[allow(dead_code)]
pub trait FileTrait {
    fn new(name: String) -> Self;
    fn name(&self) -> String;
    fn create(&self, description: &str) -> tasks::Task;
    fn now_utc(&self) -> DateTime<FixedOffset>;
    fn task(
        &self,
        id: i32,
        description: &str,
        status: &str,
        created_at: DateTime<FixedOffset>,
        updated_at: DateTime<FixedOffset>,
    ) -> tasks::Task;
    fn task_list<'a>(&self, next_count: i32, tasks: Vec<&'a tasks::Task>) -> tasks::TaskList<'a>;
}

impl FileTrait for File {
    fn new(name: String) -> Self {
        File { file_name: name }
    }
    fn name(&self) -> String {
        // format!("{}", _FILE_NAME)
        self.file_name.clone()
    }

    fn create(&self, description: &str) -> tasks::Task {
        let now_utc = Self::now_utc(self);
        let add_task = Self::task(self, 1, description, "todo", now_utc, now_utc);
        let add_tasklist = Self::task_list(self, 2, vec![&add_task]);

        let json_data =
            serde_json::to_string_pretty(&add_tasklist).expect("Unable to serialize task list");

        let mut file = std::fs::File::create(_FILE_NAME).expect("Failed to create file");
        if let Err(error) = file.write_all(json_data.as_bytes()) {
            eprintln!("Unable to write to file: {}", error);
            std::process::exit(1);
        }

        add_task
    }

    fn now_utc(&self) -> DateTime<FixedOffset> {
        let now_utc = Utc::now();
        return now_utc.into();
    }
    fn task(
        &self,
        id: i32,
        description: &str,
        status: &str,
        created_at: DateTime<FixedOffset>,
        updated_at: DateTime<FixedOffset>,
    ) -> tasks::Task {
        tasks::Task {
            id,
            description: description.to_string(),
            status: String::from(status),
            created_at: created_at,
            updated_at: updated_at,
        }
    }
    fn task_list<'a>(&self, next_count: i32, tasks: Vec<&'a tasks::Task>) -> tasks::TaskList<'a> {
        tasks::TaskList {
            next_count: next_count,
            tasks: tasks,
        }
    }

    add_task
}

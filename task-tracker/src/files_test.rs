use crate::files::File as MyFile;
use crate::tasks;

use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::prelude::FixedOffset;

#[test]
fn test_create_file() {
    test_exists_file();

    let test_file_trait = MyFile::new(test_file_name());
    let task = test_file_trait.create("Test Task");
    assert_eq!(task.description, "Test Task");
    assert_eq!(task.status, "todo");

    test_remove_file();
}

fn test_exists_file() {
    if std::path::Path::new(&test_file_name()).exists() {
        test_remove_file();
    }
}

fn test_remove_file() {
    std::fs::remove_file(&test_file_name()).expect("Failed to remove existing file");
}

fn test_file_name() -> String {
    format!("test-{}", tasks::_FILE_NAME)
}

// testing purposes
pub trait TestMyFileTrait {
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

impl TestMyFileTrait for MyFile {
    fn new(name: String) -> Self {
        MyFile { file_name: name }
    }
    fn name(&self) -> String {
        self.file_name.clone()
    }
    fn create(&self, description: &str) -> tasks::Task {
        let now_utc = Self::now_utc(self);
        let add_task = Self::task(self, 1, description, "todo", now_utc, now_utc);
        let add_tasklist = Self::task_list(self, 2, vec![&add_task]);

        let json_data =
            serde_json::to_string_pretty(&add_tasklist).expect("Unable to serialize task list");

        let mut file = File::create(Self::name(&self)).expect("Failed to create file");
        if let Err(error) = file.write_all(json_data.as_bytes()) {
            eprintln!("Unable to write to file: {}", error);
            std::process::exit(1);
        }

        add_task
    }

    fn now_utc(&self) -> DateTime<FixedOffset> {
        chrono::DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
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
}

use std::fs;
use std::fs::File as std_file;
use std::io::Write;

use crate::task::{self, Task};

// https://medium.com/@aleksej.gudkov/rust-write-to-file-example-a-practical-guide-51c24695aa80

#[allow(dead_code)]
pub struct File<'a> {
    file_name: &'a str,
}

#[allow(dead_code)]
impl<'a> File<'a> {
    pub fn new(name: &'a str) -> Self {
        // Try to open the file; if it doesn't exist, try to create it.
        match std_file::create(name) {
            Ok(mut f) => {
                let nil_json_string =
                    serde_json::to_string_pretty(&Vec::<task::Task>::new()).unwrap();
                f.write_all(nil_json_string.as_bytes())
                    .expect("Failed to write initial JSON");
                f
            }
            Err(e) => panic!("Failed to open or create file '{}': {}", name, e),
        };

        File { file_name: name }
    }

    fn name(&self) -> &str {
        &self.file_name
    }

    fn list(&self) -> Vec<Task> {
        let tasks: Vec<Task> = vec![];
        tasks
    }

    fn update(&self, update_task: Vec<Task>) -> bool {
        let json_string = serde_json::to_string_pretty(&update_task).unwrap();
        if fs::write(&self.name(), json_string).is_err() {
            return false;
        }

        return true;
    }
}

mod tests {
    use super::{File, Task, fs};

    fn test_file_name() -> &'static str {
        return "test-task-cli.json";
    }

    fn test_exists_file() {
        if std::path::Path::new(&test_file_name()).exists() {
            test_remove_file();
        }
    }

    fn test_remove_file() {
        std::fs::remove_file(&test_file_name()).expect("Failed to remove existing file");
    }

    #[test]
    fn test_file_new() {
        test_exists_file();

        let test_file = File::new(&test_file_name());
        assert_eq!(test_file.name(), test_file.file_name);

        let contents =
            fs::read_to_string(&test_file_name()).expect("Should have been able to read the file");
        assert_eq!(contents, "[]".to_string());

        test_remove_file();
    }

    #[test]
    fn test_file_update() {
        let test_file = File::new(&test_file_name());
        use chrono::{DateTime, FixedOffset};

        let created_at = DateTime::parse_from_str(
            "2025-10-13 14:07:06.072493 +07:00",
            "%Y-%m-%d %H:%M:%S%.f %z",
        )
        .expect("Failed to parse created_at");

        let updated_at = DateTime::parse_from_str(
            "2025-10-13 19:07:06.072493 +07:00",
            "%Y-%m-%d %H:%M:%S%.f %z",
        )
        .expect("Failed to parse updated_at");

        let update_task = Task {
            id: 2,
            description: "Buy cook dinner".to_string(),
            status: "in-progress".to_string(),
            created_at,
            updated_at,
        };
        let updated = test_file.update(vec![update_task]);
        assert_eq!(updated, true);

        let contents =
            fs::read_to_string(&test_file_name()).expect("Should have been able to read the file");
        assert_eq!(
            contents,
            String::from(
                "[\n  {\n    \"id\": 2,\n    \"description\": \"Buy cook dinner\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]"
            )
        );

        test_remove_file();
    }
}

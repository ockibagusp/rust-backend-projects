#[cfg(test)]
use crate::tasks;
#[cfg(test)]
use crate::tasks::_FILE_NAME;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Write;

fn test_file_name() -> String {
    format!("test-{}", _FILE_NAME)
}

use lazy_static::lazy_static;
lazy_static! {
    static ref _TIME_CREATE: chrono::DateTime<chrono::FixedOffset> = {
        chrono::DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
    };
}

#[cfg(test)]
fn create(description: &str) -> tasks::Task {
    let add_task = tasks::Task {
        id: 1,
        description: description.to_string(),
        status: String::from("todo"),
        created_at: *_TIME_CREATE,
        updated_at: *_TIME_CREATE,
    };

    let add_tasklist = tasks::TaskList {
        next_count: 2,
        tasks: vec![&add_task],
    };

    let json_data =
        serde_json::to_string_pretty(&add_tasklist).expect("Unable to serialize task list");

    let mut file = File::create(test_file_name()).expect("Failed to create file");
    if let Err(error) = file.write_all(json_data.as_bytes()) {
        eprintln!("Unable to write to file: {}", error);
        std::process::exit(1);
    }

    add_task
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_create() {
        if std::path::Path::new(&test_file_name()).exists() {
            std::fs::remove_file(test_file_name()).expect("Failed to remove existing file");
        }

        let task = create("Test Task");
        assert_eq!(task.description, "Test Task");
        assert_eq!(task.status, "todo");

        std::fs::remove_file(test_file_name()).expect("Failed to open file");
    }
}

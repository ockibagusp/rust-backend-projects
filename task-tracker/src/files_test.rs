#[cfg(test)]
use crate::tasks;
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::io::Write;

use once_cell::sync::Lazy;

static _FILE_NAME: &str = "task-cli.json";
static _TIME_CREATE: Lazy<chrono::DateTime<chrono::FixedOffset>> = Lazy::new(|| {
    chrono::DateTime::parse_from_str(
        "2025-04-10 10:10:10.000000 +07:00",
        "%Y-%m-%d %H:%M:%S%.6f %z",
    )
    .unwrap()
});

#[cfg(test)]
fn create(description: &str) -> tasks::Task {
    let add_task = tasks::Task {
        id: 2,
        description: description.to_string(),
        status: String::from("todo"),
        created_at: *_TIME_CREATE,
        updated_at: *_TIME_CREATE,
    };

    let json_data = match serde_json::to_string(&add_task) {
        Ok(data) => data,
        Err(erorr) => {
            eprintln!("Unable to load data: {}", erorr);
            std::process::exit(1);
        }
    };

    let attr = fs::metadata(_FILE_NAME);
    if attr.is_err() {
        let mut file = match fs::File::create(_FILE_NAME) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to create file: {}", error);
                std::process::exit(1);
            }
        };

        if let Err(error) = writeln!(file, "{}", json_data) {
            eprintln!("Unable to write to file: {}", error);
        }
    } else {
        let mut file = match fs::OpenOptions::new().append(true).open(_FILE_NAME) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to open file: {}", error);
                std::process::exit(1);
            }
        };

        if let Err(error) = writeln!(file, "{}", json_data) {
            eprintln!("Unable to write to file: {}", error);
        }
    }

    add_task
}

#[cfg(test)]
fn write_to_file(description: &str) -> String {
    let add_task = tasks::Task {
        id: 2,
        description: description.to_string(),
        status: String::from("todo"),
        created_at: *_TIME_CREATE,
        updated_at: *_TIME_CREATE,
    };

    let json_data = match serde_json::to_string(&add_task) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Unable to load data: {}", error);
            std::process::exit(1);
        }
    };

    json_data
}

mod tests {
    #[cfg(test)]
    use std::fs::File;

    #[cfg(test)]
    use super::*;

    #[test]
    fn test_create() {
        let task = create("Test task");
        assert_eq!(task.description, "Test task");
        assert_eq!(task.status, "todo");
    }

    #[test]
    fn test_write_to_file() {
        let task = write_to_file("Test task");
        assert_eq!(task, "Test task");

        let mut file: Result<File, std::io::Error> = File::open(_FILE_NAME);
        match file {
            Ok(_) => println!("File exists"),
            Err(err) => println!("File does not exist"),
        }
        // if file.is_err() {
        //     file = File::create(FILE_NAME);
        // }

        // let reader = std::io::BufReader::new(file);
        // let mut lines = reader.lines();

        // assert!(lines.next().is_some(), "File is empty");
        // assert_eq!(
        //     lines.next().unwrap().unwrap(),
        //     r#"{"id":2,"description":"Test task","status":"todo","created_at":"2025-04-10T10:10:10.000000+07:00","updated_at":"2025-04-10T10:10:10.000000+07:00"}"#,
        //     "File content does not match"
        // );
    }
}
